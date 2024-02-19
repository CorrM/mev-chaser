use std::str::FromStr;

use ethers_core::types::Address;
use ethers_providers::{Middleware, PubsubClient};
use serde_json::from_str;
use tokio::sync::broadcast::Sender;
use tokio_stream::StreamExt;

use super::NetworkEvent;

pub async fn stream_pending_transactions<M>(
    provider: M,
    event_sender: Sender<NetworkEvent>,
    filter_to_addresses: Option<Vec<String>>,
) where
    M: Middleware,
    M::Provider: PubsubClient,
{
    //let mut stream = ws.subscribe_pending_txs().await.unwrap();
    //let mut stream = stream.transactions_unordered(256).fuse();

    /*
    let mut stream = match provider.name() {
        "Alchemy" => {
            let alchemy_event: Value = utils::serialize(&"alchemy_pendingTransactions");
            let sub_params: Vec<Value> = if let Some(filter_to_addresses) = filter_to_addresses {
                vec![alchemy_event, json!({ "toAddress": filter_to_addresses })]
            } else {
                vec![alchemy_event]
            };

            provider.subscribe(sub_params).await.unwrap()
        }
        _ => provider.subscribe_full_pending_txs().await.unwrap(),
    };
    */

    let filter_to_addresses: Option<Vec<Address>> = filter_to_addresses.map(|filter_to_addresses| {
        filter_to_addresses
            .iter()
            .map(|a| Address::from_str(a).unwrap())
            .collect()
    });

    let mut stream = provider.subscribe_full_pending_txs().await.unwrap();
    while let Some(tx) = stream.next().await {
        let Some(to) = tx.to else {
            continue;
        };

        if let Some(ref filter_to_addresses) = filter_to_addresses {
            if !filter_to_addresses.contains(&to) {
                continue;
            }
        }

        if event_sender.send(NetworkEvent::PendingTx(tx)).is_err() {
            continue;
        }
    }

    panic!("Pending transactions stream stopped");
}
