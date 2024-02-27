use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use ethers::middleware::Middleware;

use vidger::core::Executor;
use vidger::executors::SubmitTxToMempool;

/// An executor that sends transactions to one of relays or mempool.
pub struct SmartExecutor<M> {
    client: Arc<M>,
}

impl<M> SmartExecutor<M> {
    pub fn new(client: Arc<M>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl<M> Executor<SubmitTxToMempool> for SmartExecutor<M>
where
    M: Middleware,
    M::Error: 'static,
{
    /// Send a transaction to one of relays or mempool.
    async fn execute(&self, action: &mut SubmitTxToMempool) -> Result<()> {
        /*
        Check current block validators to see if he is participating in any bundle provider
        If not, send it to mempool.

        Before that I need to collect data about validators:
            How many blocks he can mind
            Is he participating in any bundle provider
        */

        Ok(())
    }
}
