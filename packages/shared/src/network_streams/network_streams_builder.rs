use ethers::types::Filter;
use ethers_providers::{Middleware, PubsubClient};
use tokio::sync::{broadcast, broadcast::Sender};

use super::{network_event::NetworkEvent, network_streams_manager::NetworkStreamsManager};

pub struct NetworkStreamManagerBuilder<M>
where
    M: Middleware + Clone,
    M::Provider: PubsubClient,
{
    provider: M,
    new_blocks: bool,
    pending_transactions: Option<Vec<String>>,
    events: Option<Vec<Option<Filter>>>,
}

impl<M> NetworkStreamManagerBuilder<M>
where
    M: Middleware + Clone + 'static,
    <M as Middleware>::Provider: PubsubClient,
{
    pub fn new(provider: M) -> Self
    where
        M: Middleware,
        M::Provider: PubsubClient,
    {
        Self {
            provider: provider.clone(),
            new_blocks: false,
            pending_transactions: None,
            events: None,
        }
    }

    pub fn watch_new_blocks(&mut self) -> &mut Self {
        self.new_blocks = true;
        self
    }

    pub fn watch_pending_transactions(&mut self, filter_to_address: Option<Vec<String>>) -> &mut Self {
        self.pending_transactions = Some(filter_to_address.unwrap_or_default());
        self
    }

    pub fn watch_event(&mut self, filter: Option<Filter>) -> &mut Self {
        if self.events.is_none() {
            self.events = Some(Vec::new());
        }

        self.events.as_mut().unwrap().push(filter);
        self
    }

    pub fn build(&self) -> NetworkStreamsManager {
        let (event_sender, _): (Sender<NetworkEvent>, _) = broadcast::channel(512);
        NetworkStreamsManager::new(
            self.provider.clone(),
            event_sender,
            self.new_blocks,
            self.pending_transactions.clone(),
            self.events.clone(),
        )
    }
}
