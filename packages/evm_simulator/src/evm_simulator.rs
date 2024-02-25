use std::collections::btree_map::Keys;
use std::collections::BTreeMap;
use std::str::FromStr;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use ethers::{
    abi::{self, AbiEncode},
    types::{Address, Block, BlockNumber, H256, U256},
    utils::keccak256,
};
use ethers_core::abi::AbiDecode;
use ethers_core::types::spoof::State;
use ethers_core::types::transaction::eip2930::AccessList;
use ethers_core::types::{
    spoof, AccountState, BigEndianHash, BlockId, Bytes, Eip1559TransactionRequest, GethDebugBuiltInTracerType,
    GethDebugTracerType, GethDebugTracingCallOptions, GethDebugTracingOptions, GethTrace, GethTraceFrame,
    NameOrAddress, PreStateFrame, PreStateMode, TransactionRequest, TxHash, U64,
};
use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_providers::{Middleware, RawCall};

use contracts::erc20_token::{BalanceOfCall, BalanceOfReturn};
use contracts::simulator::{SimulateMultiSwapCall, SimulateMultiSwapReturn, SIMULATORABI_BYTECODE};

static TEN_ETH: U256 = U256::from(10).checked_mul(U256::from(10).pow(U256::from(18))).unwrap();
static GAS_PRICE: U256 = U256::from(100).checked_mul(U256::from(10).pow(U256::from(9))).unwrap();

pub struct EvmSimulator<M>
where
    M: Middleware + 'static,
{
    chain: U64,
    account: Address,
    state_override_set: State,
    simulator_address: Address,
    provider: Arc<M>,
}

impl<M> EvmSimulator<M>
where
    M: Middleware + 'static,
{
    pub async fn new(provider: Arc<M>) -> Self {
        let chain: U64 = provider.get_chainid().await.unwrap().as_u64().into();

        let mut state_override_set: State = spoof::state();
        let account = Address::from_str("0x9cf277A22EB4c551c6E18F7a6C0ee1893bcB034f").unwrap();
        
        // Spoof user balance with 10 ETH (for gas fees)
        state_override_set.account(account).balance(TEN_ETH).nonce(0.into());

        // Create Simulator contract with bytecode injection
        let simulator_address = Address::from_str("0xF2d01Ee818509a9540d8324a5bA52329af27D19E").unwrap();
        state_override_set
            .account(simulator_address)
            .code(SIMULATORABI_BYTECODE.clone());

        Self {
            chain,
            account,
            state_override_set,
            simulator_address,
            provider,
        }
    }

    async fn get_state_diff(&self, tx: Eip1559TransactionRequest, block_number: U64) -> Result<GethTrace> {
        let trace: GethTrace = self
            .provider
            .debug_trace_call(
                tx,
                Some(block_number.into()),
                GethDebugTracingCallOptions {
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
                },
            )
            .await?;

        Ok(trace)
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
        let calldata: Bytes = BalanceOfCall { who: account }.encode().into();

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

        let tx = Eip1559TransactionRequest {
            chain_id: Some(self.chain),
            nonce: Some(nonce),
            from: Some(account),
            to: Some(NameOrAddress::Address(token)),
            gas: None,
            value: None,
            data: Some(calldata),
            max_priority_fee_per_gas: None,
            max_fee_per_gas: None,
            access_list: AccessList::default(),
        };
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

    pub async fn eth_call_simulate_multi_swap(
        &mut self,
        target_pair: Address,
        input_token: Address,
        output_token: Address,
        input_token_balance_slot: i32,
    ) -> Result<U256> {
        // Shows how you can spoof multiple storage slots
        // but also shows that you can only test one transaction at a time
        let block = self
            .provider
            .get_block(BlockNumber::Latest)
            .await?
            .ok_or(anyhow!("failed to retrieve block"))?;

        // Spoof simulator input token balance
        let input_balance_slot: [u8; 32] = keccak256(&abi::encode(&[
            abi::Token::Address(self.simulator_address),
            abi::Token::Uint(U256::from(input_token_balance_slot)),
        ]));
        self.state_override_set
            .account(input_token)
            .store(input_balance_slot.into(), H256::from_low_u64_be(TEN_ETH.as_u64()));

        let calldata = SimulateMultiSwapCall {
            swaps: vec![],
            chain_swaps: true,
        }
        .encode();

        let tx: TypedTransaction = TransactionRequest::default()
            .from(self.account)
            .to(self.simulator_address)
            .value(U256::zero())
            .data(calldata)
            .nonce(U256::zero())
            .gas(5000000)
            .gas_price(GAS_PRICE)
            .chain_id(1)
            .into();
        let result = self
            .provider
            .provider()
            .call_raw(&tx)
            .state(&self.state_override_set)
            .block(block.number.unwrap().into())
            .await?;
        let out: U256 = SimulateMultiSwapReturn::decode(result)?.0;
        println!("simulateMultiSwap eth_call result: {:?}", out);

        Ok(out)
    }
}
