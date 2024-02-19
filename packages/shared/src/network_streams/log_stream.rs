use ethers::types::Filter;
use ethers_providers::{Middleware, PubsubClient};
use tokio::sync::broadcast::Sender;
use tokio_stream::StreamExt;


use super::NetworkEvent;

pub async fn stream_log_event<M>(provider: M, event_sender: Sender<NetworkEvent>, filter: Filter)
where
    M: Middleware,
    M::Provider: PubsubClient,
{
    let mut stream = provider.subscribe_logs(&filter).await.unwrap();

    while let Some(result) = stream.next().await {
        if event_sender.send(NetworkEvent::Log(result)).is_err() {
            continue;
        };
    }

    panic!("Log stream stopped");
}
