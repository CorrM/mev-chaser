use std::collections::btree_map::Keys;
use std::collections::BTreeMap;
use std::str::FromStr;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use ethers::contract::{ContractRevert, Multicall};
use ethers::types::{
    CallConfig, CallFrame, CallLogFrame, GethDebugBuiltInTracerConfig, GethDebugTracerConfig, Transaction,
};
use ethers::{
    abi,
    abi::{AbiDecode, AbiEncode},
    providers::{Middleware, ProviderError, RawCall, RpcError},
    types::spoof::State,
    types::transaction::eip2718::TypedTransaction,
    types::{
        spoof, AccountState, BigEndianHash, BlockId, Bytes, GethDebugBuiltInTracerType, GethDebugTracerType,
        GethDebugTracingCallOptions, GethDebugTracingOptions, GethTrace, GethTraceFrame, PreStateFrame, PreStateMode,
        TransactionRequest, TxHash, U64,
    },
    types::{Address, Block, BlockNumber, H256, U256},
    utils::__serde_json::Value,
    utils::keccak256,
};
use tokio::time::Instant;

use contracts::balancer_flash_loan_recipient::OneSwapInfo;
use contracts::erc20_token::{BalanceOfCall, BalanceOfReturn};
use contracts::simulator::{
    SimulateMultiSwapCall, SimulateMultiSwapReturn, SimulatorAbi, SimulatorAbiErrors, SIMULATORABI_DEPLOYED_BYTECODE,
};
use vidger::types::CryptoToken;

fn extract_trace_logs(call_frame: &CallFrame, logs: &mut Vec<CallLogFrame>) {
    if let Some(ref logs_vec) = call_frame.logs {
        logs.extend(logs_vec.iter().cloned());
    }

    if let Some(ref calls_vec) = call_frame.calls {
        for call in calls_vec {
            extract_trace_logs(call, logs);
        }
    }
}

pub struct EthersSimulator<M> {
    state_override_set: State,
    simulator_address: Address,
    simulator_abi: SimulatorAbi<M>,
    /// Don't use this field, use [make_simulator_tx](EthersSimulator::make_simulator_tx) instead
    simulator_tx: TypedTransaction,
    multicall: Multicall<M>,
    provider: Arc<M>,
}

