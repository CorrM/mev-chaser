use ethers_providers::{Middleware, Provider, Ws};
use tokio::sync::broadcast::Sender;
use tokio_stream::StreamExt;

use crate::provider::NodeProviderRaw;

use super::NetworkEvent;

pub async fn stream_pending_transactions<T: NodeProviderRaw>(provider: T, event_sender: Sender<NetworkEvent>) {
    let ws: &Provider<Ws> = provider.raw_ws_provider();
    let stream = ws.subscribe_pending_txs().await.unwrap();
    let mut stream = stream.transactions_unordered(256).fuse();

    while let Some(result) = stream.next().await {
        if let Ok(tx) = result {
            if event_sender.send(NetworkEvent::PendingTx(tx)).is_err() {
                continue;
            }
        };
    }
}
