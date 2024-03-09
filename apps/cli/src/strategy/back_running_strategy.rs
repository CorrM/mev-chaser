use std::sync::{Arc, RwLock};

use anyhow::Result;
use ethers::providers::Middleware;
use ethers::signers::LocalWallet;
use ethers::types::Transaction;

use shared::managers::{BlockManager, PoolManager};
use shared::types::{MevActions, MevEvents};
use vidger::core::Strategy;

pub struct BackRunningStrategyConfig {
    pub searcher_signer: LocalWallet,
}

pub struct BackRunningStrategy<M> {
    config: BackRunningStrategyConfig,
    provider: Arc<M>,
    pool_manager: Arc<RwLock<PoolManager<M>>>,
    block_manager: Arc<RwLock<BlockManager>>,
}

impl<M: Middleware + 'static> BackRunningStrategy<M> {
    /// Create a new instance
    pub fn new(
        provider: Arc<M>,
        config: BackRunningStrategyConfig,
        pool_manager: Arc<RwLock<PoolManager<M>>>,
        block_manager: Arc<RwLock<BlockManager>>,
    ) -> Self {
        Self {
            config,
            pool_manager,
            provider,
            block_manager,
        }
    }
}

impl<M: Middleware + 'static> BackRunningStrategy<M> {
    fn process_new_tx(&mut self, tx: &mut Transaction) -> Option<MevActions> {
        None
    }
}

impl<M: Middleware + 'static> Strategy<MevEvents, MevActions> for BackRunningStrategy<M> {
    fn sync_state(&mut self) -> Result<()> {
        Ok(())
    }

    /// Process incoming events
    fn process_event(&mut self, event: &mut MevEvents) -> Option<MevActions> {
        match event {
            MevEvents::NewTransaction(tx) => self.process_new_tx(tx),
            _ => None,
        }
    }
}
