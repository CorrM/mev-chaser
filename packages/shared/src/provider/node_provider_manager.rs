use std::sync::Arc;

use anyhow::Result;

use super::NodeProvider;

pub struct NodeProviderManager {
    providers: Vec<Arc<NodeProvider>>,
    debug_trace_call_providers: Vec<Arc<NodeProvider>>,
}

impl NodeProviderManager {
    pub fn new(
        providers: Vec<NodeProvider>,
        debug_trace_call_providers: Vec<NodeProvider>,
    ) -> Result<Self> {
        Ok(Self {
            providers: providers.into_iter().map(Arc::new).collect(),
            debug_trace_call_providers: debug_trace_call_providers
                .into_iter()
                .map(Arc::new)
                .collect(),
        })
    }

    pub fn get_next(&self) -> &Arc<NodeProvider> {
        &self.providers[0]
    }

    pub fn get_next_debug_trace_call(&self) -> &Arc<NodeProvider> {
        &self.debug_trace_call_providers[0]
    }
}
