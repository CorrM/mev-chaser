use ethers::types::Filter;
use tokio::sync::broadcast::Receiver;
use tokio::{
    sync::broadcast::Sender,
    task::{JoinError, JoinSet},
};

use crate::provider::NodeProviderRaw;

use super::network_event::NetworkEvent;
use super::log_stream::stream_log_event;
use super::new_block_stream::stream_new_blocks;
use super::pending_transactions_stream::stream_pending_transactions;

pub struct NetworkStreamsManager {
    join_set: JoinSet<()>,
    event_sender: Sender<NetworkEvent>,
}

impl NetworkStreamsManager {
    pub(super) fn new<T: 'static + NodeProviderRaw>(
        provider: T,
        events: &Vec<(NetworkEvent, Option<Filter>)>,
        event_sender: Sender<NetworkEvent>,
    ) -> Self {
        let mut set: JoinSet<()> = JoinSet::new();

        for (event, opt) in events {
            match event {
                NetworkEvent::Block(_) => {
                    set.spawn(stream_new_blocks(provider.clone(), event_sender.clone()));
                }
                NetworkEvent::PendingTx(_) => {
                    set.spawn(stream_pending_transactions(provider.clone(), event_sender.clone()));
                }
                NetworkEvent::Log(_) => {
                    set.spawn(stream_log_event(provider.clone(), event_sender.clone(), opt.clone().unwrap()));
                }
            }
        }

        Self {
            join_set: set,
            event_sender,
        }
    }

    pub async fn join_next(&mut self) -> Option<Result<(), JoinError>> {
        self.join_set.join_next().await
    }

    pub fn subscribe(&self) -> Receiver<NetworkEvent> {
        self.event_sender.subscribe()
    }
}
