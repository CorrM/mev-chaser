use anyhow::{Ok, Result};
use ethers::{
    providers::{Http, Provider, Ws},
    types::{
        BlockId, BlockNumber, CallConfig, Eip1559TransactionRequest, GethDebugBuiltInTracerConfig,
        GethDebugBuiltInTracerType, GethDebugTracerConfig, GethDebugTracerType, GethDebugTracingCallOptions,
        GethDebugTracingOptions, GethTrace, Transaction, transaction::eip2718::TypedTransaction, TransactionRequest, U256, U64,
    },
};
use ethers_core::utils::parse_units;
use ethers_providers::Middleware;

use crate::network::NetworkKind;

use super::{NodeProvider, NodeProviderNetworkInfo, NormalNodeProvider};

fn get_trace_options() -> GethDebugTracingCallOptions {
    let tracer: Option<GethDebugTracerType> = Some(GethDebugTracerType::BuiltInTracer(
        GethDebugBuiltInTracerType::CallTracer,
    ));

    let tracer_config = Some(GethDebugTracerConfig::BuiltInTracer(
        GethDebugBuiltInTracerConfig::CallTracer(CallConfig {
            only_top_call: Some(false),
            with_log: Some(true),
        }),
    ));

    GethDebugTracingCallOptions {
        tracing_options: GethDebugTracingOptions {
            disable_storage: None,
            disable_stack: None,
            enable_memory: None,
            enable_return_data: None,
            tracer,
            tracer_config,
            timeout: None,
        },
        state_overrides: None,
        block_overrides: None,
    }
}

#[derive(Clone)]
pub struct DebugTraceCallNodeProvider {
    data: NormalNodeProvider,
}

impl DebugTraceCallNodeProvider {
    pub async fn new(name: impl Into<String>, network_info: NodeProviderNetworkInfo) -> Result<Self> {
        Ok(Self {
            data: NormalNodeProvider::new(name, network_info).await?,
        })
    }

    pub async fn debug_trace_call(&self, tx: Transaction, block_number: Option<U64>) -> Result<GethTrace> {
        // TODO: test if passing BlockId::Hash is faster
        let legacy: bool = tx.max_fee_per_gas.is_none() && tx.max_fee_per_gas.is_none();
        let chain_id: U64 = U64::from(tx.chain_id.unwrap_or(U256::from(1)).as_u64());

        let tx: TypedTransaction = match legacy {
            true => TransactionRequest::new()
                .from(tx.from)
                .to(tx.to.unwrap())
                .value(tx.value)
                .data(tx.input)
                .chain_id(chain_id)
                .nonce(tx.nonce)
                .gas(tx.gas)
                .gas_price(tx.gas_price.unwrap_or(parse_units(40, "gwei").unwrap().into()))
                .into(),
            false => Eip1559TransactionRequest::new()
                .from(tx.from)
                .to(tx.to.unwrap())
                .value(tx.value)
                .data(tx.input)
                .chain_id(chain_id)
                .nonce(tx.nonce)
                .gas(tx.gas)
                .max_fee_per_gas(tx.max_fee_per_gas.unwrap_or(parse_units(40, "gwei").unwrap().into()))
                .max_priority_fee_per_gas(
                    tx.max_priority_fee_per_gas
                        .unwrap_or(parse_units(40, "gwei").unwrap().into()),
                )
                .into(),
        };
        let block_number: Option<BlockId> = block_number.map(|b_number| BlockId::Number(BlockNumber::Number(b_number)));

        let trace: GethTrace = self
            .data
            .raw_ws_provider()
            .debug_trace_call(tx, block_number, get_trace_options())
            .await?;

        Ok(trace)
    }
}

impl NodeProvider for DebugTraceCallNodeProvider {
    fn name(&self) -> &str {
        self.data.name()
    }

    fn network(&self) -> &NetworkKind {
        self.data.network()
    }

    fn raw_http_provider(&self) -> &Provider<Http> {
        self.data.raw_http_provider()
    }

    fn raw_ws_provider(&self) -> &Provider<Ws> {
        self.data.raw_ws_provider()
    }
}
