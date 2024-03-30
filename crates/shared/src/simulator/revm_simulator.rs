use std::str::FromStr;
use std::sync::{Arc, RwLock};

use anyhow::{anyhow, Result};
use ethers::abi::{AbiDecode, AbiEncode, AbiError};
use ethers::types::{BlockId, Log};
use ethers::{
    abi,
    providers::Middleware,
    types::{Address, Block, BlockNumber, H256, U256},
    utils::keccak256,
};
use hashbrown::HashMap;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use revm::primitives::alloy_primitives;
use revm::{
    db::{EmptyDB, EthersDB},
    primitives::HaltReason,
    primitives::{
        AccountInfo, Bytecode, CfgEnv, ExecutionResult, Output, ResultAndState, TransactTo, TxEnv, KECCAK_EMPTY,
    },
    ContextWithHandlerCfg, DatabaseRef, Evm,
};

use contracts::erc20_token::{BalanceOfCall, BalanceOfReturn};
use contracts::simulator::{
    SimulateGetAmountsOutUniswapV2Call, SimulateGetAmountsOutUniswapV2Return, SimulatorAbiErrors,
    SIMULATORABI_DEPLOYED_BYTECODE,
};
use vidger::logger::{error, info, warn};
use vidger::types::NewBlock;
use vidger::utilities::block_on;

use crate::amm::{AmmPoolKind, AmmProtocolKind};
use crate::managers::AmmManager;
use crate::simulator::ThreadSafeInMemoryDB;
use crate::types::CryptoToken;

#[derive(Debug, Clone)]
pub struct TxSuccessResult {
    pub output: alloy_primitives::Bytes,
    pub logs: Option<Vec<alloy_primitives::Log>>,
    pub gas_used: u64,
    pub gas_refunded: u64,
}

#[derive(Debug, Clone)]
pub struct TxRevertResult {
    pub output: SimulatorAbiErrors,
    pub gas_used: u64,
}

#[derive(Debug, Clone)]
pub struct TxHaltResult {
    pub reason: HaltReason,
    pub gas_used: u64,
}

pub enum TxResult {
    Success(TxSuccessResult),
    Revert(TxRevertResult),
    Halt(TxHaltResult),
}

pub struct RevmSimulator<M> {
    provider: Arc<M>,
    simulator_address: alloy_primitives::Address,
    account: alloy_primitives::Address,
    accounts_slots_to_update: HashMap<alloy_primitives::Address, Vec<alloy_primitives::U256>>,
    ctx_with_handler: ContextWithHandlerCfg<(), ThreadSafeInMemoryDB>,
}

