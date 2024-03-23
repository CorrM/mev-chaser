use std::str::FromStr;
use std::sync::{Arc, RwLock, RwLockWriteGuard};

use anyhow::{anyhow, Result};
use ethers::abi::{AbiDecode, AbiEncode};
use ethers::prelude::BlockId;
use ethers::{
    abi,
    providers::Middleware,
    types::{Address, Block, BlockNumber, H256, U256},
    utils::keccak256,
};
use hashbrown::HashMap;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use revm::db::CacheDB;
use revm::primitives::HaltReason;
use revm::{
    db::{EmptyDB, EthersDB, InMemoryDB},
    primitives::{
        AccountInfo, Bytecode, CfgEnv, ExecutionResult, Output, ResultAndState, TransactTo, TxEnv, KECCAK_EMPTY,
    },
    ContextWithHandlerCfg, DatabaseRef, Evm,
};

use contracts::balancer_flash_loan_recipient::OneSwapInfo;
use contracts::erc20_token::{BalanceOfCall, BalanceOfReturn};
use contracts::simulator::{
    SimulateGetAmountsOutCall, SimulateGetAmountsOutReturn, SimulatorAbiErrors, SIMULATORABI_DEPLOYED_BYTECODE,
};
use vidger::logger::error;
use vidger::types::NewBlock;
use vidger::utilities::block_on;

use crate::amm::{AmmPoolKind, AmmProtocolKind};
use crate::managers::AmmManager;
use crate::types::CryptoToken;

/*
use ethers::{
    abi::{self, AbiDecode, AbiEncode},
    providers::Middleware,
    types::{Address, H256, U256, U64},
    utils::keccak256,
};
*/
#[derive(Debug, Clone)]
pub struct TxSuccessResult {
    pub output: revm::primitives::Bytes,
    pub logs: Option<Vec<revm::primitives::Log>>,
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
    simulator_address: revm::primitives::Address,
    account: revm::primitives::Address,
    ctx_with_handler: ContextWithHandlerCfg<(), InMemoryDB>,
}

