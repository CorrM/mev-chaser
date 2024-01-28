mod log_stream;
mod new_block_stream;
mod pending_transactions_stream;

use crate::network_streams::new_block_stream::{stream_new_blocks, NewBlock};
use crate::network_streams::pending_transactions_stream::stream_pending_transactions;
use ethers::{
    providers::{Provider, Ws},
    types::{Log, Transaction},
};
use tokio::sync::broadcast::Receiver;
use std::sync::Arc;
use tokio::{
    sync::{broadcast, broadcast::Sender},
    task::{JoinError, JoinSet},
};

use self::log_stream::stream_log_event;

#[derive(Debug, Clone)]
pub enum Event {
    Block(NewBlock),
    PendingTx(Transaction),
    Log(Log),
}

pub struct NetworkStreamManager {
    join_set: JoinSet<()>,
    event_sender: Sender<Event>,
}

impl NetworkStreamManager {
    pub async fn wait(&mut self) -> Option<Result<(), JoinError>> {
        self.join_set.join_next().await
    }

    pub fn subscribe(&self) -> Receiver<Event> {
        self.event_sender.subscribe()
    }
}

pub struct NetworkStreamManagerBuilder {
    provider: Arc<Provider<Ws>>,
    events: Vec<(Event, Option<String>)>,
}

impl NetworkStreamManagerBuilder {
    pub fn new(provider: &Arc<Provider<Ws>>) -> Self {
        Self {
            provider: provider.clone(),
            events: Vec::new(),
        }
    }

    pub fn watch_new_blocks(&mut self) -> &mut Self {
        let event: (Event, Option<String>) = (Event::Block(NewBlock::default()), None);
        self.events.push(event);
        self
    }

    pub fn watch_pending_transactions(&mut self) -> &mut Self {
        let event: (Event, Option<_>) = (Event::PendingTx(Transaction::default()), None);
        self.events.push(event);
        self
    }

    pub fn watch_log(&mut self, event_signature: impl Into<String>) -> &mut Self {
        let event: (Event, Option<String>) = (Event::Log(Log::default()), Some(event_signature.into()));
        self.events.push(event);
        self
    }

    pub fn build(&mut self) -> NetworkStreamManager {
        let (event_sender, _): (Sender<Event>, _) = broadcast::channel(512);
        let mut set: JoinSet<()> = JoinSet::new();

        for (event, opt) in &self.events {
            match event {
                Event::Block(_) => {
                    set.spawn(stream_new_blocks(self.provider.clone(), event_sender.clone()));
                }
                Event::PendingTx(_) => {
                    set.spawn(stream_pending_transactions(self.provider.clone(), event_sender.clone()));
                }
                Event::Log(_) => {
                    set.spawn(stream_log_event(
                        self.provider.clone(),
                        opt.clone().unwrap(),
                        event_sender.clone(),
                    ));
                }
            }
        }

        NetworkStreamManager {
            join_set: set,
            event_sender,
        }
    }
}
