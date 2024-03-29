use std::sync::{Arc, RwLock};

use anyhow::Result;
use ethers::prelude::Filter;
use ethers::providers::Middleware;

use shared::managers::{BlockManager, PoolManager};
use shared::simulator::EvmSimulator;
use shared::types::{BlockInfo, MevEvents};
use vidger::core::PreStrategy;
use vidger::logger::error;
use vidger::utilities::block_on;

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

impl<M: Middleware + 'static> PreStrategy<MevEvents> for MainPreStrategy<M> {
    /// Setup by getting all pools to monitor for swaps
    fn sync_state(&mut self) -> Result<()> {
        self.block_manager.write().unwrap().setup(Arc::clone(&self.provider))?;
        self.pool_manager.write().unwrap().setup()?;

        Ok(())
    }

    /// Handle incoming events.
    fn on_event(&mut self, event: &mut MevEvents) {
        let MevEvents::NewBlock(block) = event else {
            return;
        };

        let b_info: BlockInfo = block.clone().into();
        let event_filter: Filter = Filter::new().from_block(block.number).to_block(block.number);
        let Ok(logs) = block_on(self.provider.get_logs(&event_filter)) else {
            error!("Pre-Strategy: Failed to get logs from block '{}'", block.number);
            return;
        };

        // Don't change the order of these calls, And don't call them in parallel
        self.block_manager.write().unwrap().update_block_info(b_info);
        self.simulator.write().unwrap().sync_by_block(block, &logs.clone());
        self.pool_manager.write().unwrap().on_new_block(block, &logs);
    }
}
