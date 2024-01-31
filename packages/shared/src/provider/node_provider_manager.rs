use std::sync::Arc;

use anyhow::Result;

use super::{DebugTraceCallNodeProvider, NormalNodeProvider};

pub struct NodeProviderManager {
    providers: Vec<Arc<NormalNodeProvider>>,
    debug_trace_call_providers: Vec<Arc<DebugTraceCallNodeProvider>>,
}

impl NodeProviderManager {
    pub fn new(
        providers: Vec<NormalNodeProvider>,
        debug_trace_call_providers: Vec<DebugTraceCallNodeProvider>,
    ) -> Result<Self> {
        Ok(Self {
            providers: providers.into_iter().map(Arc::new).collect(),
            debug_trace_call_providers: debug_trace_call_providers
                .into_iter()
                .map(Arc::new)
                .collect(),
        })
    }

    pub fn get_next(&self) -> &Arc<NormalNodeProvider> {
        &self.providers[0]
    }

    pub fn get_next_debug_trace_call(&self) -> &Arc<DebugTraceCallNodeProvider> {
        &self.debug_trace_call_providers[0]
    }
}
