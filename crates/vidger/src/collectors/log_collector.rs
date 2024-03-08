use std::sync::Arc;

use anyhow::Result;
use ethers::{
    providers::{Middleware, PubsubClient},
    types::{Filter, Log},
};
use futures::StreamExt;

use crate::core::{Collector, CollectorStream};
use crate::utilities::block_on;

/// A collector that listens for new blockchain event logs based on a [Filter](Filter),
/// and generates a stream of [events](Log).
pub struct LogCollector<M> {
    provider: Arc<M>,
    filter: Filter,
}

impl<M> LogCollector<M> {
    pub fn new(provider: Arc<M>, filter: Filter) -> Self {
        Self { provider, filter }
    }
}

/// Implementation of the [Collector](Collector) trait for the [LogCollector](LogCollector).
/// This implementation uses the [PubsubClient](PubsubClient) to subscribe to new logs.
impl<M> Collector<Log> for LogCollector<M>
where
    M: Middleware,
    M::Provider: PubsubClient,
    M::Error: 'static,
{
    fn get_event_stream(&self) -> Result<CollectorStream<'_, Log>> {
        let stream = block_on(self.provider.subscribe_logs(&self.filter))?;
        let stream = stream.filter_map(|log| async move { Some(log) });
        Ok(Box::pin(stream))
    }
}