impl<M: Middleware + 'static> RevmSimulator<M> {
    fn deploy_amm(
        mut revm_ctx: ContextWithHandlerCfg<(), ThreadSafeInMemoryDB>,
        ethers_db: &Arc<RwLock<EthersDB<M>>>,
        amm: &Arc<AmmProtocolKind>,
    ) {
        let mut account_info_list: Vec<(alloy_primitives::Address, AccountInfo)> = vec![];

        // TODO: For uniswap_v3 you need quoter and router
        match &**amm {
            AmmProtocolKind::UniswapV2(uniswap2) => {
                let router_address: alloy_primitives::Address = uniswap2.router().0.into();
                let factory_address: alloy_primitives::Address = uniswap2.factory().0.into();

                account_info_list.extend([
                    (
                        router_address,
                        ethers_db.read().unwrap().basic_ref(router_address).unwrap().unwrap(),
                    ),
                    (
                        factory_address,
                        ethers_db.read().unwrap().basic_ref(factory_address).unwrap().unwrap(),
                    ),
                ]);
            }
        };

        // Commit
        let db: &mut ThreadSafeInMemoryDB = &mut revm_ctx.context.evm.db;
        for (acc, info) in account_info_list {
            db.insert_account_info(acc, info);
        }
    }

    fn deploy_token_and_spoof_balance(
        mut revm_ctx: ContextWithHandlerCfg<(), ThreadSafeInMemoryDB>,
        token: &CryptoToken,
        accounts_list_to_spoof: &[alloy_primitives::Address],
    ) {
        let hundred_grand_eth: alloy_primitives::U256 = alloy_primitives::U256::from(100_000)
            .checked_mul(alloy_primitives::U256::from(10).pow(alloy_primitives::U256::from(18)))
            .unwrap();

        // Deploy token
        let token_address: alloy_primitives::Address = token.proxy_or_address().0.into();
        let code = Bytecode::new_raw(alloy_primitives::Bytes::from(token.code().clone()));
        let token_acc_info = AccountInfo::new(hundred_grand_eth, 0, code.hash_slow(), code);

        // Spoof balance
        // https://ethereum.stackexchange.com/questions/147205/how-to-view-the-amount-of-storage-a-contract-uses
        // https://ethereum.stackexchange.com/questions/47986/using-getstorageat-on-mappingaddress-uint64
        let input_balance_slots: Option<Vec<alloy_primitives::U256>> = if token.balance_contract_slot() != -1 {
            // Inject simulator contract with token balance
            // Inject account with token balance
            let slots_to_spoof: Vec<alloy_primitives::U256> = accounts_list_to_spoof
                .iter()
                .map(|address_to_spoof| {
                    alloy_primitives::U256::from_be_bytes(keccak256(abi::encode(&[
                        abi::Token::Address(ethers::types::Address::from(address_to_spoof.0 .0)),
                        abi::Token::Uint(ethers::types::U256::from(token.balance_contract_slot())),
                    ])))
                })
                .collect();
            Some(slots_to_spoof)
        } else {
            None
        };

        // Commit
        let db: &mut ThreadSafeInMemoryDB = &mut revm_ctx.context.evm.db;
        db.insert_account_info(token_address, token_acc_info);

        if let Some(input_balance_slots) = input_balance_slots {
            for input_balance_slot_index in input_balance_slots {
                db
                    .insert_account_storage(token_address, input_balance_slot_index, hundred_grand_eth)
                    .expect("failed to insert token balance in DB");
            }
        }
    }

    fn deploy_pool(
        mut revm_ctx: ContextWithHandlerCfg<(), ThreadSafeInMemoryDB>,
        ethers_db: &Arc<RwLock<EthersDB<M>>>,
        pool: &AmmPoolKind,
        accounts_list_to_spoof: &[alloy_primitives::Address],
        accounts_slots_to_update: &mut HashMap<alloy_primitives::Address, Vec<alloy_primitives::U256>>,
    ) {
        /*
        we can notice that it retrieves the balance of token0, token1 making calls to the token contracts.
        This means that our newly deployed pair contract has to have real token balances to perform a real swap.
        */
        // Add pool to spoof
        // Don't make `accounts_list_to_spoof` as mutable vector, as it will keep push all pools to spoof
        let mut accounts_list_to_spoof: Vec<alloy_primitives::Address> = accounts_list_to_spoof.to_vec();
        accounts_list_to_spoof.push(pool.address().0.into());

        // Deploy tokens
        Self::deploy_token_and_spoof_balance(revm_ctx.clone(), pool.token0(), &accounts_list_to_spoof);
        Self::deploy_token_and_spoof_balance(revm_ctx.clone(), pool.token1(), &accounts_list_to_spoof);

        // Deploy pool
        let pool_address: alloy_primitives::Address = pool.address().0.into();
        let pool_acc_info: AccountInfo = ethers_db.read().unwrap().basic_ref(pool_address).unwrap().unwrap();

        // Prepare pool
        let mut slots: HashMap<alloy_primitives::U256, alloy_primitives::U256> = HashMap::new();
        match pool {
            AmmPoolKind::UniswapV2(_) => {
                const RESERVE_SLOT: u32 = 8;

                /*
                Why set unlocked slot?
                  Because it is only initialized when the pool is initialized with Creation Bytecode
                  and the default value is 0
                */

                // uniswapV2 pool has 13 slots (0 -> 12)
                let slots_idx = (0..=12).map(alloy_primitives::U256::from);
                for slot_idx in slots_idx {
                    if let Ok(slot_value) = ethers_db.read().unwrap().storage_ref(pool_address, slot_idx) {
                        slots.insert(slot_idx, slot_value);
                    } else {
                        warn!("Failed to get slot {} from pool {}", slot_idx, pool_address);
                    }
                }

                // Only reserve slot needs to be updated
                accounts_slots_to_update.insert(pool_address, vec![alloy_primitives::U256::from(RESERVE_SLOT)]);
            }
        };

        // Commit
        let db: &mut ThreadSafeInMemoryDB = &mut revm_ctx.context.evm.db;
        db.insert_account_info(pool_address, pool_acc_info);

        for (slot, value) in slots {
            db
                .insert_account_storage(pool_address, slot, value)
                .expect("failed to insert pool reserves in DB");
        }
    }

    fn deploy_full_amm(
        revm_ctx: ContextWithHandlerCfg<(), ThreadSafeInMemoryDB>,
        ethers_db: &Arc<RwLock<EthersDB<M>>>,
        amm: &Arc<AmmProtocolKind>,
        accounts_list_to_spoof: &[alloy_primitives::Address],
        accounts_slots_to_update: &mut HashMap<alloy_primitives::Address, Vec<alloy_primitives::U256>>,
    ) {
        Self::deploy_amm(revm_ctx.clone(), ethers_db, amm);

        // Deploy pools
        // TODO: Parallelize
        for pool in amm.pools() {
            Self::deploy_pool(revm_ctx.clone(), ethers_db, pool, accounts_list_to_spoof, accounts_slots_to_update);
        }
    }

    #[inline]
    fn get_tx_result(result: ExecutionResult) -> TxResult {
        let output: TxResult = match result {
            ExecutionResult::Success {
                gas_used,
                gas_refunded,
                output,
                logs,
                ..
            } => match output {
                Output::Call(o) => TxResult::Success(TxSuccessResult {
                    output: o,
                    logs: Some(logs),
                    gas_used,
                    gas_refunded,
                }),
                Output::Create(o, _) => TxResult::Success(TxSuccessResult {
                    output: o,
                    logs: Some(logs),
                    gas_used,
                    gas_refunded,
                }),
            },
            ExecutionResult::Revert { output, gas_used } => {
                if output.is_empty() {
                    error!("Empty revert output, Mostly solidity abi encoding/decoding error");
                }

                println!("DECODE: {}", output);
                let error: Result<SimulatorAbiErrors, AbiError> = SimulatorAbiErrors::decode(output);
                if error.is_err() {
                    panic!("Failed to decode revert output: {:?}", error.unwrap_err());
                }

                TxResult::Revert(TxRevertResult {
                    // TODO: That's not necessary
                    output: error.unwrap(),
                    gas_used,
                })
            }
            ExecutionResult::Halt { reason, gas_used } => TxResult::Halt(TxHaltResult { reason, gas_used }),
        };

        output
    }

    pub(super) fn new(provider: Arc<M>, amm_manager: &AmmManager) -> Result<Self> {
        // https://github.com/bluealloy/revm/issues/1062
        let hundred_grand_eth: alloy_primitives::U256 = alloy_primitives::U256::from(100_000)
            .checked_mul(alloy_primitives::U256::from(10).pow(alloy_primitives::U256::from(18)))
            .unwrap();
        let account = alloy_primitives::Address::from_str("0x9cf277A22EB4c551c6E18F7a6C0ee1893bcB034f").unwrap();
        let mut accounts_slots_to_update: HashMap<alloy_primitives::Address, Vec<alloy_primitives::U256>> =
            HashMap::new();

        // Prepare in-memory DB
        let mut db = ThreadSafeInMemoryDB::new(EmptyDB::new());
        let ethers_db: Arc<RwLock<EthersDB<M>>> = Arc::new(RwLock::new(
            EthersDB::new(provider.clone(), Some(BlockId::Number(BlockNumber::Latest))).unwrap(),
        ));

        // Give the user enough ETH to pay for gas
        let user_acc_info = AccountInfo::new(hundred_grand_eth, 0, KECCAK_EMPTY, Bytecode::default());
        db.insert_account_info(account, user_acc_info);

        // Deploy Simulator contract
        let simulator_address =
            alloy_primitives::Address::from_str("0xF2d01Ee818509a9540d8324a5bA52329af27D19E").unwrap();
        let simulator_bytes = Bytecode::new_raw((*SIMULATORABI_DEPLOYED_BYTECODE.0).into());
        let simulator_acc_info = AccountInfo::new(hundred_grand_eth, 0, simulator_bytes.hash_slow(), simulator_bytes);
        db.insert_account_info(simulator_address, simulator_acc_info);

        // Create EVM
        //let db: ThreadSafeInMemoryDB = Arc::try_unwrap(db).unwrap().into_inner().unwrap();
        let mut evm: Evm<'static, (), ThreadSafeInMemoryDB> = Evm::builder().with_db(db).build();

        // overriding some default env values to make it more efficient for testing
        let evm_cfg: &mut CfgEnv = evm.cfg_mut();
        evm_cfg.limit_contract_code_size = None;
        evm_cfg.disable_block_gas_limit = true;
        evm_cfg.disable_base_fee = true;

        // Create context
        let ctx_with_handler: ContextWithHandlerCfg<(), ThreadSafeInMemoryDB> = evm.into_context_with_handler_cfg();

        // Deploy amm
        let address_to_spoof: Vec<alloy_primitives::Address> = vec![simulator_address, account];
        //let db: Arc<RwLock<ThreadSafeInMemoryDB>> = Arc::new(RwLock::new(db));
        for amm in amm_manager.amms() {
            Self::deploy_full_amm(
                ctx_with_handler.clone(),
                &ethers_db,
                amm,
                &address_to_spoof,
                &mut accounts_slots_to_update,
            );
        }

        Ok(Self {
            provider,
            simulator_address,
            account,
            accounts_slots_to_update,
            ctx_with_handler,
        })
    }
}

