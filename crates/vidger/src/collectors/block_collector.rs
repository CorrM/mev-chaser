use std::sync::Arc;

use anyhow::Result;
use ethers::providers::{Middleware, PubsubClient};
use tokio_stream::StreamExt;

use crate::core::{Collector, CollectorStream};
use crate::types::NewBlock;
use crate::utilities::block_on;

/// A collector that listens for new blocks, and generates a stream of
/// [events](NewBlock) which contain the block number and hash.
pub struct BlockCollector<M> {
    provider: Arc<M>,
}

impl<M> BlockCollector<M>
where
    M: Middleware + 'static,
    M::Provider: PubsubClient,
    M::Error: 'static,
{
    pub fn new(provider: Arc<M>) -> Self {
        Self { provider }
    }
}

/// Implementation of the [Collector](Collector) trait for the [BlockCollector](BlockCollector).
/// This implementation uses the [PubsubClient](PubsubClient) to subscribe to new blocks.
impl<M> Collector<NewBlock> for BlockCollector<M>
where
    M: Middleware,
    M::Provider: PubsubClient,
    M::Error: 'static,
{
    fn get_event_stream(&self) -> Result<CollectorStream<'_, NewBlock>> {
        let stream = block_on(self.provider.subscribe_blocks())?;
        let stream = stream.filter_map(|block| match block.number {
            Some(number) => Some(NewBlock {
                number,
                gas_limit: block.gas_limit,
                gas_used: block.gas_used,
                base_fee_per_gas: block.base_fee_per_gas.unwrap_or_default(),
                timestamp: block.timestamp,
            }),
            None => None,
        });

        Ok(Box::pin(stream))
    }
}
