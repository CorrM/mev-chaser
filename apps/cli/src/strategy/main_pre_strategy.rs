use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use ethers::providers::Middleware;
use tokio::sync::RwLock;

use shared::managers::{BlockManager, PoolManager};
use shared::simulator::EvmSimulator;
use shared::types::MevEvents;
use vidger::core::PreStrategy;

pub struct MainPreStrategy<M> {
    /// Ethers client
    provider: Arc<M>,
    /// Keeps track of onchain pools
    pool_manager: Arc<RwLock<PoolManager<M>>>,
    /// Block manager
    block_manager: Arc<RwLock<BlockManager>>,
    /// EVM simulator
    simulator: Arc<RwLock<EvmSimulator<M>>>,
}

impl<M: Middleware + 'static> MainPreStrategy<M> {
    /// Create a new instance
    pub fn new(
        provider: Arc<M>,
        pool_manager: Arc<RwLock<PoolManager<M>>>,
        block_manager: Arc<RwLock<BlockManager>>,
        simulator: Arc<RwLock<EvmSimulator<M>>>,
    ) -> Self {
        Self {
            provider,
            pool_manager,
            block_manager,
            simulator,
        }
    }
}

#[async_trait]
impl<M: Middleware + 'static> PreStrategy<MevEvents> for MainPreStrategy<M> {
    /// Setup by getting all pools to monitor for swaps
    async fn sync_state(&mut self) -> Result<()> {
        self.block_manager
            .write()
            .await
            .setup(Arc::clone(&self.provider))
            .await?;
        self.pool_manager.write().await.setup().await?;

        Ok(())
    }

    /// Handle incoming events.
    async fn on_event(&mut self, event: &mut MevEvents) {
        let MevEvents::NewBlock(block) = event else {
            return;
        };

        self.block_manager.write().await.update_block_info(block.clone());
        self.pool_manager.write().await.on_new_block(block.number).await;
        self.simulator.write().await.update_block().await;
    }
}
