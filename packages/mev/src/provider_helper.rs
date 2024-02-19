use std::sync::Arc;

use anyhow::Result;
use ethers_core::types::{
    CallConfig, CallFrame, CallLogFrame, GethDebugBuiltInTracerConfig, GethDebugBuiltInTracerType,
    GethDebugTracerConfig, GethDebugTracerType, GethDebugTracingCallOptions, GethDebugTracingOptions, GethTrace,
    GethTraceFrame, Transaction, TxHash, U64,
};
use ethers_providers::Middleware;

pub struct ProviderHelper;

impl ProviderHelper {
    pub fn extract_trace_logs(call_frame: &CallFrame, logs: &mut Vec<CallLogFrame>) {
        if let Some(ref logs_vec) = call_frame.logs {
            logs.extend(logs_vec.iter().cloned());
        }

        if let Some(ref calls_vec) = call_frame.calls {
            for call in calls_vec {
                ProviderHelper::extract_trace_logs(call, logs);
            }
        }
    }

    pub async fn debug_trace_call<M>(
        provider: Arc<M>,
        tx: &Transaction,
        block_number: Option<U64>,
    ) -> Result<Option<CallFrame>>
    where
        M: Middleware + 'static,
    {
        /*
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
        */
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

        /*
        // Nonce needed when spacfing the block
        let mut tx: Transaction = tx.clone();
        let nonce: U256 = self
            .raw_ws_provider()
            .get_transaction_count(tx.from, Some(block_number.into()))
            .await
            .unwrap_or_default();
        tx.nonce = nonce;
        */

        let trace: GethTrace = provider.debug_trace_call(tx, None, trace_options).await?;

        let GethTrace::Known(call_tracer) = trace else {
            return Ok(None);
        };
        let GethTraceFrame::CallTracer(frame) = call_tracer else {
            return Ok(None);
        };

        Ok(Some(frame))
    }

    pub async fn debug_trace_transaction<M>(provider: &M, tx_hash: TxHash) -> Result<Option<CallFrame>>
    where
        M: Middleware + 'static,
    {
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
