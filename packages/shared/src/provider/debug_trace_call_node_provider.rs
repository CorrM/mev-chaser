use std::sync::Arc;

use anyhow::Result;
use ethers::{
    providers::Provider,
    types::{
        CallConfig, GethDebugBuiltInTracerConfig, GethDebugBuiltInTracerType, GethDebugTracerConfig, GethDebugTracerType,
        GethDebugTracingCallOptions, GethDebugTracingOptions, GethTrace, Transaction, U64,
    },
};
use ethers_core::types::{CallFrame, CallLogFrame, GethTraceFrame};
use ethers_providers::{Middleware, Ws, Http};

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

    pub async fn debug_trace_call(&self, tx: &Transaction, block_number: U64) -> Result<Option<CallFrame>> {
        let call_config = CallConfig {
            with_log: Some(true), // 👈 make sure we are getting logs
            ..Default::default()
        };
        
        let mut opts = GethDebugTracingCallOptions::default();
        opts.tracing_options.tracer = Some(GethDebugTracerType::BuiltInTracer(
            GethDebugBuiltInTracerType::CallTracer,
        ));
        opts.tracing_options.tracer_config = Some(GethDebugTracerConfig::BuiltInTracer(
            GethDebugBuiltInTracerConfig::CallTracer(call_config),
        ));

        let provider: &Arc<Provider<Ws>> = self.raw_ws_provider();
        let mut tx: Transaction = tx.clone();

        let nonce = self
            .raw_ws_provider()
            .get_transaction_count(tx.from, Some(block_number.into()))
            .await
            .unwrap_or_default();
        tx.nonce = nonce;

        let trace = provider.debug_trace_call(&tx, Some(block_number.into()), opts).await;
        if trace.is_err() {
            return Ok(None);
        }

        let trace: GethTrace = trace.unwrap();
        let GethTrace::Known(call_tracer) = trace else { return Ok(None); };
        let GethTraceFrame::CallTracer(frame) = call_tracer else { return Ok(None); };

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
