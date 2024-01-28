use crate::network_streams::Event;
use ethers::{
    providers::{Provider, Ws},
    types::{Filter, Log},
};
use ethers_providers::{Middleware, SubscriptionStream};
use std::sync::Arc;
use tokio::sync::broadcast::Sender;
use tokio_stream::StreamExt;

pub async fn stream_log_event(
    provider: Arc<Provider<Ws>>,
    event_signature: impl Into<String>,
    event_sender: Sender<Event>,
) {
    let filter: Filter = Filter::new().event(&event_signature.into());
    let mut stream: SubscriptionStream<Ws, Log> = provider.subscribe_logs(&filter).await.unwrap();

    while let Some(result) = stream.next().await {
        if event_sender.send(Event::Log(result)).is_err() {
            continue;
        };
    }
}