impl<M> RevmSimulator<M> {
    #[inline]
    pub fn provider(&self) -> &Arc<M> {
        &self.provider
    }
}

impl<M> RevmSimulator<M>
where
    M: Middleware + 'static,
{
    #[inline]
    fn get_evm(&self) -> Evm<(), ThreadSafeInMemoryDB> {
        Evm::builder()
            .with_context_with_handler_cfg(self.ctx_with_handler.clone())
            .build()
    }

    #[inline]
    fn clone_evm(
        &self,
        context_with_handler_cfg: ContextWithHandlerCfg<(), ThreadSafeInMemoryDB>,
    ) -> Evm<(), ThreadSafeInMemoryDB> {
        Evm::builder()
            .with_context_with_handler_cfg(context_with_handler_cfg)
            .build()
    }

    #[inline]
    fn get_storage_at(
        &self,
        address: alloy_primitives::Address,
        slot: alloy_primitives::U256,
    ) -> alloy_primitives::U256 {
        self.get_evm().db().storage_ref(address, slot).unwrap()
    }

    pub fn sync_by_block(&mut self, new_block: &NewBlock, logs: &[Log]) {
        // Get touched addresses that need to be updated
        let touched_addresses: HashMap<alloy_primitives::Address, &Vec<alloy_primitives::U256>> = logs
            .par_iter()
            .filter_map(|log: &Log| {
                let address: alloy_primitives::Address = log.address.0.into();
                let slots_to_update: &Vec<alloy_primitives::U256> = self.accounts_slots_to_update.get(&address)?;
                if slots_to_update.is_empty() {
                    return None;
                }

                Some((address, slots_to_update))
            })
            .collect();

        if touched_addresses.is_empty() {
            info!(
                "Simulator 'on_new_block' no touched addresses found for block {}",
                new_block.number
            );
            return;
        }

        // Create ethers db
        let Some(e_db) = EthersDB::new(Arc::clone(&self.provider), Some(new_block.number.into())) else {
            error!(
                "Simulator 'on_new_block' failed to create ethers db for block {}",
                new_block.number
            );
            return;
        };

        // Get slots values from ethers db
        // TODO: Parallelize
        let mut slots_values: HashMap<alloy_primitives::Address, (alloy_primitives::U256, alloy_primitives::U256)> =
            HashMap::new();
        for (address, slots_to_update) in touched_addresses {
            for slot in slots_to_update {
                let Ok(slot_value) = e_db.storage_ref(address, *slot) else {
                    error!(
                        "Simulator 'on_new_block' failed to get storage value for address {:?} and slot {:?}",
                        address, slot
                    );
                    continue;
                };

                slots_values.insert(address, (*slot, slot_value));
            }
        }

        // Update EVM db
        let db: &mut ThreadSafeInMemoryDB = &mut self.ctx_with_handler.context.evm.db;
        for (address, (slot, value)) in slots_values {
            db.insert_account_storage(address, slot, value)
                .expect("failed to slot storage value");
        }

        info!("Simulator updated storage values for block '{}'", new_block.number);
    }

    pub fn get_tokens_balance_slot(&self, tokens: &[Address]) -> Result<HashMap<Address, Result<Option<i32>>>> {
        let mut evm: Evm<(), ThreadSafeInMemoryDB> = self.get_evm();

        // Get token account info from ethers middleware and insert it into EVM
        let cur_block: Block<H256> =
            block_on(self.provider.get_block(BlockNumber::Latest))?.ok_or(anyhow!("failed to retrieve block"))?;
        let ethers_db: EthersDB<M> =
            EthersDB::new(Arc::clone(&self.provider), Some(cur_block.number.unwrap().into())).unwrap();

        let tokens_accounts: Vec<(AccountInfo, alloy_primitives::Address)> = tokens
            .par_iter()
            .map(|token: &Address| {
                let token: alloy_primitives::Address = token.0.into();
                let token_acc_info: AccountInfo = ethers_db.basic_ref(token).unwrap().unwrap();

                (token_acc_info, token)
            })
            .collect();

        for (token_acc_info, token) in tokens_accounts {
            evm.db_mut().insert_account_info(token, token_acc_info);
        }

        // Call balanceOf
        let calldata: alloy_primitives::Bytes = AbiEncode::encode(BalanceOfCall {
            who: self.account.0 .0.into(),
        })
        .into();

        let handler_cfg: &ContextWithHandlerCfg<(), ThreadSafeInMemoryDB> = &evm.into_context_with_handler_cfg();
        let ret: HashMap<Address, Result<Option<i32>>> = tokens
            .par_iter()
            .map(|token: &Address| {
                let mut evm: Evm<(), ThreadSafeInMemoryDB> = self.clone_evm(handler_cfg.clone());

                let _token: alloy_primitives::Address = token.0.into();

                let tx: &mut TxEnv = evm.tx_mut();
                tx.caller = self.account.0.into();
                tx.transact_to = TransactTo::Call(_token);
                tx.data = calldata.clone();

                let result_and_state: ResultAndState = match evm.transact() {
                    Ok(result) => result,
                    Err(e) => {
                        return (*token, Err(anyhow!("EVM call failed: {e:?}")));
                    }
                };

                // Get touched storage
                let token_acc: &revm::primitives::Account = result_and_state.state.get(&_token).unwrap();
                let touched_storage: &revm::primitives::Storage = &token_acc.storage;
                println!("Touched storage slots: {:?}", touched_storage);

                // Some tokens have a lot of storage slots like
                // https://polygonscan.com/token/0x9C9e5fD8bbc25984B178FdCE6117Defa39d2db39
                // balance slot are 51
                for i in 0..200 {
                    let slot: [u8; 32] = keccak256(&abi::encode(&[
                        abi::Token::Address(self.account.0 .0.into()),
                        abi::Token::Uint(U256::from(i)),
                    ]));

                    let slot = alloy_primitives::U256::from_be_bytes(slot);
                    if touched_storage.get(&slot).is_none() {
                        continue;
                    };

                    println!("Balance storage slot: {:?} ({:?})", i, slot);
                    return (*token, Ok(Some(i)));
                }

                (*token, Ok(None))
            })
            .collect();

        Ok(ret)
    }

    pub fn get_token_balance(&self, token: Address) -> Result<U256> {
        let calldata: Vec<u8> = AbiEncode::encode(BalanceOfCall {
            who: self.account.0 .0.into(),
        });

        let mut evm: Evm<(), ThreadSafeInMemoryDB> = self.get_evm();
        let tx: &mut TxEnv = evm.tx_mut();
        tx.caller = self.account.0.into();
        tx.transact_to = TransactTo::Call(token.0.into());
        tx.data = calldata.into();

        let result_and_state: ResultAndState = match evm.transact() {
            Ok(result) => result,
            Err(e) => return Err(anyhow!("EVM call failed: {e:?}")),
        };

        let tx_result: TxResult = Self::get_tx_result(result_and_state.result);
        match tx_result {
            TxResult::Success(result) => {
                let Ok(decoded_output) = BalanceOfReturn::decode(&result.output) else {
                    return Err(anyhow!("Failed to decode output"));
                };

                Ok(decoded_output.0)
            }
            TxResult::Revert(_) => Err(anyhow!("Failed to get token balance")),
            TxResult::Halt(_) => Err(anyhow!("Failed to get token balance")),
        }
    }

    pub fn get_amounts_out(&self, pool: &AmmPoolKind, input: &CryptoToken, amount_in: U256) -> Result<U256> {
        // TODO: For uniswap_v3 `contract_address` are the quarter
        let calldata: Vec<u8> = match pool {
            AmmPoolKind::UniswapV2(_) => {
                //let path: ethers::types::Bytes = abi::encode(&[abi::Token::Array(vec![
                //    abi::Token::Address(*pool.token0().address()),
                //    abi::Token::Address(*pool.token1().address()),
                //])])
                //.into();

                let path: Vec<Address> = if pool.token0().address() == input.address() {
                    vec![*pool.token0().address(), *pool.token1().address()]
                } else {
                    vec![*pool.token1().address(), *pool.token0().address()]
                };

                AbiEncode::encode(SimulateGetAmountsOutUniswapV2Call {
                    router: pool.dex().router().0.into(),
                    path,
                    amount_in,
                })
            }
        };

        let mut evm: Evm<(), ThreadSafeInMemoryDB> = self.get_evm();
        let tx: &mut TxEnv = evm.tx_mut();
        tx.caller = self.account.0.into();
        tx.transact_to = TransactTo::Call(self.simulator_address);
        tx.data = calldata.into();

        let result_and_state: ResultAndState = match evm.transact() {
            Ok(result) => result,
            Err(e) => return Err(anyhow!("EVM call failed: {e:?}")),
        };

        let tx_result: TxResult = Self::get_tx_result(result_and_state.result);
        match tx_result {
            TxResult::Success(result) => match pool {
                AmmPoolKind::UniswapV2(_) => {
                    let Ok(decoded_output) = SimulateGetAmountsOutUniswapV2Return::decode(&result.output) else {
                        return Err(anyhow!("Failed to decode output"));
                    };

                    Ok(decoded_output.0)
                }
            },
            TxResult::Revert(revert) => Err(anyhow!("Failed to get token balance REVERT: {:?}", revert.output)),
            TxResult::Halt(halt) => Err(anyhow!("Failed to get token balance HALT: {:?}", halt.reason)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::amm::{UniswapV2Pool, UniswapV2Protocol};
    use ethers::providers::{Http, Provider};
    use std::sync::OnceLock;

    fn get_provider<'a>() -> &'a Arc<Provider<Http>> {
        static PROVIDER: OnceLock<Arc<Provider<Http>>> = OnceLock::new();
        PROVIDER.get_or_init(|| {
            Arc::new(
                Provider::<Http>::try_from("https://polygon-mainnet.infura.io/v3/c230ccbf294b44bcac907f4a719d06c4")
                    .unwrap(),
            )
        })
    }

    /// Polygon network, SushiSwapV2
    fn get_amm_manager(provider: &Arc<Provider<Http>>) -> &'static AmmManager {
        static AMM_MANAGER: OnceLock<AmmManager> = OnceLock::new();
        AMM_MANAGER.get_or_init(|| {
            let mut uniswap_v2 = Arc::new(AmmProtocolKind::UniswapV2(
                UniswapV2Protocol::new(
                    "SushiSwapV2",
                    "0xc35DADB65012eC5796536bD9864eD8773aBc74C4",
                    "0x1b02dA8Cb0d097eB8D57A175b88c7D8b47997506",
                )
                    .unwrap(),
            ));

            let token0_address: &str = "0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270";
            let token0 = CryptoToken::new(
                token0_address,
                None,
                "Wrapped Matic",
                "WMATIC",
                18,
                3,
                block_on(provider.get_code(Address::from_str(token0_address).unwrap(), None))
                    .unwrap()
                    .0,
            )
                .unwrap();

            let token1_address: &str = "0x7ceB23fD6bC0adD59E62ac25578270cFf1b9f619";
            let token1 = CryptoToken::new(
                token1_address,
                None,
                "Wrapped Ether",
                "WETH",
                18,
                0,
                block_on(provider.get_code(Address::from_str(token1_address).unwrap(), None))
                    .unwrap()
                    .0,
            )
                .unwrap();
            let pool = AmmPoolKind::UniswapV2(
                UniswapV2Pool::new(
                    Address::from_str("0xc4e595acDD7d12feC385E5dA5D43160e8A0bAC0E").unwrap(),
                    Arc::clone(&uniswap_v2),
                    Arc::new(token0),
                    Arc::new(token1),
                )
                    .unwrap(),
            );

            unsafe {
                let _uniswap_v2 = Arc::into_raw(uniswap_v2) as *mut AmmProtocolKind;
                (*_uniswap_v2).add_pool(pool);
                uniswap_v2 = Arc::from_raw(_uniswap_v2);
            }

            let amms: Vec<Arc<AmmProtocolKind>> = vec![uniswap_v2];
            AmmManager::new(amms)
        })
    }

    fn get_simulator(provider: &Arc<Provider<Http>>, amm_manager: &AmmManager) -> RevmSimulator<Provider<Http>> {
        RevmSimulator::new(Arc::clone(provider), amm_manager).unwrap()
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_get_amounts_out_uniswap_v2() {
        let provider: &Arc<Provider<Http>> = get_provider();
        let amm_manager: &AmmManager = get_amm_manager(provider);
        let pool: &Arc<AmmPoolKind> = amm_manager.amms().first().unwrap().pools().first().unwrap();
        let simulator: RevmSimulator<Provider<Http>> = get_simulator(provider, amm_manager);

        let result: U256 = simulator
            .get_amounts_out(pool, pool.token0(), pool.token0().convert_to_amount(1.0_f64))
            .unwrap();
        assert_ne!(result, U256::zero());
        assert!(result > U256::zero());

        println!(
            "{} -> {} = {}",
            pool.token0().symbol(),
            pool.token1().symbol(),
            pool.token1().convert_to_decimal(result)
        );
    }
}
