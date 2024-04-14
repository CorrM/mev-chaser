use std::collections::BTreeMap;
use std::str::FromStr;
use std::sync::{Arc, OnceLock, RwLock};

use anyhow::{anyhow, Result};
use ethers::{
    abi,
    abi::AbiDecode,
    abi::AbiEncode,
    abi::AbiError,
    providers::Middleware,
    types::{
        transaction::eip2718::TypedTransaction, AccountState as eAccountState, Address as eAddress, Block as eBlock,
        BlockId as eBlockId, BlockNumber as eBlockNumber, CallConfig, CallFrame, GethDebugBuiltInTracerConfig,
        GethDebugBuiltInTracerType, GethDebugTracerConfig, GethDebugTracerType, GethDebugTracingCallOptions,
        GethDebugTracingOptions, GethTrace, GethTraceFrame, Log as eLog, PreStateFrame, PreStateMode,
        Transaction as eTransaction, TransactionRequest, H256 as eH256, U256 as eU256, U64 as eU64,
    },
    utils::keccak256,
    utils::to_checksum,
};
use hashbrown::HashMap;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use revm::primitives::{Account, Log, Storage, U256};
use revm::{
    db::{EmptyDB, EthersDB},
    primitives::{
        AccountInfo, Address, Bytecode, Bytes, CfgEnv, ExecutionResult, HaltReason, Output, ResultAndState, State,
        TransactTo, TxEnv, KECCAK_EMPTY,
    },
    ContextWithHandlerCfg, DatabaseRef, Evm,
};

use contracts::erc20_token::{BalanceOfCall, BalanceOfReturn, ERC20TokenAbi};
use contracts::simulator::{
    SimulateGetAmountsOutUniswapV2Call, SimulateGetAmountsOutUniswapV2Return, SimulatorAbiErrors,
    SIMULATORABI_DEPLOYED_BYTECODE,
};
use contracts::uniswap_v2_factory::{CreatePairCall, CreatePairReturn};
use vidger::utilities::block_on;
use vidger::{
    logger::{error, info, warn},
    types::NewBlock,
};

use crate::amm::{AmmPoolKind, AmmProtocolKind};
use crate::managers::AmmManager;
use crate::simulator::SharedInMemoryDB;
use crate::types::CryptoToken;