impl<M> EthersSimulator<M>
where
    M: Middleware + 'static,
{
    pub async fn new(provider: Arc<M>, tokens_to_override_balance: &[CryptoToken]) -> Self {
        let ten_eth: U256 = U256::from(10).checked_mul(U256::from(10).pow(U256::from(18))).unwrap();
        let gas_price: U256 = U256::from(100).checked_mul(U256::from(10).pow(U256::from(9))).unwrap();

        let mut state_override_set: State = spoof::state();
        let account = Address::from_str("0x9cf277A22EB4c551c6E18F7a6C0ee1893bcB034f").unwrap();

        // Spoof user balance with 10 ETH (for gas fees)
        state_override_set.account(account).balance(ten_eth).nonce(0.into());

        // Create Simulator contract with bytecode injection
        let simulator_address = Address::from_str("0xF2d01Ee818509a9540d8324a5bA52329af27D19E").unwrap();
        state_override_set
            .account(simulator_address)
            .code(SIMULATORABI_DEPLOYED_BYTECODE.clone());

        // Spoof tokens balance for the user
        for crypto_token in tokens_to_override_balance {
            let input_balance_slot: [u8; 32] = keccak256(abi::encode(&[
                abi::Token::Address(simulator_address),
                abi::Token::Uint(U256::from(crypto_token.balance_contract_slot())),
            ]));

            state_override_set
                .account(*crypto_token.address())
                .store(input_balance_slot.into(), H256::from_low_u64_be(ten_eth.as_u64()));
        }

        // Create ABI
        let simulator_abi = SimulatorAbi::new(simulator_address, Arc::clone(&provider));
        
        // Create transaction
        let chain: U64 = provider.get_chainid().await.unwrap().as_u64().into();
        let simulator_tx: TypedTransaction = TransactionRequest::default()
            .from(account)
            .to(simulator_address)
            .value(U256::zero())
            .nonce(U256::zero())
            .gas(5_000_000)
            .gas_price(gas_price)
            .chain_id(chain)
            .into();
        
        // Create Multicall
        let multicall: Multicall<M> = Multicall::new(Arc::clone(&provider), None)
            .await
            .unwrap()
            .state(state_override_set.clone());

        Self {
            state_override_set,
            simulator_address,
            simulator_abi,
            simulator_tx,
            multicall,
            provider,
        }
    }

    #[inline]
    fn make_simulator_tx(&self, data: impl Into<Bytes>, nonce: Option<U256>) -> TypedTransaction {
        let mut transaction: TypedTransaction = self.simulator_tx.clone();
        transaction.set_data(data.into());

        if let Some(nonce) = nonce {
            transaction.set_nonce(nonce);
        }

        transaction
    }

    async fn get_state_diff(&self, tx: TypedTransaction, block_number: U64) -> Result<GethTrace> {
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

        let trace: GethTrace = self
            .provider
            .debug_trace_call(tx, Some(block_number.into()), OPTIONS.clone())
            .await?;

        Ok(trace)
    }

    pub async fn debug_trace_call(&self, tx: &Transaction, block_number: Option<U64>) -> Result<Option<CallFrame>> {
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

        let trace: GethTrace = self
            .provider
            .debug_trace_call(
                tx,
                block_number.map(|block_id| BlockId::Number(BlockNumber::Number(block_id))),
                TRACE_OPTIONS.clone(),
            )
            .await?;
        let GethTrace::Known(call_tracer) = trace else {
            return Ok(None);
        };
        let GethTraceFrame::CallTracer(frame) = call_tracer else {
            return Ok(None);
        };

        Ok(Some(frame))
    }

    pub async fn get_proxy_implementation(&self, token: Address, block_number: U64) -> Result<Option<Address>> {
        // adapted from: https://github.com/gnosis/evm-proxy-detection/blob/main/src/index.ts
        let eip_1967_logic_slot: U256 =
            U256::from("0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc");
        let eip_1967_beacon_slot: U256 =
            U256::from("0xa3f0ad74e5423aebfd80d3ef4346578335a9a72aeaee59ff6cb3582b35133d50");
        let open_zeppelin_implementation_slot: U256 =
            U256::from("0x7050c9e0f4ca769c69bd3a8ef740bc37934f8e2c036e5a723fd8ee048ed3f8c3");
        let eip_1822_logic_slot: U256 =
            U256::from("0xc5f16f0fcc639fa48a6947836d9850f504798523bf8c9a3a87d5876cf622bcf7");

        let implementation_slots: Vec<U256> = vec![
            eip_1967_logic_slot,
            eip_1967_beacon_slot,
            open_zeppelin_implementation_slot,
            eip_1822_logic_slot,
        ];

        let slots = async_scoped::TokioScope::scope_and_block(|s| {
            for slot in &implementation_slots {
                s.spawn(async {
                    self.provider
                        .get_storage_at(token, TxHash::from_uint(slot), Some(block_number.into()))
                        .await
                });
            }
        })
        .1;

        for slot in slots {
            let out: TxHash = slot??;
            let implementation = Address::from(out);
            if implementation != Address::zero() {
                return Ok(Some(implementation));
            }
        }

        Ok(None)
    }

    pub async fn get_token_balance_slot(&self, token: Address, account: Address) -> Result<i32> {
        let calldata: Bytes = AbiEncode::encode(BalanceOfCall { who: account }).into();

        let block_task = self.provider.get_block(BlockNumber::Latest);
        let nonce_task = self
            .provider
            .get_transaction_count(account, Some(BlockId::Number(BlockNumber::Latest)));

        //let provider_c = provider.clone();
        //tokio::spawn(async move {
        //    provider_c.get_block(BlockNumber::Latest);
        //});

        let (block, nonce) = tokio::join!(block_task, nonce_task);
        let block: Block<H256> = block?.ok_or(anyhow!("failed to retrieve block"))?;
        let nonce: U256 = nonce?;

        let tx: TypedTransaction = self.make_simulator_tx(calldata, Some(nonce));
        let geth_trace: GethTrace = self.get_state_diff(tx, block.number.unwrap()).await?;
        let prestate: PreStateMode = match geth_trace {
            GethTrace::Known(GethTraceFrame::PreStateTracer(PreStateFrame::Default(prestate_mode))) => {
                Some(prestate_mode)
            }
            _ => None,
        }
        .unwrap();

        let geth_touched_accs: Keys<Address, AccountState> = prestate.0.keys();
        println!("Geth trace: {:?}", geth_touched_accs);

        let token_acc_state: &AccountState = prestate.0.get(&token).ok_or(anyhow!("no token key"))?;
        let token_touched_storage: &BTreeMap<H256, H256> =
            token_acc_state.storage.as_ref().ok_or(anyhow!("no storage values"))?;

        for i in 0..20 {
            let slot: [u8; 32] = keccak256(&abi::encode(&[
                abi::Token::Address(account),
                abi::Token::Uint(U256::from(i)),
            ]));

            if token_touched_storage.get(&slot.into()).is_none() {
                continue;
            }

            println!(
                "Balance storage slot: {:?} ({:?})",
                i,
                BalanceOfReturn::decode(slot).unwrap().0
            );
            return Ok(i);
        }

        Ok(0)
    }

    pub async fn multi_swap(&self, block_number: U64, swaps: Vec<OneSwapInfo>, chain_swaps: bool) -> Result<U256> {
        let calldata: Vec<u8>;
        unsafe {
            let swaps: Vec<contracts::simulator::OneSwapInfo> =
                (&swaps as *const _ as *const Vec<contracts::simulator::simulator_abi::OneSwapInfo>).read();

            calldata = SimulateMultiSwapCall { swaps, chain_swaps }.encode();
        }

        // swaps already consumed in `calldata.encode()`, but rust will drop it when it goes out of scope,
        // so we need to forget it
        std::mem::forget(swaps);

        let tx: TypedTransaction = self.make_simulator_tx(calldata, None);

        let mut result: Option<Bytes> = None;
        for _i in 0..4 {
            let _result: Result<Bytes, ProviderError> = self
                .provider
                .provider()
                .call_raw(&tx)
                .state(&self.state_override_set)
                .block(block_number.into())
                .await;

            if _result.is_ok() {
                result = Some(_result.unwrap());
                break;
            }

            // https://github.com/ledgerwatch/erigon/issues/7548
            let error: ProviderError = _result.unwrap_err();
            if error.to_string().contains("hex number with leading zero digits") {
                println!("Ignoring hex number with leading zero digits error");
                continue;
            }

            println!("new error: {:?}", error);

            let value: &Value = RpcError::as_error_response(&error).unwrap().data.as_ref().unwrap();
            let error: SimulatorAbiErrors = SimulatorAbiErrors::decode_hex(value.as_str().unwrap()).unwrap();

            return Err(anyhow!("failed to call eth_call: {:?}", error));
        }

        let result: Bytes = result.unwrap();
        let out: U256 = SimulateMultiSwapReturn::decode(result)?.0;

        Ok(out)
    }

    pub async fn multicall_multi_swap(
        &self,
        block_number: U64,
        swaps: Vec<OneSwapInfo>,
        chain_swaps: bool,
    ) -> Result<U256> {
        let mut multicall: Multicall<M> = self.multicall.clone().block(BlockNumber::Number(block_number));

        let _swaps: Vec<contracts::simulator::OneSwapInfo>;
        unsafe {
            _swaps = (&swaps as *const _ as *const Vec<contracts::simulator::simulator_abi::OneSwapInfo>).read();
        }

        // swaps already consumed, but rust will drop it when it goes out of scope, so we need to forget it
        std::mem::forget(swaps);

        for _i in 0..250 {
            multicall.add_call(self.simulator_abi.simulate_multi_swap(_swaps.clone(), chain_swaps), true);
        }

        let start = Instant::now();
        let vec = multicall.call_raw().await.expect("failed to multicall");
        for x in vec {
            if x.is_err() {
                let error = x.unwrap_err();
                let option = SimulatorAbiErrors::decode_with_selector(&error);
                
                println!("multicall error: {:?}", option);
            }
        }
        println!("duration x: {}ms", start.elapsed().as_millis());
        
        Ok(50.into())
    }
}
