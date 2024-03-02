use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use ethers::providers::Middleware;

use shared::managers::{BlockManager, PoolManager};
use shared::types::MevEvents;
use vidger::core::PreStrategy;

pub struct MainPreStrategy<M> {
    /// Ethers client
    provider: Arc<M>,
    /// Keeps track of onchain pools
    pool_manager: PoolManager<M>,
    /// Block manager
    block_manager: BlockManager,
}

impl<M: Middleware + 'static> MainPreStrategy<M> {
    /// Create a new instance
    pub fn new(client: Arc<M>) -> Self {
        Self {
            provider: Arc::clone(&client),
            pool_manager: PoolManager::new(client),
            block_manager: BlockManager::new(),
        }
    }
}

#[async_trait]
impl<M: Middleware + 'static> PreStrategy<MevEvents> for MainPreStrategy<M> {
    /// Setup by getting all pools to monitor for swaps
    async fn sync_state(&mut self) -> Result<()> {
        self.block_manager.setup(self.provider.clone()).await?;
        self.pool_manager.setup().await?;

        Ok(())
    }

    /// Handle incoming events.
    async fn on_event(&mut self, event: &mut MevEvents) {
        let MevEvents::NewBlock(block) = event else {
            return;
        };

        self.block_manager.update_block_info(block.clone());
        self.pool_manager.on_new_block(block.number).await;
    }
}
