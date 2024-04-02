use std::str::FromStr;
use std::sync::{Arc, RwLock};

use anyhow::{anyhow, Result};
use ethers::utils::to_checksum;
use ethers::{
    abi,
    abi::AbiDecode,
    abi::AbiEncode,
    abi::AbiError,
    providers::Middleware,
    types::{
        Address as eAddress, Block as eBlock, BlockId as eBlockId, BlockNumber as eBlockNumber, Log as eLog,
        H256 as eH256, U256 as eU256,
    },
    utils::keccak256,
};
use hashbrown::HashMap;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use revm::db::CacheDB;
use revm::primitives::{Account, Log, U256};
use revm::{
    db::{EmptyDB, EthersDB},
    primitives::{
        AccountInfo, Address, Bytecode, Bytes, CfgEnv, ExecutionResult, HaltReason, Output, ResultAndState, State,
        TransactTo, TxEnv, KECCAK_EMPTY,
    },
    ContextWithHandlerCfg, DatabaseRef, Evm,
};

use contracts::erc20_token::{BalanceOfCall, BalanceOfReturn};
use contracts::simulator::{
    SimulateGetAmountsOutUniswapV2Call, SimulateGetAmountsOutUniswapV2Return, SimulatorAbiErrors,
    SIMULATORABI_DEPLOYED_BYTECODE,
};
use contracts::uniswap_v2_factory::{CreatePairCall, CreatePairReturn};
use vidger::{
    logger::{error, info, warn},
    types::NewBlock,
    utilities::block_on,
};

use crate::amm::{AmmPoolKind, AmmProtocolKind};
use crate::managers::AmmManager;
use crate::simulator::ThreadSafeInMemoryDB;
use crate::types::CryptoToken;

type RevmContext = ContextWithHandlerCfg<(), ThreadSafeInMemoryDB>;

#[derive(Debug, Clone)]
struct TxSuccessResult {
    pub output: Bytes,
    pub logs: Option<Vec<Log>>,
    pub gas_used: u64,
    pub gas_refunded: u64,
}

#[derive(Debug, Clone)]
struct TxRevertResult {
    pub output: SimulatorAbiErrors,
    pub gas_used: u64,
}

#[derive(Debug, Clone)]
struct TxHaltResult {
    pub reason: HaltReason,
    pub gas_used: u64,
}

enum TxResult {
    Success(TxSuccessResult),
    Revert(TxRevertResult),
    Halt(TxHaltResult),
}

pub struct EvmSimulator<M> {
    provider: Arc<M>,
    simulator_address: Address,
    account: Address,
    accounts_slots_to_update: HashMap<Address, Vec<U256>>,
    revm_ctx: RevmContext,
}

