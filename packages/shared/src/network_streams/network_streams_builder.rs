use ethers::{
    providers::{Provider, Ws},
    types::{Filter, Log, Transaction},
};
use std::sync::Arc;
use tokio::sync::{broadcast, broadcast::Sender};

use super::{event::Event, network_streams_manager::NetworkStreamsManager, new_block_stream::NewBlock};

pub struct NetworkStreamManagerBuilder {
    provider: Arc<Provider<Ws>>,
    events: Vec<(Event, Option<Filter>)>,
}

impl NetworkStreamManagerBuilder {
    pub fn new(provider: &Arc<Provider<Ws>>) -> Self {
        Self {
            provider: provider.clone(),
            events: Vec::new(),
        }
    }

    pub fn watch_new_blocks(&mut self) -> &mut Self {
        let event: (Event, Option<Filter>) = (Event::Block(NewBlock::default()), None);
        self.events.push(event);
        self
    }

    pub fn watch_pending_transactions(&mut self) -> &mut Self {
        let event: (Event, Option<_>) = (Event::PendingTx(Transaction::default()), None);
        self.events.push(event);
        self
    }

    pub fn watch_event(&mut self, filter: Option<Filter>) -> &mut Self {
        let event: (Event, Option<Filter>) = (Event::Log(Log::default()), filter);
        self.events.push(event);
        self
    }

    pub fn build(&mut self) -> NetworkStreamsManager {
        let (event_sender, _): (Sender<Event>, _) = broadcast::channel(512);
        NetworkStreamsManager::new(&self.provider, &self.events, event_sender)
    }
}
