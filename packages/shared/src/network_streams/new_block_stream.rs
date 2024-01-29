use std::sync::Arc;

use ethers::types::{Block, H256, U256, U64};
use ethers_providers::Middleware;
use tokio::sync::broadcast::Sender;
use tokio_stream::StreamExt;

use crate::{provider::NodeProvider, utils::calculate_next_block_base_fee};

use super::network_event::NetworkEvent;

#[derive(Default, Debug, Clone)]
pub struct NewBlock {
    pub block_number: U64,
    pub base_fee: U256,
    pub next_base_fee: U256,
}

pub async fn stream_new_blocks(provider: Arc<NodeProvider>, event_sender: Sender<NetworkEvent>) {
    let stream = provider.ws_provider().subscribe_blocks().await.unwrap();
    let mut stream = stream.filter_map(|block: Block<H256>| match block.number {
        Some(number) => Some(NewBlock {
            block_number: number,
            base_fee: block.base_fee_per_gas.unwrap_or_default(),
            next_base_fee: calculate_next_block_base_fee(
                block.gas_used,
                block.gas_limit,
                block.base_fee_per_gas.unwrap_or_default(),
            ),
        }),
        None => None,
    });

    while let Some(block) = stream.next().await {
        if event_sender.send(NetworkEvent::Block(block)).is_err() {
            continue;
        }
    }
}
