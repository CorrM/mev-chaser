use std::sync::Arc;

use anyhow::Result;

use super::NodeProvider;

pub struct NodeProviderManager {
    providers: Vec<Arc<NodeProvider>>,
}

impl NodeProviderManager {
    pub fn new(providers: Vec<NodeProvider>) -> Result<Self> {
        Ok(Self {
            providers: providers.into_iter().map(Arc::new).collect(),
        })
    }

    pub fn get_next(&self) -> &Arc<NodeProvider> {
        &self.providers[0]
    }
}