impl<M: Middleware + 'static> EvmSimulator<M> {
    #[inline]
    fn get_evm(revm_ctx: RevmContext) -> Evm<'static, (), ThreadSafeInMemoryDB> {
        Evm::builder().with_context_with_handler_cfg(revm_ctx).build()
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

    fn send_tx(
        revm_ctx: RevmContext,
        caller: Address,
        to: Address,
        calldata: Bytes,
        commit: bool,
    ) -> Result<(ExecutionResult, Option<State>)> {
        let mut evm: Evm<(), ThreadSafeInMemoryDB> = Self::get_evm(revm_ctx);
        let tx: &mut TxEnv = evm.tx_mut();
        tx.caller = caller;
        tx.transact_to = TransactTo::Call(to);
        tx.data = calldata;

        if commit {
            let result: ExecutionResult = evm.transact_commit()?;
            Ok((result, None))
        } else {
            let r: ResultAndState = evm.transact()?;
            Ok((r.result, Some(r.state)))
        }
    }

    pub fn new(provider: Arc<M>, amm_manager: &AmmManager) -> Result<Self> {
        // https://github.com/bluealloy/revm/issues/1062
        let hundred_grand_eth: U256 = U256::from(100_000)
            .checked_mul(U256::from(10).pow(U256::from(18)))
            .unwrap();
        let account = Address::from_str("0x9cf277A22EB4c551c6E18F7a6C0ee1893bcB034f").unwrap();

        // Prepare in-memory DB
        let mut db = ThreadSafeInMemoryDB::new(EmptyDB::new());
        let ethers_db: Arc<RwLock<EthersDB<M>>> = Arc::new(RwLock::new(
            EthersDB::new(provider.clone(), Some(eBlockId::Number(eBlockNumber::Latest))).unwrap(),
        ));

        // Give the user enough ETH to pay for gas
        let user_acc_info = AccountInfo::new(hundred_grand_eth, 0, KECCAK_EMPTY, Bytecode::default());
        db.insert_account_info(account, user_acc_info);

        // Deploy Simulator contract
        let simulator_address = Address::from_str("0xF2d01Ee818509a9540d8324a5bA52329af27D19E").unwrap();
        let simulator_bytes = Bytecode::new_raw((*SIMULATORABI_DEPLOYED_BYTECODE.0).into());
        let simulator_acc_info = AccountInfo::new(hundred_grand_eth, 0, simulator_bytes.hash_slow(), simulator_bytes);
        db.insert_account_info(simulator_address, simulator_acc_info);

        // Create EVM
        let mut evm: Evm<'static, (), ThreadSafeInMemoryDB> = Evm::builder().with_db(db).build();

        // overriding some default env values to make it more efficient for testing
        let evm_cfg: &mut CfgEnv = evm.cfg_mut();
        evm_cfg.limit_contract_code_size = None;
        evm_cfg.disable_block_gas_limit = true;
        evm_cfg.disable_base_fee = true;

        // Create context
        let ctx_with_handler: RevmContext = evm.into_context_with_handler_cfg();

        let mut ret = Self {
            provider,
            simulator_address,
            account,
            accounts_slots_to_update: HashMap::new(),
            revm_ctx: ctx_with_handler,
        };

        // Deploy amm
        let address_to_spoof: Vec<Address> = vec![simulator_address, account];
        for amm in amm_manager.amms() {
            ret.deploy_full_amm(&ethers_db, amm, &address_to_spoof);
        }

        Ok(ret)
    }
}

impl<M> EvmSimulator<M> {
    #[inline]
    pub fn provider(&self) -> &Arc<M> {
        &self.provider
    }
}

