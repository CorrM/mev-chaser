use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use ethers::providers::Middleware;
use ethers::signers::LocalWallet;
use ethers::types::Transaction;
use tokio::sync::RwLock;

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
    async fn process_new_tx(&mut self, tx: &mut Transaction) -> Option<MevActions> {
        None
    }
}

#[async_trait]
impl<M: Middleware + 'static> Strategy<MevEvents, MevActions> for BackRunningStrategy<M> {
    async fn sync_state(&mut self) -> Result<()> {
        Ok(())
    }

    /// Process incoming events
    async fn process_event(&mut self, event: &mut MevEvents) -> Option<MevActions> {
        match event {
            MevEvents::NewTransaction(tx) => self.process_new_tx(tx).await,
            _ => None,
        }
    }
}
