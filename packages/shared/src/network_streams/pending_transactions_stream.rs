use std::sync::Arc;

use ethers_providers::Middleware;
use tokio::sync::broadcast::Sender;
use tokio_stream::StreamExt;

use crate::provider::NodeProvider;

use super::NetworkEvent;

pub async fn stream_pending_transactions(provider: Arc<NodeProvider>, event_sender: Sender<NetworkEvent>) {
    let stream = provider.ws_provider().subscribe_pending_txs().await.unwrap();
    let mut stream = stream.transactions_unordered(256).fuse();

    while let Some(result) = stream.next().await {
        if let Ok(tx) = result {
            if event_sender.send(NetworkEvent::PendingTx(tx)).is_err() {
                continue;
            }
        };
    }
}
