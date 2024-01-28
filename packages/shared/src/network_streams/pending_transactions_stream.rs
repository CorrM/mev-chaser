use std::sync::Arc;
use ethers::providers::{Provider, Ws};
use ethers_providers::Middleware;
use tokio::sync::broadcast::Sender;
use tokio_stream::StreamExt;
use crate::network_streams::Event;

pub async fn stream_pending_transactions(provider: Arc<Provider<Ws>>, event_sender: Sender<Event>) {
    let stream = provider.subscribe_pending_txs().await.unwrap();
    let mut stream = stream.transactions_unordered(256).fuse();

    while let Some(result) = stream.next().await {
        match result {
            Ok(tx) => match event_sender.send(Event::PendingTx(tx)) {
                Ok(_) => {}
                Err(_) => {}
            },
            Err(_) => {}
        };
    }
}