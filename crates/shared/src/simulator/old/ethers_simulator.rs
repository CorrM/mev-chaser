/*use std::collections::BTreeMap;
use std::str::FromStr;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use ethers::abi::Token;
use ethers::{
    abi,
    abi::{AbiDecode, AbiEncode},
    contract::Multicall,
    providers::{Middleware, ProviderError, RawCall, RpcError},
    types::spoof::State,
    types::transaction::eip2718::TypedTransaction,
    types::{
        spoof, AccountState, BlockId, Bytes, GethDebugBuiltInTracerType, GethDebugTracerType,
        GethDebugTracingCallOptions, GethDebugTracingOptions, GethTrace, GethTraceFrame, PreStateFrame, PreStateMode,
        TransactionRequest, U64,
    },
    types::{Address, BlockNumber, H256, U256},
    types::{CallConfig, CallFrame, CallLogFrame, GethDebugBuiltInTracerConfig, GethDebugTracerConfig, Transaction},
    utils::__serde_json::Value,
    utils::keccak256,
};
use hashbrown::HashMap;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use revm::primitives::bitvec::macros::internal::funty::Fundamental;

use contracts::balancer_flash_loan_recipient::OneSwapInfo;
use contracts::erc20_token::BalanceOfCall;
use contracts::simulator::{
    SimulateGetAmountsOutCall, SimulateGetAmountsOutReturn, SimulateMultiSwapCall, SimulateMultiSwapReturn,
    SimulatorAbi, SimulatorAbiErrors, SIMULATORABI_DEPLOYED_BYTECODE,
};
use vidger::types::CryptoToken;
use vidger::utilities::block_on;

use crate::amm::{AmmPoolKind, AmmProtocolKind};

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
    account: Address,
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
    pub(super) fn new(provider: Arc<M>, tokens_to_override_balance: &[CryptoToken]) -> Self {
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
                Token::Address(simulator_address),
                Token::Uint(U256::from(crypto_token.balance_contract_slot())),
            ]));

            state_override_set
                .account(account)
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
            account,
            state_override_set,
            simulator_address,
            simulator_abi,
            simulator_tx,
            multicall,
            provider,
        }
    }

    pub fn provider(&self) -> &Arc<M> {
        &self.provider
    }
}

impl<M> EthersSimulator<M>
where
    M: Middleware + 'static,
{
    #[inline]
    fn make_simulator_tx(&self, data: impl Into<Bytes>, nonce: Option<U256>) -> TypedTransaction {
        let mut transaction: TypedTransaction = self.simulator_tx.clone();
        transaction.set_data(data.into());

        if let Some(nonce) = nonce {
            transaction.set_nonce(nonce);
        }

        transaction
    }

    #[inline]
    fn debug_trace_call_get_state_diff(&self, tx: TypedTransaction, block_number: U64) -> Result<GethTrace> {
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

    pub fn debug_trace_call(&self, tx: &Transaction, block_number: Option<U64>) -> Result<Option<CallFrame>> {
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

    pub fn get_tokens_balance_slot(
        &self,
        tokens: &[Address],
        block_number: U64,
    ) -> Result<HashMap<Address, Result<Option<i32>>>> {
        let calldata: Bytes = AbiEncode::encode(BalanceOfCall { who: self.account }).into();

        let nonce_task = self
            .provider
            .get_transaction_count(self.account, Some(block_number.into()));
        let nonce: U256 = block_on(nonce_task).expect("failed to get nonce");

        let ret: HashMap<Address, Result<Option<i32>>> = tokens
            .par_iter()
            .map(|token| -> (Address, Result<Option<i32>>) {
                let mut tx: TypedTransaction = self.make_simulator_tx(calldata.clone(), Some(nonce));
                tx.set_to(*token);

                let geth_trace: Result<GethTrace> = block_on(self.debug_trace_call_get_state_diff(tx, block_number));

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

                //let geth_touched_accs = prestate.0.keys();
                //println!("geth touched accounts: {:?}", geth_touched_accs);

                let token_acc_state: Result<&AccountState> = prestate.0.get(token).ok_or(anyhow!("no token key"));
                let Ok(token_acc_state) = token_acc_state else {
                    return (*token, Err(token_acc_state.unwrap_err()));
                };

                let token_touched_storage: Result<&BTreeMap<H256, H256>> =
                    token_acc_state.storage.as_ref().ok_or(anyhow!("no storage values"));

                let Ok(token_touched_storage) = token_touched_storage else {
                    return (*token, Err(token_touched_storage.unwrap_err()));
                };

                for i in 0..20 {
                    let slot: [u8; 32] = keccak256(&abi::encode(&[
                        abi::Token::Address(self.account),
                        abi::Token::Uint(U256::from(i)),
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

    pub fn get_amounts_out(&self, pool: &AmmPoolKind, amount_in: U256) -> Result<U256> {
        let protocol: u8 = match &**pool.dex() {
            AmmProtocolKind::UniswapV2(_) => 0,
        };

        let path: Bytes = abi::encode(&[Token::FixedArray(vec![
            Token::Address(*pool.token0().address()),
            Token::Address(*pool.token1().address()),
        ])])
        .into();
        let calldata: Vec<u8> = AbiEncode::encode(SimulateGetAmountsOutCall {
            protocol,
            contract_address: *pool.dex().router(),
            path,
            amount_in,
        });
        let tx: TypedTransaction = self.make_simulator_tx(calldata, None);
        let result = block_on(self.provider.provider().call_raw(&tx).state(&self.state_override_set));

        let result: Bytes = result.unwrap();
        let out: U256 = SimulateGetAmountsOutReturn::decode(result)?.0;
        Ok(out)
    }

    pub fn multi_swap(&self, block_number: U64, swaps: Vec<OneSwapInfo>, chain_swaps: bool) -> Result<U256> {
        let calldata: Vec<u8>;
        unsafe {
            let swaps: Vec<contracts::simulator::OneSwapInfo> =
                (&swaps as *const _ as *const Vec<contracts::simulator::simulator_abi::OneSwapInfo>).read();

            calldata = AbiEncode::encode(SimulateMultiSwapCall { swaps, chain_swaps });
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

    pub fn multicall_multi_swap(
        &self,
        block_number: U64,
        swaps: Vec<OneSwapInfo>,
        chain_swaps: bool,
    ) -> Result<U256> {
        // https://geth.ethereum.org/docs/developers/evm-tracing/built-in-tracers
        //panic!("TODO: Not implemented");
        let multicall: Multicall<M> = self
            .multicall
            .clone()
            .block(BlockNumber::Number(block_number))
            .state(self.state_override_set.clone());

        let _swaps: Vec<contracts::simulator::OneSwapInfo>;
        unsafe {
            _swaps = (&swaps as *const _ as *const Vec<contracts::simulator::simulator_abi::OneSwapInfo>).read();
        }

        // swaps already consumed, but rust will drop it when it goes out of scope, so we need to forget it
        std::mem::forget(swaps);

        let tokens_cnt: f32 = 1_f32;
        let batch: f32 = (tokens_cnt / 250_f32).ceil();
        let tokens_per_batch: usize = (tokens_cnt / batch).ceil() as usize;
        let tokens_cnt: usize = tokens_cnt as usize;
        let batch: usize = batch as usize;

        // TODO: Test which faster rayon or async_scoped
        (0..batch).into_par_iter().for_each(|i| {
            let start_idx: usize = i * tokens_per_batch;
            let end_idx: usize = std::cmp::min(start_idx + tokens_per_batch, tokens_cnt);

            let mut multicall: Multicall<M> = multicall.clone();
            for _idx in start_idx..end_idx {
                multicall.add_call(
                    self.simulator_abi.simulate_multi_swap(_swaps.clone(), chain_swaps),
                    true,
                );
            }

            let vec = block_on(multicall.call_raw()).expect("failed to multicall");
            /*
            for x in vec {
                if x.is_err() {
                    let error = x.unwrap_err();
                    let option = SimulatorAbiErrors::decode_with_selector(&error);

                    println!("multicall error: {:?}", option);
                }
            }
            */
        });

        //let batches: Vec<usize> = (0..batch).collect();
        /*
        async_scoped::TokioScope::scope_and_block(|s| {
            for i in &batches {
                s.spawn(async {
                    let start_idx: usize = i.clone() * tokens_per_batch;
                    let end_idx: usize = std::cmp::min(start_idx + tokens_per_batch, tokens_cnt);

                    let mut multicall: Multicall<M> = multicall.clone();
                    for _idx in start_idx..end_idx {
                        multicall.add_call(
                            self.simulator_abi.simulate_multi_swap(_swaps.clone(), chain_swaps),
                            true,
                        );
                    }

                    let vec = multicall.call_raw().await.expect("failed to multicall");
                    /*
                    for x in vec {
                        if x.is_err() {
                            let error = x.unwrap_err();
                            let option = SimulatorAbiErrors::decode_with_selector(&error);

                            println!("multicall error: {:?}", option);
                        }
                    }
                    */
                });
            }
        });
        */

        Ok(50.into())
    }
}
*/
