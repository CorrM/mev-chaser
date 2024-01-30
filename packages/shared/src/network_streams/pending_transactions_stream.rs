use ethers_core::utils;
use ethers_providers::{Middleware, Provider, Ws};
use serde_json::{json, Value};
use tokio::sync::broadcast::Sender;
use tokio_stream::StreamExt;

use crate::provider::NodeProvider;

use super::NetworkEvent;

pub async fn stream_pending_transactions<T: NodeProvider>(
    provider: T,
    event_sender: Sender<NetworkEvent>,
    filter_to_addresses: Option<Vec<String>>,
) {
    let ws: &Provider<Ws> = provider.raw_ws_provider();

    //let mut stream = ws.subscribe_pending_txs().await.unwrap();
    //let mut stream = stream.transactions_unordered(256).fuse();

    //let mut stream = ws.subscribe_full_pending_txs().await.unwrap();

    let mut stream = match provider.name() {
        "Alchemy" => {
            let alchemy_event: Value = utils::serialize(&"alchemy_pendingTransactions");
            let sub_params: Vec<Value> = if let Some(filter_to_addresses) = filter_to_addresses {
                vec![alchemy_event, json!({ "toAddress": filter_to_addresses })]
            } else {
                vec![alchemy_event]
            };

            ws.subscribe(sub_params).await.unwrap()
        }
        _ => {
            ws.subscribe_full_pending_txs().await.unwrap()
        }
    };

    while let Some(tx) = stream.next().await {
        if event_sender.send(NetworkEvent::PendingTx(tx)).is_err() {
            continue;
        }
    }

    panic!("Pending transactions stream stopped");
}
