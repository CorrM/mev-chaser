use std::sync::Arc;

use ethers::{
    providers::Ws,
    types::{Filter, Log},
};
use ethers_providers::{Middleware, Provider, SubscriptionStream};
use tokio::sync::broadcast::Sender;
use tokio_stream::StreamExt;

use crate::provider::NodeProvider;

use super::NetworkEvent;

pub async fn stream_log_event<T: NodeProvider>(
    provider: T,
    event_sender: Sender<NetworkEvent>,
    filter: Filter,
) {
    let ws: &Arc<Provider<Ws>> = provider.raw_ws_provider();
    let mut stream: SubscriptionStream<Ws, Log> = ws.subscribe_logs(&filter).await.unwrap();

    while let Some(result) = stream.next().await {
        if event_sender.send(NetworkEvent::Log(result)).is_err() {
            continue;
        };
    }

    panic!("Log stream stopped");
}
