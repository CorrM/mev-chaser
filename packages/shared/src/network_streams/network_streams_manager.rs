use ethers::types::Filter;
use ethers_providers::{Middleware, PubsubClient};
use tokio::sync::broadcast::Receiver;
use tokio::{
    sync::broadcast::Sender,
    task::{JoinError, JoinSet},
};

use super::log_stream::stream_log_event;
use super::network_event::NetworkEvent;
use super::new_block_stream::stream_new_blocks;
use super::pending_transactions_stream::stream_pending_transactions;

pub struct NetworkStreamsManager {
    join_set: JoinSet<()>,
    event_sender: Sender<NetworkEvent>,
}

impl NetworkStreamsManager {
    pub(super) fn new<M>(
        provider: M,
        event_sender: Sender<NetworkEvent>,
        new_blocks: bool,
        pending_transactions: Option<Vec<String>>,
        events: Option<Vec<Option<Filter>>>,
    ) -> Self
    where
        M: Middleware + Clone + 'static,
        M::Provider: PubsubClient,
    {
        let mut set: JoinSet<()> = JoinSet::new();

        if new_blocks {
            set.spawn(stream_new_blocks(provider.clone(), event_sender.clone()));
        }

        if let Some(pending_transactions) = pending_transactions {
            set.spawn(stream_pending_transactions(
                provider.clone(),
                event_sender.clone(),
                Some(pending_transactions),
            ));
        }

        if let Some(events) = events {
            for event in events {
                set.spawn(stream_log_event(provider.clone(), event_sender.clone(), event.unwrap()));
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
