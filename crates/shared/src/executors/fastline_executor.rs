use std::sync::Arc;

use anyhow::Result;
use ethers::providers::Middleware;

use vidger::core::Executor;
use vidger::types::{Notification, SubmitTxInfo};

/// An executor that sends transactions to one of relays or mempool.
pub struct FastLineExecutor<M> {
    client: Arc<M>,
}

impl<M> FastLineExecutor<M> {
    pub fn new(client: Arc<M>) -> Self {
        Self { client }
    }
}

impl<M> Executor<SubmitTxInfo> for FastLineExecutor<M>
where
    M: Middleware,
    M::Error: 'static,
{
    /// Send a transaction to one of relays or mempool.
    fn execute(&self, action: SubmitTxInfo) -> Result<Option<Notification>> {
        Ok(Some(Notification {
            message: "Test".to_owned(),
        }))
    }
}