type RevmContext = ContextWithHandlerCfg<(), SharedInMemoryDB>;

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
    fn get_evm(revm_ctx: RevmContext) -> Evm<'static, (), SharedInMemoryDB> {
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
        let mut evm: Evm<(), SharedInMemoryDB> = Self::get_evm(revm_ctx);

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
        let mut db = SharedInMemoryDB::new(EmptyDB::new());
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
        let mut evm: Evm<'static, (), SharedInMemoryDB> = Evm::builder().with_db(db).build();

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
        for amm in amm_manager.amms() {
            ret.deploy_full_amm(&ethers_db, amm);
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

    #[inline]
    fn make_simulator_tx(&self, data: Bytes, nonce: Option<eU256>) -> TypedTransaction {
        static TX: OnceLock<TypedTransaction> = OnceLock::new();
        let tx: &TypedTransaction = TX.get_or_init(|| {
            let ret: TypedTransaction = TransactionRequest::default()
                .from(Into::<eAddress>::into(self.account.0 .0))
                .to(Into::<eAddress>::into(self.simulator_address.0 .0))
                .value(eU256::zero())
                .nonce(eU256::zero())
                .into();

            ret
        });

        let mut transaction: TypedTransaction = tx.clone();
        transaction.set_data(data.0.into());

        if let Some(nonce) = nonce {
            transaction.set_nonce(nonce);
        }

        transaction
    }

    fn spoof_token_balance(&mut self, token: &CryptoToken, balance: U256, accounts_list_to_spoof: &[Address]) {
        // https://ethereum.stackexchange.com/questions/147205/how-to-view-the-amount-of-storage-a-contract-uses
        // https://ethereum.stackexchange.com/questions/47986/using-getstorageat-on-mappingaddress-uint64
        let input_balance_slots: Option<Vec<U256>> = if token.balance_contract_slot() != -1 {
            // Inject simulator contract with token balance
            // Inject account with token balance
            let slots_to_spoof: Vec<U256> = accounts_list_to_spoof
                .iter()
                .map(|address_to_spoof: &Address| {
                    U256::from_be_bytes(keccak256(abi::encode(&[
                        abi::Token::Address(eAddress::from(address_to_spoof.0 .0)),
                        abi::Token::Uint(eU256::from(token.balance_contract_slot())),
                    ])))
                })
                .collect();
            Some(slots_to_spoof)
        } else {
            None
        };

        let db: &mut SharedInMemoryDB = &mut self.revm_ctx.context.evm.db;
        if let Some(input_balance_slots) = input_balance_slots {
            for input_balance_slot_index in input_balance_slots {
                db.insert_account_storage(token.address().0.into(), input_balance_slot_index, balance)
                    .expect("failed to insert token balance in DB");
            }
        }
    }

    fn deploy_token(&mut self, ethers_db: &Arc<RwLock<EthersDB<M>>>, token: &CryptoToken) {
        /*
        NOTES:
            - Storage are in the token contract(proxy) and the code in the implementation contract
            - Keep in mind the proxy contract have a slot for the implementation address
              that's hard to get and set for every kind of proxy
              so will just grab the implementation AccountInfo(code, code_hash, etc)
              then assign it to the original token contract to get raid of the proxy concept/contract
              now the token contract will be the implementation.
              (BIG BRAIN xD)
        */
        let token_address: Address = token.address().0.into();

        // Skip if already deployed
        let db: &SharedInMemoryDB = &self.revm_ctx.context.evm.db;
        if db.have_account(&token_address) {
            panic!("Token already deployed: {}({})", token.symbol(), token_address);
        }

        // Deploy proxy
        // TODO: Maybe add proxy_code to database
        let token_acc_info: AccountInfo = if let Some(proxy_address) = token.proxy_address() {
            ethers_db
                .read()
                .unwrap()
                .basic_ref(proxy_address.0.into())
                .unwrap()
                .unwrap()
        } else {
            let code = Bytecode::new_raw(Bytes::from(token.code().clone()));
            AccountInfo::new(U256::from(0), 0, code.hash_slow(), code)
        };

        // Commit
        let db: &mut SharedInMemoryDB = &mut self.revm_ctx.context.evm.db;
        db.insert_account_info(token_address, token_acc_info);
    }

    fn deploy_pool(&mut self, ethers_db: &Arc<RwLock<EthersDB<M>>>, pool: &AmmPoolKind) {
        /*
        NOTE:
            We can notice that it retrieves the balance of token0, token1 making calls to the token contracts.
            This means that our newly deployed pair contract has to have real token balances to perform a real swap.
        */
        let pool_address: Address = pool.address().0.into();

        // Skip if already deployed
        if self.revm_ctx.context.evm.db.have_account(&pool_address) {
            panic!("Pool already deployed: {}", pool_address);
        }

        // Deploy tokens
        // I deploy tokens here because maybe later pools have diff strategy for holding tokens
        // so I can depend on pool type to deploy tokens
        let token_address: Address = pool.token0().address().0.into();
        if !self.revm_ctx.context.evm.db.have_account(&token_address) {
            self.deploy_token(ethers_db, pool.token0());
        }

        let token_address: Address = pool.token1().address().0.into();
        if !self.revm_ctx.context.evm.db.have_account(&token_address) {
            self.deploy_token(ethers_db, pool.token1());
        }

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
                let reserve_slot_idx = U256::from(RESERVE_SLOT);
                if let Ok(slot_value) = ethers_db.read().unwrap().storage_ref(pool_address, reserve_slot_idx) {
                    slots.insert(reserve_slot_idx, slot_value);
                } else {
                    warn!("Failed to get slot '{}' from pool '{}'", reserve_slot_idx, pool_address);
                }

                // Only reserve slot needs to be updated
                self.accounts_slots_to_update
                    .insert(pool_address, vec![reserve_slot_idx]);
            }
        };

        // Get pool account info using ethers_db
        let pool_acc_info: AccountInfo = ethers_db.read().unwrap().basic_ref(pool_address).unwrap().unwrap();

        // Commit
        let db: &mut SharedInMemoryDB = &mut self.revm_ctx.context.evm.db;
        db.insert_account_info(pool_address, pool_acc_info);

        for (slot, value) in slots {
            db.insert_account_storage(pool_address, slot, value)
                .expect("failed to insert pool reserves in DB");
        }

        // Spoof tokens balances for the pool, account and simulator contract
        static HUNDRED_GRAND_ETH: OnceLock<U256> = OnceLock::new();
        let balance: &U256 = HUNDRED_GRAND_ETH.get_or_init(|| {
            U256::from(100_000)
                .checked_mul(U256::from(10).pow(U256::from(18)))
                .unwrap()
        });

        let accounts_to_spoof = [pool_address, self.account, self.simulator_address];
        self.spoof_token_balance(pool.token0(), *balance, &accounts_to_spoof);
        self.spoof_token_balance(pool.token1(), *balance, &accounts_to_spoof);
    }

    fn deploy_contracts(&mut self, ethers_db: &Arc<RwLock<EthersDB<M>>>, contracts: &[Address]) {
        // TODO: Parallelize
        let account_info_list: Vec<(Address, AccountInfo)> = contracts
            .iter()
            .map(|c: &Address| (*c, ethers_db.read().unwrap().basic_ref(*c).unwrap().unwrap()))
            .collect();

        // Commit
        let db: &mut SharedInMemoryDB = &mut self.revm_ctx.context.evm.db;
        for (acc, info) in account_info_list {
            if db.have_account(&acc) {
                panic!("Contract already deployed: {}", acc.to_checksum(None));
            }

            db.insert_account_info(acc, info);
        }
    }

    fn deploy_amm(&mut self, ethers_db: &Arc<RwLock<EthersDB<M>>>, amm: &Arc<AmmProtocolKind>) {
        // TODO: For uniswap_v3 needs quoter and router contracts
        match &**amm {
            AmmProtocolKind::UniswapV2(uniswap2) => {
                let router_address: Address = uniswap2.router().0.into();
                let factory_address: Address = uniswap2.factory().0.into();

                self.deploy_contracts(ethers_db, &[factory_address, router_address]);
            }
        };
    }

    fn deploy_full_amm(&mut self, ethers_db: &Arc<RwLock<EthersDB<M>>>, amm: &Arc<AmmProtocolKind>) {
        self.deploy_amm(ethers_db, amm);

        // Deploy pools
        // TODO: Parallelize
        for pool in amm.pools() {
            self.deploy_pool(ethers_db, pool);
        }
    }

    #[inline]
    fn node_debug_trace_call_get_state_diff(&self, tx: TypedTransaction) -> Result<GethTrace> {
        static OPTIONS: GethDebugTracingCallOptions = GethDebugTracingCallOptions {
            tracing_options: GethDebugTracingOptions {
                disable_storage: None,
                disable_stack: None,
                enable_memory: None,
                enable_return_data: None,
                tracer: Some(GethDebugTracerType::BuiltInTracer(
                    GethDebugBuiltInTracerType::PreStateTracer,
                )),
                tracer_config: None,
                timeout: None,
            },
            state_overrides: None,
            block_overrides: None,
        };

        let trace: GethTrace = block_on(self.provider.debug_trace_call(tx, None, OPTIONS.clone()))?;

        Ok(trace)
    }

    fn node_debug_trace_call(&self, tx: &eTransaction, block_number: Option<eU64>) -> Result<Option<CallFrame>> {
        static TRACE_OPTIONS: GethDebugTracingCallOptions = GethDebugTracingCallOptions {
            tracing_options: GethDebugTracingOptions {
                tracer: Some(GethDebugTracerType::BuiltInTracer(
                    GethDebugBuiltInTracerType::CallTracer,
                )),
                tracer_config: Some(GethDebugTracerConfig::BuiltInTracer(
                    GethDebugBuiltInTracerConfig::CallTracer(CallConfig {
                        with_log: Some(true), // 👈 make sure we are getting logs
                        only_top_call: Some(false),
                    }),
                )),
                disable_storage: None,
                disable_stack: None,
                enable_memory: None,
                enable_return_data: None,
                timeout: None,
            },
            state_overrides: None,
            block_overrides: None,
        };

        let trace: GethTrace = block_on(self.provider.debug_trace_call(
            tx,
            block_number.map(|block_id| eBlockId::Number(eBlockNumber::Number(block_id))),
            TRACE_OPTIONS.clone(),
        ))?;
        let GethTrace::Known(call_tracer) = trace else {
            return Ok(None);
        };
        let GethTraceFrame::CallTracer(frame) = call_tracer else {
            return Ok(None);
        };

        Ok(Some(frame))
    }

    pub fn node_get_tokens_balance_slot(&self, tokens: &[Address]) -> Result<HashMap<Address, Result<Option<i32>>>> {
        let calldata: Bytes = AbiEncode::encode(BalanceOfCall {
            who: self.account.0 .0.into(),
        })
        .into();

        let nonce_task = self
            .provider
            .get_transaction_count(Into::<eAddress>::into(self.account.0 .0), None);
        let nonce: eU256 = block_on(nonce_task).expect("failed to get nonce");

        let ret: HashMap<Address, Result<Option<i32>>> = tokens
            .par_iter()
            .map(|token: &Address| -> (Address, Result<Option<i32>>) {
                let mut tx: TypedTransaction = self.make_simulator_tx(calldata.clone(), Some(nonce));
                tx.set_to(Into::<eAddress>::into(token.0 .0));

                let geth_trace: Result<GethTrace> = self.node_debug_trace_call_get_state_diff(tx);
                let Ok(geth_trace) = geth_trace else {
                    return (*token, Err(geth_trace.unwrap_err()));
                };

                let prestate: PreStateMode = match geth_trace {
                    GethTrace::Known(GethTraceFrame::PreStateTracer(PreStateFrame::Default(prestate_mode))) => {
                        Some(prestate_mode)
                    }
                    _ => None,
                }
                .unwrap();

                println!("geth touched accounts: {:#?}", prestate.0);

                let token_acc_state: Result<&eAccountState> = prestate
                    .0
                    .get(&Into::<eAddress>::into(token.0 .0))
                    .ok_or(anyhow!("no token key"));
                let Ok(token_acc_state) = token_acc_state else {
                    return (*token, Err(token_acc_state.unwrap_err()));
                };

                let token_touched_storage: Result<&BTreeMap<eH256, eH256>> =
                    token_acc_state.storage.as_ref().ok_or(anyhow!("no storage values"));

                let Ok(token_touched_storage) = token_touched_storage else {
                    return (*token, Err(token_touched_storage.unwrap_err()));
                };

                for i in 0..400 {
                    let slot: [u8; 32] = keccak256(&abi::encode(&[
                        abi::Token::Address(self.account.0 .0.into()),
                        abi::Token::Uint(eU256::from(i)),
                    ]));

                    if token_touched_storage.get(&slot.into()).is_none() {
                        continue;
                    }

                    return (*token, Ok(Some(i)));
                }

                (*token, Ok(None))
            })
            .collect();

        Ok(ret)
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
        let db: &mut SharedInMemoryDB = &mut self.revm_ctx.context.evm.db;
        for (address, (slot, value)) in slots_values {
            db.insert_account_storage(address, slot, value)
                .expect("failed to slot storage value");
        }

        info!("Simulator updated storage values for block '{}'", new_block.number);
    }

    pub fn get_tokens_balance_slot(&self, tokens: &[eAddress]) -> Result<HashMap<eAddress, Result<Option<i32>>>> {
        let db: &SharedInMemoryDB = &self.revm_ctx.context.evm.db;

        // Check if all tokens are deployed
        for token in tokens {
            let token: Address = token.0.into();
            if !db.have_account(&token) {
                panic!("Token not deployed: {}", token.to_checksum(None));
            }
        }

        // Call balanceOf
        let calldata: Bytes = AbiEncode::encode(BalanceOfCall {
            who: self.account.0 .0.into(),
        })
        .into();

        Ok(tokens
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

                if let ExecutionResult::Revert { output, .. } = result_and_state.0 {
                    panic!("Try decode revert output: {:?}", output);
                }

                println!("State: {:#?}", result_and_state.1.clone().unwrap());

                // Get touched storage
                let token_acc: &Account = result_and_state.1.as_ref().unwrap().get(&_token).unwrap();
                let touched_storage: &Storage = &token_acc.storage;
                println!("Touched storage slots: {:?}", touched_storage);

                if touched_storage.is_empty() {
                    return (*token, Ok(None));
                }

                // Some tokens have a lot of storage slots like
                // https://polygonscan.com/token/0x9C9e5fD8bbc25984B178FdCE6117Defa39d2db39
                // balance slot are 51
                let balance_slot: Option<i32> = (0..400).into_par_iter().find_first(|i: &i32| {
                    let slot: [u8; 32] = keccak256(abi::encode(&[
                        abi::Token::Address(self.account.0 .0.into()),
                        abi::Token::Uint(eU256::from(*i)),
                    ]));

                    let slot = U256::from_be_bytes(slot);
                    if touched_storage.get(&slot).is_none() {
                        return false; // continue
                    };

                    println!("Balance storage slot: {:?} ({:?})", i, slot);
                    true
                });

                if balance_slot.is_some() {
                    return (*token, Ok(balance_slot));
                }

                (*token, Ok(None))
            })
            .collect())
    }

    pub fn get_token_balance(&self, token: &Address) -> Result<eU256> {
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

                //Ok(U256::from_limbs(decoded_output.0.0))
                Ok(decoded_output.0)
            }
            TxResult::Revert(_) => Err(anyhow!("Failed to get token balance")),
            TxResult::Halt(_) => Err(anyhow!("Failed to get token balance")),
        }
    }

    pub fn get_amounts_out(
        &self,
        pool: &AmmPoolKind,
        input: &CryptoToken,
        amount_in: eU256,
    ) -> Result<eU256, SimulatorAbiErrors> {
        // TODO: For uniswap_v3 `contract_address` are the quarter
        let calldata: Bytes = match pool {
            AmmPoolKind::UniswapV2(_) => {
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
            Err(e) => panic!("EVM call failed: {:?}", e),
        };

        let tx_result: TxResult = Self::get_tx_result(result_and_state.0);
        match tx_result {
            TxResult::Success(result) => match pool {
                AmmPoolKind::UniswapV2(_) => {
                    let decode1 = SimulateGetAmountsOutUniswapV2Return::decode(&result.output);
                    let Ok(decoded_output) = decode1 else {
                        return Err(SimulatorAbiErrors::decode(&result.output).unwrap());
                    };

                    Ok(decoded_output.0)
                }
            },
            TxResult::Revert(revert) => Err(revert.output),
            TxResult::Halt(halt) => panic!("Failed to get token balance HALT: {:?}", halt.reason),
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use std::path::Path;
    use std::sync::OnceLock;

    use ethers::providers::{Http, Provider};

    use contracts::erc20_token::TransferCall;
    use contracts::simulator::MultiSwapError;
    use vidger::types::NetworkKind;

    use crate::amm::{UniswapV2Pool, UniswapV2Protocol};
    use crate::database::{Database, DbToken, DbTokenNetwork};
    use crate::managers::TokenManager;

    use super::*;

    fn get_db() -> Database {
        Database::new(Path::new("H:\\Projects\\mev-chaser\\Main.db")).unwrap()
    }

    fn get_provider() -> Arc<Provider<Http>> {
        Arc::new(
            Provider::<Http>::try_from(
                "https://polygon.blockpi.network/v1/rpc/03d6815a3ad15c13cc9fa5e00f7649f72ee3ad4f",
            )
            .unwrap(),
        )
    }

    fn get_amm_manager(db: &Database) -> AmmManager {
        let token_manager = TokenManager::new_by_db(db, &NetworkKind::Polygon).unwrap();
        AmmManager::new_by_db(db, &NetworkKind::Polygon, &token_manager).unwrap()
    }

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

    fn get_simulator<M: Middleware + 'static>(provider: &Arc<M>, amm_manager: &AmmManager) -> EvmSimulator<M> {
        EvmSimulator::new(Arc::clone(provider), amm_manager).unwrap()
    }

    fn make_token(db: &Database, address: &str) -> CryptoToken {
        let (db_token, db_token_network): (DbToken, DbTokenNetwork) = db
            .get_token_and_network(address, &NetworkKind::Polygon)
            .unwrap()
            .unwrap();
        CryptoToken::new(
            db_token_network.address,
            db_token_network.proxy,
            db_token.name,
            db_token.symbol,
            db_token.decimals as u8,
            db_token_network.balance_contract_slot,
            db_token_network.code,
        )
        .unwrap()
    }

    fn weth_token(db: &Database) -> &'static CryptoToken {
        static TOKEN: OnceLock<CryptoToken> = OnceLock::new();
        TOKEN.get_or_init(|| make_token(db, "0x7ceB23fD6bC0adD59E62ac25578270cFf1b9f619"))
    }

    fn wmatic_token(db: &Database) -> &'static CryptoToken {
        static TOKEN: OnceLock<CryptoToken> = OnceLock::new();
        TOKEN.get_or_init(|| make_token(db, "0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270"))
    }

    fn quick_token(db: &Database) -> &'static CryptoToken {
        static TOKEN: OnceLock<CryptoToken> = OnceLock::new();
        TOKEN.get_or_init(|| make_token(db, "0xB5C064F955D8e7F38fE0460C556a72987494eE17"))
    }

    fn usdc_token(db: &Database) -> &'static CryptoToken {
        static TOKEN: OnceLock<CryptoToken> = OnceLock::new();
        TOKEN.get_or_init(|| make_token(db, "0x3c499c542cEF5E3811e1192ce70d8cC03d5c3359"))
    }

    fn usdt_token(db: &Database) -> &'static CryptoToken {
        static TOKEN: OnceLock<CryptoToken> = OnceLock::new();
        TOKEN.get_or_init(|| make_token(db, "0xc2132D05D31c914a87C6611C10748AEb04B58e8F"))
    }

    fn sushi_token(db: &Database) -> &'static CryptoToken {
        static TOKEN: OnceLock<CryptoToken> = OnceLock::new();
        TOKEN.get_or_init(|| make_token(db, "0x0b3F868E0BE5597D5DB7fEB59E1CADBb0fdDa50a"))
    }

    fn wbtc_token(db: &Database) -> &'static CryptoToken {
        static TOKEN: OnceLock<CryptoToken> = OnceLock::new();
        TOKEN.get_or_init(|| make_token(db, "0x1BFD67037B42Cf73acF2047067bd4F2C47D9BfD6"))
    }

    fn balance_of_tokens<M: Middleware + 'static>(provider: &Arc<M>, tokens: &[&CryptoToken]) {
        let mut simulator: EvmSimulator<M> = get_simulator(provider, &AmmManager::new(vec![]));
        let ethers_db = Arc::new(RwLock::new(EthersDB::new(Arc::clone(provider), None).unwrap()));

        let hundred_grand_eth: U256 = U256::from(100_000)
            .checked_mul(U256::from(10).pow(U256::from(18)))
            .unwrap();
        let e_hundred_grand_eth = eU256::from(hundred_grand_eth.clone().to_be_bytes());

        let accounts_to_spoof = [simulator.simulator_address, simulator.account];
        for token in tokens {
            simulator.deploy_token(&ethers_db, token);
            simulator.spoof_token_balance(token, hundred_grand_eth, &accounts_to_spoof);

            let balance: Result<eU256> = simulator.get_token_balance(&token.address().0.into());
            assert!(balance.is_ok(), "{} send_tx failed", token.symbol());

            let balance: eU256 = balance.unwrap();
            assert_eq!(
                balance,
                e_hundred_grand_eth,
                "{} balance: {:?}",
                token.symbol(),
                balance
            );

            println!("{} balance: {:?}", token.symbol(), balance);
        }
    }

    fn transfer_tokens(provider: &Arc<Provider<Http>>, tokens: &[&CryptoToken]) {
        let mut simulator: EvmSimulator<Provider<Http>> = get_simulator(provider, &AmmManager::new(vec![]));
        let ethers_db = Arc::new(RwLock::new(EthersDB::new(Arc::clone(provider), None).unwrap()));

        let hundred_grand_eth: U256 = U256::from(100_000)
            .checked_mul(U256::from(10).pow(U256::from(18)))
            .unwrap();
        //let e_hundred_grand_eth = eU256::from(hundred_grand_eth.clone().to_be_bytes());

        let accounts_to_spoof = [simulator.simulator_address, simulator.account];
        for token in tokens {
            simulator.deploy_token(&ethers_db, token);
            simulator.spoof_token_balance(token, hundred_grand_eth, &accounts_to_spoof);

            let calldata: Bytes = AbiEncode::encode(TransferCall {
                to: simulator.simulator_address.0 .0.into(),
                value: token.convert_to_amount(1.0_f64),
            })
            .into();

            let result_and_state: (ExecutionResult, Option<State>) = EvmSimulator::<Provider<Http>>::send_tx(
                simulator.revm_ctx.clone(),
                simulator.account,
                token.address().0.into(),
                calldata,
                false,
            )
            .unwrap();
            assert!(result_and_state.0.is_success(), "{} send_tx failed", token.symbol());

            let data: &Bytes = result_and_state.0.output().unwrap();
            assert_ne!(data.len(), 0, "{} data: {:?}", token.symbol(), data);

            println!("{} data: {:?}", token.symbol(), data);
        }
    }

    fn get_amounts_out<M: Middleware + 'static>(simulator: &EvmSimulator<M>, amm_manager: &AmmManager) {
        for amm in amm_manager.amms() {
            for pool in amm.pools() {
                println!(
                    "pool: {}, {} -> {}",
                    to_checksum(pool.address(), None),
                    pool.token0().symbol(),
                    pool.token1().symbol()
                );

                let token: &Arc<CryptoToken> = pool.token0();
                let amount: eU256 = pool.token0().convert_to_amount(1.0_f64);
                let result: Result<eU256, SimulatorAbiErrors> = simulator.get_amounts_out(pool, token, amount);

                if let Err(e) = &result {
                    let multi_swap_error: &MultiSwapError = match e {
                        SimulatorAbiErrors::MultiSwapError(e) => e,
                        _ => panic!("multi_swap_error: {}", e),
                    };

                    if multi_swap_error.error_reason.contains("UniswapV2: K") {
                        println!("UniswapV2: K");
                        println!("=====================");
                        continue;
                    }
                }

                let result: eU256 = result.unwrap();

                assert_ne!(result, eU256::zero());
                assert!(result > eU256::zero());

                println!(
                    "{} -> {} = {}",
                    pool.token0().symbol(),
                    pool.token1().symbol(),
                    pool.token1().convert_to_decimal(result)
                );
                println!("=====================");
            }
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn balance_of_no_proxy_tokens() {
        let provider: Arc<Provider<Http>> = get_provider();
        let db: Database = get_db();

        let tokens = [wmatic_token(&db), weth_token(&db), quick_token(&db)];
        balance_of_tokens(&provider, &tokens);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn balance_of_proxy_tokens() {
        let provider: Arc<Provider<Http>> = get_provider();
        let db: Database = get_db();

        let tokens = [usdc_token(&db), usdt_token(&db), sushi_token(&db), wbtc_token(&db)];
        balance_of_tokens(&provider, &tokens);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn transfer_no_proxy_tokens() {
        let provider: Arc<Provider<Http>> = get_provider();
        let db: Database = get_db();

        let tokens = [wmatic_token(&db), weth_token(&db), quick_token(&db)];
        transfer_tokens(&provider, &tokens);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn transfer_proxy_tokens() {
        let provider: Arc<Provider<Http>> = get_provider();
        let db: Database = get_db();

        let tokens = [usdc_token(&db), usdt_token(&db)];
        transfer_tokens(&provider, &tokens);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn get_amounts_out_tokens() {
        let provider: Arc<Provider<Http>> = get_provider();
        let db: Database = get_db();
        let amm_manager: AmmManager = get_amm_manager(&db);
        let simulator: EvmSimulator<Provider<Http>> = get_simulator(&provider, &amm_manager);

        get_amounts_out(&simulator, &amm_manager);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn get_tokens_balance_slot_proxy_tokens() {
        let db: Database = get_db();
        let token0: &CryptoToken = wmatic_token(&db);
        let token1: CryptoToken = make_token(&db, "0x6396252377F54ad33cFF9131708Da075b21d9B88");

        let amm_manager: AmmManager = get_uniswap_v2_amm_manager(
            "QuickSwapV2",
            "0x5757371414417b8C6CAad45bAeF941aBc7d3Ab32",
            "0xa5E0829CaCEd8fFDD4De3c43696c57F7D7A678ff",
            "0x9646f7CFbeCE44b94825F3AAEc88D591941b8dC4",
            token0,
            &token1,
        );

        let provider: Arc<Provider<Http>> = get_provider();
        let simulator: EvmSimulator<Provider<Http>> = get_simulator(&provider, &amm_manager);
        let token_address: Address = token0.address().0.into();

        //for i in 0..400 {
        //    let slot = U256::from(i);
        //    let uint = simulator.get_storage_at(token_address, slot);
        //    println!("slot {}: {:?}", i, uint);
        //}

        //let t: Address = token1.address().0.into();
        //let result = simulator.get_token_balance(&t);
        //println!("Token balance: {:?}", result);

        let x = simulator
            .node_get_tokens_balance_slot(&[token1.address().0.into()])
            .unwrap();
        println!("{:?}", x);

        let slot: HashMap<eAddress, Result<Option<i32>>> =
            simulator.get_tokens_balance_slot(&[*token1.address()]).unwrap();
        let slot: &Result<Option<i32>> = slot.get(token1.address()).unwrap();
        assert!(
            slot.is_ok() && slot.as_ref().unwrap().is_some(),
            "Failed to get token balance slot"
        );

        let slot: i32 = slot.as_ref().unwrap().unwrap();
        assert!(slot >= 0, "Failed to get token balance slot");

        println!("Balance slot: {}", slot);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn get_tokens_balance_slot_tokens() {
        let provider: Arc<Provider<Http>> = get_provider();
        let db: Database = get_db();
        let amm_manager: AmmManager = get_amm_manager(&db);
        let simulator: EvmSimulator<Provider<Http>> = get_simulator(&provider, &amm_manager);

        let result: Result<HashMap<eAddress, Result<Option<i32>>>> = simulator.get_tokens_balance_slot(&[]);
        assert!(result.is_ok(), "Failed to get tokens balance slot");

        let result: HashMap<eAddress, Result<Option<i32>>> = result.unwrap();
        assert_ne!(result.len(), 0, "Failed to get tokens balance slot");

        for (address, result) in result {
            assert!(
                result.is_ok() && result.as_ref().unwrap().is_some(),
                "Failed to get token '{}' balance slot",
                to_checksum(&address, None)
            );

            assert!(
                result.unwrap().unwrap() >= 0,
                "Failed to get token '{}' balance slot",
                to_checksum(&address, None)
            );
        }
    }
}