impl<M: Middleware + 'static> RevmSimulator<M> {
    fn deploy_amm(db: &Arc<RwLock<InMemoryDB>>, ethers_db: &Arc<RwLock<EthersDB<M>>>, amm: &Arc<AmmProtocolKind>) {
        let mut account_info_list: Vec<(revm::primitives::Address, AccountInfo)> = vec![];

        // TODO: For uniswap_v3 you need quoter and router
        match &**amm {
            AmmProtocolKind::UniswapV2(uniswap2) => {
                let router_address: revm::primitives::Address = uniswap2.router().0.into();
                account_info_list.push((
                    router_address,
                    ethers_db.read().unwrap().basic_ref(router_address).unwrap().unwrap(),
                ));
            }
        };

        // Commit
        let mut write_guard: RwLockWriteGuard<InMemoryDB> = db.write().unwrap();
        for (acc, info) in account_info_list {
            write_guard.insert_account_info(acc, info);
        }
    }

    fn deploy_token_and_spoof_balance(
        db: &Arc<RwLock<InMemoryDB>>,
        token: &CryptoToken,
        address: &[revm::primitives::Address],
    ) {
        let hundred_grand_eth: revm::primitives::U256 = revm::primitives::U256::from(100_000)
            .checked_mul(revm::primitives::U256::from(10).pow(revm::primitives::U256::from(18)))
            .unwrap();

        // Deploy token
        let token_address: revm::primitives::Address = token.proxy_or_address().0.into();
        let code = Bytecode::new_raw(revm::primitives::Bytes::from(token.code().clone()));
        let token_acc_info = AccountInfo::new(hundred_grand_eth, 0, code.hash_slow(), code);

        // Spoof balance
        // https://ethereum.stackexchange.com/questions/147205/how-to-view-the-amount-of-storage-a-contract-uses
        // https://ethereum.stackexchange.com/questions/47986/using-getstorageat-on-mappingaddress-uint64
        let input_balance_slots: Option<Vec<revm::primitives::U256>> = if token.balance_contract_slot() != -1 {
            // Inject simulator contract with token balance
            // Inject account with token balance
            let slots_to_spoof: Vec<revm::primitives::U256> = address
                .iter()
                .map(|address_to_spoof| {
                    revm::primitives::U256::from_be_bytes(keccak256(abi::encode(&[
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
        let mut write_guard: RwLockWriteGuard<InMemoryDB> = db.write().unwrap();
        write_guard.insert_account_info(token_address, token_acc_info);

        if let Some(input_balance_slots) = input_balance_slots {
            for input_balance_slot_index in input_balance_slots {
                write_guard
                    .insert_account_storage(token_address, input_balance_slot_index, hundred_grand_eth)
                    .expect("failed to insert token balance in DB");
            }
        }
    }

    fn deploy_pool(
        db: &Arc<RwLock<InMemoryDB>>,
        ethers_db: &Arc<RwLock<EthersDB<M>>>,
        pool: &AmmPoolKind,
        address: &[revm::primitives::Address],
    ) {
        // Deploy tokens
        Self::deploy_token_and_spoof_balance(db, pool.token0(), address);
        Self::deploy_token_and_spoof_balance(db, pool.token1(), address);

        // Deploy pool
        let pool_address: revm::primitives::Address = pool.address().0.into();
        let pool_acc_info: AccountInfo = ethers_db.read().unwrap().basic_ref(pool_address).unwrap().unwrap();

        // Prepare pool
        let mut slots: HashMap<revm::primitives::U256, revm::primitives::U256> = HashMap::new();
        match pool {
            AmmPoolKind::UniswapV2(_) => {
                let slot = revm::primitives::U256::from(8);
                if let Ok(slot_value) = ethers_db.read().unwrap().storage_ref(pool_address, slot) {
                    slots.insert(slot, slot_value);
                };
            }
        };

        // Commit
        let mut write_guard: RwLockWriteGuard<InMemoryDB> = db.write().unwrap();
        write_guard.insert_account_info(pool_address, pool_acc_info);

        for (slot, value) in slots {
            write_guard
                .insert_account_storage(pool_address, slot, value)
                .expect("failed to insert pool reserves in DB");
        }
    }

    pub(super) fn new(provider: Arc<M>, amm_manager: &AmmManager) -> Result<Self> {
        // https://github.com/bluealloy/revm/issues/1062
        let hundred_grand_eth: revm::primitives::U256 = revm::primitives::U256::from(100_000)
            .checked_mul(revm::primitives::U256::from(10).pow(revm::primitives::U256::from(18)))
            .unwrap();
        let account = revm::primitives::Address::from_str("0x9cf277A22EB4c551c6E18F7a6C0ee1893bcB034f").unwrap();

        // Prepare in-memory DB
        let mut db = InMemoryDB::new(EmptyDB::default());
        let ethers_db: Arc<RwLock<EthersDB<M>>> = Arc::new(RwLock::new(
            EthersDB::new(provider.clone(), Some(BlockId::Number(BlockNumber::Latest))).unwrap(),
        ));

        // Give the user enough ETH to pay for gas
        let user_acc_info = AccountInfo::new(hundred_grand_eth, 0, KECCAK_EMPTY, Bytecode::default());
        db.insert_account_info(account, user_acc_info); // TODO: Remove .into()

        // Deploy Simulator contract
        let simulator_address =
            revm::primitives::Address::from_str("0xF2d01Ee818509a9540d8324a5bA52329af27D19E").unwrap();
        let simulator_bytes = Bytecode::new_raw((*SIMULATORABI_DEPLOYED_BYTECODE.0).into());
        let simulator_acc_info = AccountInfo::new(hundred_grand_eth, 0, simulator_bytes.hash_slow(), simulator_bytes);
        db.insert_account_info(simulator_address, simulator_acc_info);

        // Deploy amm contracts
        let db: Arc<RwLock<InMemoryDB>> = Arc::new(RwLock::new(db));
        for amm in amm_manager.amms() {
            Self::deploy_amm(&db, &ethers_db, amm);
        }

        // Deploy pools
        // TODO: Parallelize
        let address_to_spoof = [simulator_address, account];
        for amm in amm_manager.amms() {
            for pool in amm.pools() {
                Self::deploy_pool(&db, &ethers_db, pool, &address_to_spoof)
            }
        }

        // Create EVM
        let db: InMemoryDB = Arc::try_unwrap(db).unwrap().into_inner().unwrap();
        let mut evm: Evm<'static, (), InMemoryDB> = Evm::builder().with_db(db).build();

        // overriding some default env values to make it more efficient for testing
        let evm_cfg: &mut CfgEnv = evm.cfg_mut();
        evm_cfg.limit_contract_code_size = None;
        evm_cfg.disable_block_gas_limit = true;
        evm_cfg.disable_base_fee = true;

        // Create context
        let ctx_with_handler: ContextWithHandlerCfg<(), InMemoryDB> = evm.into_context_with_handler_cfg();

        Ok(Self {
            provider,
            simulator_address,
            account,
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
    fn get_evm(&self) -> Evm<(), InMemoryDB> {
        Evm::builder()
            .with_context_with_handler_cfg(self.ctx_with_handler.clone())
            .build()
    }

    #[inline]
    fn clone_evm(&self, context_with_handler_cfg: ContextWithHandlerCfg<(), InMemoryDB>) -> Evm<(), InMemoryDB> {
        Evm::builder()
            .with_context_with_handler_cfg(context_with_handler_cfg)
            .build()
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

                TxResult::Revert(TxRevertResult {
                    // TODO: That's not necessary
                    output: SimulatorAbiErrors::decode(output).unwrap(),
                    gas_used,
                })
            }
            ExecutionResult::Halt { reason, gas_used } => TxResult::Halt(TxHaltResult { reason, gas_used }),
        };

        output
    }

    pub fn on_new_block(&mut self, block: &NewBlock) {
        // Update all touched AccountInfo in the db
    }

    pub fn get_tokens_balance_slot(&self, tokens: &[Address]) -> Result<HashMap<Address, Result<Option<i32>>>> {
        let mut evm: Evm<(), InMemoryDB> = self.get_evm();

        // Get token account info from ethers middleware and insert it into EVM
        let cur_block: Block<H256> =
            block_on(self.provider.get_block(BlockNumber::Latest))?.ok_or(anyhow!("failed to retrieve block"))?;
        let ethers_db: EthersDB<M> =
            EthersDB::new(Arc::clone(&self.provider), Some(cur_block.number.unwrap().into())).unwrap();

        let tokens_accounts: Vec<(AccountInfo, revm::primitives::Address)> = tokens
            .par_iter()
            .map(|token: &Address| {
                let token: revm::primitives::Address = token.0.into();
                let token_acc_info: AccountInfo = ethers_db.basic_ref(token).unwrap().unwrap();

                (token_acc_info, token)
            })
            .collect();

        for (token_acc_info, token) in tokens_accounts {
            evm.db_mut().insert_account_info(token, token_acc_info);
        }

        // Call balanceOf
        let calldata: revm::primitives::Bytes = AbiEncode::encode(BalanceOfCall {
            who: self.account.0 .0.into(),
        })
        .into();

        let handler_cfg: &ContextWithHandlerCfg<(), InMemoryDB> = &evm.into_context_with_handler_cfg();
        let ret: HashMap<Address, Result<Option<i32>>> = tokens
            .par_iter()
            .map(|token: &Address| {
                let mut evm: Evm<(), InMemoryDB> = self.clone_evm(handler_cfg.clone());

                let _token: revm::primitives::Address = token.0.into();

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

                    let slot = revm::primitives::U256::from_be_bytes(slot);
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

        let mut evm: Evm<(), InMemoryDB> = self.get_evm();
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

    pub fn get_amounts_out(&self, pool: &AmmPoolKind, amount_in: U256) -> Result<U256> {
        let path: ethers::types::Bytes = abi::encode(&[abi::Token::Array(vec![
            abi::Token::Address(*pool.token0().address()),
            abi::Token::Address(*pool.token1().address()),
        ])])
        .into();

        println!("path: {}", path);

        let calldata: Vec<u8> = match pool {
            AmmPoolKind::UniswapV2(_) => AbiEncode::encode(SimulateGetAmountsOutCall {
                protocol: 0, // UniswapV2
                contract_address: pool.dex().router().0.into(),
                path,
                amount_in,
            }),
        };

        let mut evm: Evm<(), InMemoryDB> = self.get_evm();
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
            TxResult::Success(result) => {
                let Ok(decoded_output) = SimulateGetAmountsOutReturn::decode(&result.output) else {
                    return Err(anyhow!("Failed to decode output"));
                };

                Ok(decoded_output.0)
            }
            TxResult::Revert(revert) => Err(anyhow!("Failed to get token balance REVERT: {:?}", revert.output)),
            TxResult::Halt(halt) => Err(anyhow!("Failed to get token balance HALT: {:?}", halt.reason)),
        }
    }

    pub fn multi_swaps(&self, swaps: Vec<OneSwapInfo>, chain_swaps: bool) -> Result<U256> {
        todo!()
    }
}
