use std::sync::Arc;

use anyhow::Result;
use ethers::{
    providers::Provider,
    types::{
        CallConfig, GethDebugBuiltInTracerConfig, GethDebugBuiltInTracerType, GethDebugTracerConfig,
        GethDebugTracerType, GethDebugTracingCallOptions, GethDebugTracingOptions, GethTrace, Transaction,
    },
};
use ethers_core::types::{CallFrame, CallLogFrame, GethTraceFrame, TxHash, U256, U64};
use ethers_providers::{Http, Middleware, Ws};

use crate::network::NetworkKind;

use super::{NodeProvider, NodeProviderNetworkInfo, NormalNodeProvider};

#[derive(Clone)]
pub struct DebugTraceCallNodeProvider {
    data: NormalNodeProvider,
    debug_trace_options: GethDebugTracingCallOptions,
}

impl DebugTraceCallNodeProvider {
    pub async fn new(name: impl Into<String>, network_info: NodeProviderNetworkInfo) -> Result<Self> {
        let tracer = Some(GethDebugTracerType::BuiltInTracer(
            GethDebugBuiltInTracerType::CallTracer,
        ));

        let tracer_config = Some(GethDebugTracerConfig::BuiltInTracer(
            GethDebugBuiltInTracerConfig::CallTracer(CallConfig {
                only_top_call: Some(false),
                with_log: Some(true),
            }),
        ));

        let debug_trace_options: GethDebugTracingCallOptions = GethDebugTracingCallOptions {
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
        };

        Ok(Self {
            data: NormalNodeProvider::new(name, network_info).await?,
            debug_trace_options,
        })
    }

    pub fn extract_trace_logs(call_frame: &CallFrame, logs: &mut Vec<CallLogFrame>) {
        if let Some(ref logs_vec) = call_frame.logs {
            logs.extend(logs_vec.iter().cloned());
        }

        if let Some(ref calls_vec) = call_frame.calls {
            for call in calls_vec {
                DebugTraceCallNodeProvider::extract_trace_logs(call, logs);
            }
        }
    }

    pub async fn debug_trace_call(&self, tx: &Transaction, block_number: Option<U64>) -> Result<Option<CallFrame>> {
        let provider: &Arc<Provider<Ws>> = self.raw_ws_provider();

        /*
        let call_config = CallConfig {
            with_log: Some(true), // 👈 make sure we are getting logs
            ..Default::default()
        };

        let mut trace_options = GethDebugTracingCallOptions::default();
        trace_options.tracing_options.tracer = Some(GethDebugTracerType::BuiltInTracer(
            GethDebugBuiltInTracerType::CallTracer,
        ));
        trace_options.tracing_options.tracer_config = Some(GethDebugTracerConfig::BuiltInTracer(
            GethDebugBuiltInTracerConfig::CallTracer(call_config),
        ));
        
        // Nonce needed when spacfing the block
        let mut tx: Transaction = tx.clone();
        let nonce: U256 = self
            .raw_ws_provider()
            .get_transaction_count(tx.from, Some(block_number.into()))
            .await
            .unwrap_or_default();
        tx.nonce = nonce;
        */

        let trace: GethTrace = provider
            .debug_trace_call(tx, None, self.debug_trace_options.clone())
            .await?;

        let GethTrace::Known(call_tracer) = trace else {
            return Ok(None);
        };
        let GethTraceFrame::CallTracer(frame) = call_tracer else {
            return Ok(None);
        };

        Ok(Some(frame))
    }
    pub async fn debug_trace_transaction(&self, tx_hash: TxHash) -> Result<Option<CallFrame>> {
        let provider: &Arc<Provider<Ws>> = self.raw_ws_provider();

        let debug_opts = GethDebugTracingOptions {
            tracer: Some(GethDebugTracerType::BuiltInTracer(
                GethDebugBuiltInTracerType::CallTracer,
            )),
            ..Default::default()
        };

        let trace: GethTrace = provider.debug_trace_transaction(tx_hash, debug_opts).await?;
        let GethTrace::Known(call_tracer) = trace else {
            return Ok(None);
        };

        let GethTraceFrame::CallTracer(frame) = call_tracer else {
            return Ok(None);
        };

        Ok(Some(frame))
    }
}

impl NodeProvider for DebugTraceCallNodeProvider {
    fn name(&self) -> &str {
        self.data.name()
    }

    fn network(&self) -> &NetworkKind {
        self.data.network()
    }

    fn raw_http_provider(&self) -> &Arc<Provider<Http>> {
        self.data.raw_http_provider()
    }

    fn raw_ws_provider(&self) -> &Arc<Provider<Ws>> {
        self.data.raw_ws_provider()
    }
}
