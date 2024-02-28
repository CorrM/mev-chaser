use std::{
    ops::{Div, Mul},
    sync::Arc,
};

use anyhow::{Context, Result};
use async_trait::async_trait;
use ethers::abi::AbiEncode;
use ethers::prelude::PendingTransaction;
use ethers::{providers::Middleware, types::U256};

use crate::core::Executor;
use crate::types::{Notification, SubmitTxInfo};

/// Information about the gas bid for a transaction.
#[derive(Debug, Clone)]
pub struct GasBidInfo {
    /// Total profit expected from opportunity
    pub total_profit: U256,

    /// Percentage of bid profit to use for gas
    pub bid_percentage: u64,
}

/// An executor that sends transactions to the mempool.
pub struct MempoolExecutor<M> {
    client: Arc<M>,
}

impl<M> MempoolExecutor<M> {
    pub fn new(client: Arc<M>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl<M> Executor<SubmitTxInfo> for MempoolExecutor<M>
where
    M: Middleware,
    M::Error: 'static,
{
    /// Send a transaction to the mempool.
    #[inline]
    async fn execute(&self, mut action: SubmitTxInfo) -> Result<Option<Notification>> {
        let gas_usage: U256 = self
            .client
            .estimate_gas(&action.tx, None)
            .await
            .context("Error estimating gas usage: {}")?;

        let bid_gas_price: U256;
        if let Some(gas_bid_info) = action.gas_bid_info {
            // gas price at which we'd break even, meaning 100% of profit goes to validator
            let breakeven_gas_price: U256 = gas_bid_info.total_profit / gas_usage;
            // gas price corresponding to bid percentage
            bid_gas_price = breakeven_gas_price.mul(gas_bid_info.bid_percentage).div(100);
        } else {
            bid_gas_price = self
                .client
                .get_gas_price()
                .await
                .context("Error getting gas price: {}")?;
        }

        action.tx.set_gas_price(bid_gas_price);
        let p_tx: PendingTransaction<<M as Middleware>::Provider> =
            self.client.send_transaction(action.tx.clone(), None).await?;

        Ok(Some(Notification {
            message: format!("Sent transaction: {}", p_tx.tx_hash().encode_hex()),
        }))
    }
}
