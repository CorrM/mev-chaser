use std::sync::Arc;

use ethers::{
    providers::{Provider, Ws},
    types::Filter,
};
use tokio::sync::broadcast::Receiver;
use tokio::{
    sync::broadcast::Sender,
    task::{JoinError, JoinSet},
};

use super::event::Event;
use super::log_stream::stream_log_event;
use super::new_block_stream::stream_new_blocks;
use super::pending_transactions_stream::stream_pending_transactions;

pub struct NetworkStreamsManager {
    join_set: JoinSet<()>,
    event_sender: Sender<Event>,
}

impl NetworkStreamsManager {
    pub(super) fn new(
        provider: &Arc<Provider<Ws>>,
        events: &Vec<(Event, Option<Filter>)>,
        event_sender: Sender<Event>,
    ) -> Self {
        let mut set: JoinSet<()> = JoinSet::new();

        for (event, opt) in events {
            match event {
                Event::Block(_) => {
                    set.spawn(stream_new_blocks(provider.clone(), event_sender.clone()));
                }
                Event::PendingTx(_) => {
                    set.spawn(stream_pending_transactions(provider.clone(), event_sender.clone()));
                }
                Event::Log(_) => {
                    set.spawn(stream_log_event(
                        provider.clone(),
                        event_sender.clone(),
                        opt.clone().unwrap(),
                    ));
                }
            }
        }

        Self {
            join_set: JoinSet::new(),
            event_sender,
        }
    }

    pub async fn wait(&mut self) -> Option<Result<(), JoinError>> {
        self.join_set.join_next().await
    }

    pub fn subscribe(&self) -> Receiver<Event> {
        self.event_sender.subscribe()
    }
}
