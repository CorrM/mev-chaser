use crate::network_streams::Event;
use ethers::providers::{Provider, Ws};
use ethers_providers::Middleware;
use std::sync::Arc;
use tokio::sync::broadcast::Sender;
use tokio_stream::StreamExt;

pub async fn stream_pending_transactions(provider: Arc<Provider<Ws>>, event_sender: Sender<Event>) {
    let stream = provider.subscribe_pending_txs().await.unwrap();
    let mut stream = stream.transactions_unordered(256).fuse();

    while let Some(result) = stream.next().await {
        if let Ok(tx) = result {
            if event_sender.send(Event::PendingTx(tx)).is_err() {
                continue;
            }
        };
    }
}
