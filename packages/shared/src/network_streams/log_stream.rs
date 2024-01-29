use std::sync::Arc;

use ethers::{
    providers::Ws,
    types::{Filter, Log},
};
use ethers_providers::{Middleware, SubscriptionStream};
use tokio::sync::broadcast::Sender;
use tokio_stream::StreamExt;

use crate::provider::NodeProvider;

use super::NetworkEvent;

pub async fn stream_log_event(
    provider: Arc<NodeProvider>,
    event_sender: Sender<NetworkEvent>,
    filter: Filter,
) {
    let mut stream: SubscriptionStream<Ws, Log> = provider.ws_provider().subscribe_logs(&filter).await.unwrap();

    while let Some(result) = stream.next().await {
        if event_sender.send(NetworkEvent::Log(result)).is_err() {
            continue;
        };
    }
}
