use anyhow::Result;
use async_trait::async_trait;
use ethers::middleware::Middleware;
use ethers::signers::LocalWallet;
use std::sync::Arc;

use crate::vidger_types::{Actions, Events};

pub struct BackRunningStrategyConfig {
    pub searcher_signer: LocalWallet,
}

pub struct BackRunningStrategy<M> {
    /// Ethers client
    provider: Arc<M>,
    /// Keeps track of onchain pools
    pool_manager: PoolManager<M>,
    /// Block manager
    block_manager: BlockManager,
    /// Keeps track of weth inventory & token dust
    sando_state_manager: SandoStateManager,
}

impl<M: Middleware + 'static> BackRunningStrategy<M> {
    /// Create a new instance
    pub fn new(client: Arc<M>, config: BackRunningStrategyConfig) -> Self {
        Self {
            pool_manager: PoolManager::new(client.clone()),
            provider: client,
            block_manager: BlockManager::new(),
            sando_state_manager: SandoStateManager::new(
                config.sando_address,
                config.searcher_signer,
                config.sando_inception_block,
            ),
        }
    }
}

#[async_trait]
impl<M: Middleware + 'static> Strategy<Events, Actions> for BackRunningStrategy<M> {
    /// Setup by getting all pools to monitor for swaps
    async fn sync_state(&mut self) -> Result<()> {
        self.pool_manager.setup().await?;
        self.sando_state_manager
            .setup(self.provider.clone())
            .await?;
        self.block_manager.setup(self.provider.clone()).await?;
        Ok(())
    }

    /// Process incoming events
    async fn process_event(&mut self, event: Events) -> Option<Actions> {
        match event {
            Events::NewBlock(block) => match self.process_new_block(block).await {
                Ok(_) => None,
                Err(e) => {
                    panic!("strategy is out of sync {}", e);
                }
            },
            Events::NewTransaction(tx) => self.process_new_tx(tx).await,
        }
    }
}