impl<M> EvmSimulator<M>
where
    M: Middleware + 'static,
{
    #[inline]
    fn get_storage_at(&self, address: Address, slot: U256) -> U256 {
        self.revm_ctx.context.evm.db.storage_ref(address, slot).unwrap()
    }

    fn deploy_token_and_spoof_balance(
        &mut self,
        ethers_db: &Arc<RwLock<EthersDB<M>>>,
        token: &CryptoToken,
        accounts_list_to_spoof: &[Address],
    ) {
        let token_address: Address = token.address().0.into();

        // Skip if already deployed
        let db: &ThreadSafeInMemoryDB = &self.revm_ctx.context.evm.db;
        if db.0.read().unwrap().accounts.get(&token_address).is_some() {
            return;
        }

        // Deploy proxy
        if let Some(proxy_address) = token.proxy_address() {
            self.deploy_contracts(ethers_db, &[proxy_address.0.into()]);
        }

        // Deploy token
        let hundred_grand_eth: U256 = U256::from(100_000)
            .checked_mul(U256::from(10).pow(U256::from(18)))
            .unwrap();
        let code = Bytecode::new_raw(Bytes::from(token.code().clone()));
        let token_acc_info = AccountInfo::new(hundred_grand_eth, 0, code.hash_slow(), code);

        // Spoof balance
        // https://ethereum.stackexchange.com/questions/147205/how-to-view-the-amount-of-storage-a-contract-uses
        // https://ethereum.stackexchange.com/questions/47986/using-getstorageat-on-mappingaddress-uint64
        let input_balance_slots: Option<Vec<U256>> = if token.balance_contract_slot() != -1 {
            // Inject simulator contract with token balance
            // Inject account with token balance
            let slots_to_spoof: Vec<U256> = accounts_list_to_spoof
                .iter()
                .map(|address_to_spoof| {
                    U256::from_be_bytes(keccak256(abi::encode(&[
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
        let db: &mut ThreadSafeInMemoryDB = &mut self.revm_ctx.context.evm.db;
        db.insert_account_info(token_address, token_acc_info);

        if let Some(input_balance_slots) = input_balance_slots {
            for input_balance_slot_index in input_balance_slots {
                db.insert_account_storage(token_address, input_balance_slot_index, hundred_grand_eth)
                    .expect("failed to insert token balance in DB");
            }
        }
    }

    fn deploy_pool(
        &mut self,
        ethers_db: &Arc<RwLock<EthersDB<M>>>,
        pool: &AmmPoolKind,
        accounts_list_to_spoof: &[Address],
    ) {
        /*
        NOTE:
            We can notice that it retrieves the balance of token0, token1 making calls to the token contracts.
            This means that our newly deployed pair contract has to have real token balances to perform a real swap.
        */

        // Add pool to spoof
        // Don't change `accounts_list_to_spoof` to mutable vector,
        // as it will keep push all pools to spoof form the caller
        let mut accounts_list_to_spoof: Vec<Address> = accounts_list_to_spoof.to_vec();
        accounts_list_to_spoof.push(pool.address().0.into());

        // Deploy tokens
        self.deploy_token_and_spoof_balance(ethers_db, pool.token0(), &accounts_list_to_spoof);
        self.deploy_token_and_spoof_balance(ethers_db, pool.token1(), &accounts_list_to_spoof);

        // Deploy pool
        let pool_address: Address = pool.address().0.into();
        let db: &mut ThreadSafeInMemoryDB = &mut self.revm_ctx.context.evm.db;
        if db.0.read().unwrap().accounts.get(&pool_address).is_some() {
            panic!("Pool already deployed: {}", pool_address);
        }

        let pool_acc_info: AccountInfo = ethers_db.read().unwrap().basic_ref(pool_address).unwrap().unwrap();

        // Prepare pool
        let mut slots: HashMap<U256, U256> = HashMap::new();
        match pool {
            AmmPoolKind::UniswapV2(_) => {
                const RESERVE_SLOT: u32 = 8;

                /*
                Why set unlocked slot?
                  Because it is only initialized when the pool is initialized with Creation Bytecode
                  and the default value is 0
                */

                let calldata: Bytes = AbiEncode::encode(CreatePairCall {
                    token_a: pool.token0().address().0.into(),
                    token_b: pool.token1().address().0.into(),
                })
                .into();

                let tx_result: Result<(ExecutionResult, Option<State>)> = Self::send_tx(
                    self.revm_ctx.clone(),
                    self.account.0.into(),
                    pool.dex().factory().0.into(),
                    calldata.clone(),
                    true, // Should commit
                );

                let result_and_state: (ExecutionResult, Option<State>) = match tx_result {
                    Ok(result) => result,
                    Err(e) => panic!("Failed to deploy pool: {}", e),
                };

                match result_and_state.0 {
                    ExecutionResult::Success { .. } => {}
                    ExecutionResult::Revert { output, .. } => {
                        println!("Try decode revert output: {:?}", output);

                        let error: String = <String as AbiDecode>::decode(output).unwrap();
                        panic!("Failed to deploy pool (Revert): {:?}", error);
                    }
                    ExecutionResult::Halt { reason, .. } => {
                        panic!("Failed to deploy pool(Halt): {:?}", reason);
                    }
                }

                let result: &Bytes = result_and_state.0.output().unwrap();
                assert_eq!(
                    CreatePairReturn::decode(result).unwrap().pair,
                    *pool.address(),
                    "Invalid pool address deployed. (Factory: {})",
                    to_checksum(pool.dex().factory(), None)
                );

                // Inject reserves
                let slot_idx = U256::from(RESERVE_SLOT);
                if let Ok(slot_value) = ethers_db.read().unwrap().storage_ref(pool_address, slot_idx) {
                    slots.insert(slot_idx, slot_value);
                } else {
                    warn!("Failed to get slot '{}' from pool '{}'", slot_idx, pool_address);
                }

                // Only reserve slot needs to be updated
                self.accounts_slots_to_update
                    .insert(pool_address, vec![U256::from(RESERVE_SLOT)]);
            }
        };

        // Commit
        let db: &mut ThreadSafeInMemoryDB = &mut self.revm_ctx.context.evm.db;
        db.insert_account_info(pool_address, pool_acc_info);

        for (slot, value) in slots {
            db.insert_account_storage(pool_address, slot, value)
                .expect("failed to insert pool reserves in DB");
        }
    }

    fn deploy_contracts(&mut self, ethers_db: &Arc<RwLock<EthersDB<M>>>, contracts: &[Address]) {
        // TODO: Parallelize
        let account_info_list: Vec<(Address, AccountInfo)> = contracts
            .iter()
            .map(|c: &Address| (*c, ethers_db.read().unwrap().basic_ref(*c).unwrap().unwrap()))
            .collect();

        // Commit
        let db: &mut ThreadSafeInMemoryDB = &mut self.revm_ctx.context.evm.db;
        for (acc, info) in account_info_list {
            db.insert_account_info(acc, info);
        }
    }

    fn deploy_amm(&mut self, ethers_db: &Arc<RwLock<EthersDB<M>>>, amm: &Arc<AmmProtocolKind>) {
        // TODO: For uniswap_v3 you need quoter and router
        match &**amm {
            AmmProtocolKind::UniswapV2(uniswap2) => {
                let router_address: Address = uniswap2.router().0.into();
                let factory_address: Address = uniswap2.factory().0.into();

                self.deploy_contracts(ethers_db, &[factory_address, router_address]);
            }
        };
    }

    fn deploy_full_amm(
        &mut self,
        ethers_db: &Arc<RwLock<EthersDB<M>>>,
        amm: &Arc<AmmProtocolKind>,
        accounts_to_spoof: &[Address],
    ) {
        self.deploy_amm(ethers_db, amm);

        // Deploy pools
        // TODO: Parallelize
        for pool in amm.pools() {
            self.deploy_pool(ethers_db, pool, accounts_to_spoof);
        }
    }

    pub fn sync_by_block(&mut self, new_block: &NewBlock, logs: &[eLog]) {
        // Get touched addresses that need to be updated
        let touched_addresses: HashMap<Address, &Vec<U256>> = logs
            .par_iter()
            .filter_map(|log: &eLog| {
                let address: Address = log.address.0.into();
                let slots_to_update: &Vec<U256> = self.accounts_slots_to_update.get(&address)?;
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
        let mut slots_values: HashMap<Address, (U256, U256)> = HashMap::new();
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
        let db: &mut ThreadSafeInMemoryDB = &mut self.revm_ctx.context.evm.db;
        for (address, (slot, value)) in slots_values {
            db.insert_account_storage(address, slot, value)
                .expect("failed to slot storage value");
        }

        info!("Simulator updated storage values for block '{}'", new_block.number);
    }

    // TODO: self should not be mutable
    pub fn get_tokens_balance_slot(&mut self, tokens: &[eAddress]) -> Result<HashMap<eAddress, Result<Option<i32>>>> {
        // Get token account info from ethers middleware and insert it into EVM
        let cur_block: eBlock<eH256> =
            block_on(self.provider.get_block(eBlockNumber::Latest))?.ok_or(anyhow!("failed to retrieve block"))?;
        let ethers_db: EthersDB<M> =
            EthersDB::new(Arc::clone(&self.provider), Some(cur_block.number.unwrap().into())).unwrap();

        let tokens_accounts: Vec<(AccountInfo, Address)> = tokens
            .par_iter()
            .map(|token: &eAddress| {
                let token: Address = token.0.into();
                let token_acc_info: AccountInfo = ethers_db.basic_ref(token).unwrap().unwrap();

                (token_acc_info, token)
            })
            .collect();

        // TODO: Should make another DB for this, since we only need to get balance slot
        let db: &mut ThreadSafeInMemoryDB = &mut self.revm_ctx.context.evm.db;
        for (token_acc_info, token) in tokens_accounts {
            db.insert_account_info(token, token_acc_info);
        }

        // Call balanceOf
        let calldata: Bytes = AbiEncode::encode(BalanceOfCall {
            who: self.account.0 .0.into(),
        })
        .into();

        let ret: HashMap<eAddress, Result<Option<i32>>> = tokens
            .par_iter()
            .map(|token: &eAddress| {
                let _token: Address = token.0.into();
                let tx_result: Result<(ExecutionResult, Option<State>)> = Self::send_tx(
                    self.revm_ctx.clone(),
                    self.account.0.into(),
                    _token,
                    calldata.clone(),
                    false,
                );

                let result_and_state: (ExecutionResult, Option<State>) = match tx_result {
                    Ok(result) => result,
                    Err(e) => {
                        return (*token, Err(anyhow!("EVM call failed: {e:?}")));
                    }
                };

                // Get touched storage
                let token_acc: &Account = result_and_state.1.as_ref().unwrap().get(&_token).unwrap();
                let touched_storage: &revm::primitives::Storage = &token_acc.storage;
                println!("Touched storage slots: {:?}", touched_storage);

                // Some tokens have a lot of storage slots like
                // https://polygonscan.com/token/0x9C9e5fD8bbc25984B178FdCE6117Defa39d2db39
                // balance slot are 51
                for i in 0..200 {
                    let slot: [u8; 32] = keccak256(&abi::encode(&[
                        abi::Token::Address(self.account.0 .0.into()),
                        abi::Token::Uint(eU256::from(i)),
                    ]));

                    let slot = U256::from_be_bytes(slot);
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

    pub fn get_token_balance(&self, token: Address) -> Result<eU256> {
        let calldata: Bytes = AbiEncode::encode(BalanceOfCall {
            who: self.account.0 .0.into(),
        })
        .into();

        let tx_result: Result<(ExecutionResult, Option<State>)> = Self::send_tx(
            self.revm_ctx.clone(),
            self.account.0.into(),
            token.0.into(),
            calldata.clone(),
            false,
        );

        let result_and_state: (ExecutionResult, Option<State>) = match tx_result {
            Ok(result) => result,
            Err(e) => return Err(anyhow!("EVM call failed: {e:?}")),
        };

        let tx_result: TxResult = Self::get_tx_result(result_and_state.0);
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

    pub fn get_amounts_out(&self, pool: &AmmPoolKind, input: &CryptoToken, amount_in: eU256) -> Result<eU256> {
        // TODO: For uniswap_v3 `contract_address` are the quarter
        let calldata: Bytes = match pool {
            AmmPoolKind::UniswapV2(_) => {
                //let path: ethers::types::Bytes = abi::encode(&[abi::Token::Array(vec![
                //    abi::Token::Address(*pool.token0().address()),
                //    abi::Token::Address(*pool.token1().address()),
                //])])
                //.into();

                let path: Vec<eAddress> = if pool.token0().address() == input.address() {
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
        }
        .into();

        let tx_result: Result<(ExecutionResult, Option<State>)> = Self::send_tx(
            self.revm_ctx.clone(),
            self.account.0.into(),
            self.simulator_address,
            calldata.clone(),
            false,
        );

        let result_and_state: (ExecutionResult, Option<State>) = match tx_result {
            Ok(result) => result,
            Err(e) => return Err(anyhow!("EVM call failed: {e:?}")),
        };

        let tx_result: TxResult = Self::get_tx_result(result_and_state.0);
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

    const SUSHI_NAME: &str = "SushiSwap V2";
    const SUSHI_FACTORY: &str = "0xc35DADB65012eC5796536bD9864eD8773aBc74C4";
    const SUSHI_ROUTER: &str = "0x1b02dA8Cb0d097eB8D57A175b88c7D8b47997506";
    const SUSHI_WMATIC_WETH_POOL: &str = "0xc4e595acDD7d12feC385E5dA5D43160e8A0bAC0E";
    const SUSHI_WMATIC_USDC_POOL: &str = "0x96c7FC08D8CDACdB95a8613b19fffe4D54307263";
    const GRAVITY_FINANCE_FACTORY: &str = "0x3ed75AfF4094d2Aaa38FaFCa64EF1C152ec1Cf20";
    const GRAVITY_FINANCE_ROUTER: &str = "0x57dE98135e8287F163c59cA4fF45f1341b680248";
    const GRAVITY_FINANCE_WMATIC_WETH_POOL: &str = "0x0Dfbf1A50bdcB570Bd0fF7Bb307313B553a02598";

    fn get_provider() -> Arc<Provider<Http>> {
        Arc::new(
            Provider::<Http>::try_from("https://polygon-mainnet.infura.io/v3/c230ccbf294b44bcac907f4a719d06c4")
                .unwrap(),
        )
    }

    fn make_token(
        provider: &Arc<Provider<Http>>,
        address: &str,
        proxy: Option<&str>,
        name: &str,
        symbol: &str,
        decimals: u8,
        balance_slot: i32,
    ) -> CryptoToken {
        CryptoToken::new(
            address,
            proxy,
            name,
            symbol,
            decimals,
            balance_slot,
            block_on(provider.get_code(eAddress::from_str(address).unwrap(), None))
                .unwrap()
                .0,
        )
        .unwrap()
    }

    fn weth_token(provider: &Arc<Provider<Http>>) -> &'static CryptoToken {
        static TOKEN: OnceLock<CryptoToken> = OnceLock::new();
        TOKEN.get_or_init(|| {
            make_token(
                provider,
                "0x7ceB23fD6bC0adD59E62ac25578270cFf1b9f619",
                None,
                "Wrapped Ether",
                "WETH",
                18,
                0,
            )
        })
    }

    fn wmatic_token(provider: &Arc<Provider<Http>>) -> &'static CryptoToken {
        static TOKEN: OnceLock<CryptoToken> = OnceLock::new();
        TOKEN.get_or_init(|| {
            make_token(
                provider,
                "0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270",
                None,
                "Wrapped Matic",
                "WMATIC",
                18,
                3,
            )
        })
    }

    fn quick_token(provider: &Arc<Provider<Http>>) -> &'static CryptoToken {
        static TOKEN: OnceLock<CryptoToken> = OnceLock::new();
        TOKEN.get_or_init(|| {
            make_token(
                provider,
                "0xB5C064F955D8e7F38fE0460C556a72987494eE17",
                None,
                "QuickSwap",
                "QUICK",
                18,
                2,
            )
        })
    }

    fn usdc(provider: &Arc<Provider<Http>>) -> &'static CryptoToken {
        static TOKEN: OnceLock<CryptoToken> = OnceLock::new();
        TOKEN.get_or_init(|| {
            make_token(
                provider,
                "0x3c499c542cEF5E3811e1192ce70d8cC03d5c3359",
                Some("0x235AE97b28466Db30469b89A9fe4cFf0659f82Cb"),
                "USD Coin",
                "USDC",
                6,
                9,
            )
        })
    }

    fn usdt(provider: &Arc<Provider<Http>>) -> &'static CryptoToken {
        static TOKEN: OnceLock<CryptoToken> = OnceLock::new();
        TOKEN.get_or_init(|| {
            make_token(
                provider,
                "0xc2132D05D31c914a87C6611C10748AEb04B58e8F",
                Some("0x7FFB3d637014488b63fb9858E279385685AFc1e2"),
                "Tether USD",
                "USDT",
                6,
                0,
            )
        })
    }

    /// Polygon network, SushiSwapV2, WMATIC, WETH
    fn get_uniswap_v2_amm_manager(
        amm_name: &str,
        factory: &str,
        router: &str,
        pool_address: &str,
        token0: &CryptoToken,
        token1: &CryptoToken,
    ) -> AmmManager {
        let mut uniswap_v2 = Arc::new(AmmProtocolKind::UniswapV2(
            UniswapV2Protocol::new(amm_name, factory, router).unwrap(),
        ));

        let pool = AmmPoolKind::UniswapV2(
            UniswapV2Pool::new(
                eAddress::from_str(pool_address).unwrap(),
                Arc::clone(&uniswap_v2),
                Arc::new(token0.clone()),
                Arc::new(token1.clone()),
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
    }

    fn get_simulator(provider: &Arc<Provider<Http>>, amm_manager: &AmmManager) -> EvmSimulator<Provider<Http>> {
        EvmSimulator::new(Arc::clone(provider), amm_manager).unwrap()
    }

    fn get_amounts_out_uniswap_v2_no_proxy_tokens(provider: &Arc<Provider<Http>>, amm_manager: AmmManager) {
        let pool: &Arc<AmmPoolKind> = amm_manager.amms().first().unwrap().pools().first().unwrap();
        let simulator: EvmSimulator<Provider<Http>> = get_simulator(provider, &amm_manager);

        let result: eU256 = simulator
            .get_amounts_out(pool, pool.token0(), pool.token0().convert_to_amount(1.0_f64))
            .unwrap();
        assert_ne!(result, eU256::zero());
        assert!(result > eU256::zero());

        println!(
            "{} -> {} = {}",
            pool.token0().symbol(),
            pool.token1().symbol(),
            pool.token1().convert_to_decimal(result)
        );
    }

    fn balance_of_tokens(provider: &Arc<Provider<Http>>, tokens: &[&CryptoToken]) {
        let mut simulator: EvmSimulator<Provider<Http>> = get_simulator(provider, &AmmManager::new(vec![]));
        let ethers_db = Arc::new(RwLock::new(EthersDB::new(Arc::clone(provider), None).unwrap()));

        let calldata: Bytes = AbiEncode::encode(BalanceOfCall {
            who: simulator.account.0 .0.into(),
        })
        .into();

        let accounts_to_spoof = [simulator.simulator_address, simulator.account];
        for token in tokens {
            simulator.deploy_token_and_spoof_balance(&ethers_db, token, &accounts_to_spoof);

            let result_and_state: (ExecutionResult, Option<State>) = EvmSimulator::<Provider<Http>>::send_tx(
                simulator.revm_ctx.clone(),
                simulator.account,
                token.proxy_or_address().0.into(),
                calldata.clone(),
                false,
            )
            .unwrap();
            assert!(result_and_state.0.is_success(), "{} send_tx failed", token.symbol());

            let data: &Bytes = result_and_state.0.output().unwrap();
            assert_ne!(data.len(), 0, "{} data: {:?}", token.symbol(), data);

            let balance = BalanceOfReturn::decode(data).unwrap();
            assert_ne!(balance.0, eU256::zero(), "{} balance: {:?}", token.symbol(), balance.0);
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_balance_of_no_proxy_tokens() {
        let provider: Arc<Provider<Http>> = get_provider();

        let tokens = [wmatic_token(&provider), weth_token(&provider), quick_token(&provider)];
        balance_of_tokens(&provider, &tokens);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_balance_of_proxy_tokens() {
        let provider: Arc<Provider<Http>> = get_provider();

        let tokens = [usdc(&provider), usdt(&provider)];
        balance_of_tokens(&provider, &tokens);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_get_amounts_out_uniswap_v2_no_proxy_tokens() {
        let provider: Arc<Provider<Http>> = get_provider();
        let token0: &CryptoToken = wmatic_token(&provider);
        let token1: &CryptoToken = weth_token(&provider);

        let amm_manager: AmmManager = get_uniswap_v2_amm_manager(
            SUSHI_NAME,
            SUSHI_FACTORY,
            SUSHI_ROUTER,
            SUSHI_WMATIC_WETH_POOL,
            token0,
            token1,
        );

        get_amounts_out_uniswap_v2_no_proxy_tokens(&provider, amm_manager);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_get_amounts_out_uniswap_v2_proxy_tokens() {
        let provider: Arc<Provider<Http>> = get_provider();

        let token0: &CryptoToken = wmatic_token(&provider);
        let token1: &CryptoToken = usdc(&provider);

        let amm_manager: AmmManager = get_uniswap_v2_amm_manager(
            SUSHI_NAME,
            SUSHI_FACTORY,
            SUSHI_ROUTER,
            SUSHI_WMATIC_USDC_POOL,
            token0,
            token1,
        );

        get_amounts_out_uniswap_v2_no_proxy_tokens(&provider, amm_manager);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_gravity_finance() {
        let provider: Arc<Provider<Http>> = get_provider();

        let token0: &CryptoToken = wmatic_token(&provider);
        let token1: &CryptoToken = weth_token(&provider);

        let amm_manager: AmmManager = get_uniswap_v2_amm_manager(
            "GravityFinanceV2",
            GRAVITY_FINANCE_FACTORY,
            GRAVITY_FINANCE_ROUTER,
            GRAVITY_FINANCE_WMATIC_WETH_POOL,
            token0,
            token1,
        );

        get_amounts_out_uniswap_v2_no_proxy_tokens(&provider, amm_manager);
    }
}
