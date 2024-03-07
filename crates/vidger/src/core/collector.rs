use std::pin::Pin;

use futures::Stream;
use tokio_stream::StreamExt;

/// A stream of events emitted by a [Collector](Collector).
pub type CollectorStream<'a, E> = Pin<Box<dyn Stream<Item = E> + Send + 'a>>;

/// Collector trait, which defines a source of events.
pub trait Collector<E>: Send + Sync {
    /// Returns the core event stream for the collector.
    fn get_event_stream(&self) -> anyhow::Result<CollectorStream<'_, E>>;
}

/// CollectorMap is a wrapper around a [Collector](Collector) that maps outgoing
/// events to a different type.
pub struct CollectorMapper<E, F> {
    collector: Box<dyn Collector<E>>,
    f: F,
}

impl<E, F> CollectorMapper<E, F> {
    pub fn new(collector: Box<dyn Collector<E>>, f: F) -> Self {
        Self { collector, f }
    }
}

impl<E1, E2, F> Collector<E2> for CollectorMapper<E1, F>
where
    E1: Send + Sync + 'static,
    E2: Send + Sync + 'static,
    F: Fn(E1) -> E2 + Send + Sync + Clone + 'static,
{
    #[inline]
    fn get_event_stream(&self) -> anyhow::Result<CollectorStream<'_, E2>> {
        let stream: CollectorStream<E1> = self.collector.get_event_stream()?;
        let f = self.f.clone();
        let stream = stream.map(f);
        Ok(Box::pin(stream))
    }
}
