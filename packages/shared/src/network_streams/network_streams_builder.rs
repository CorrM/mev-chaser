use std::ops::Deref;
use std::sync::Arc;

use ethers::types::Filter;
use tokio::sync::{broadcast, broadcast::Sender};

use crate::provider::NodeProviderKind;

use super::{
    network_event::NetworkEvent, network_streams_manager::NetworkStreamsManager
    ,
};

pub struct NetworkStreamManagerBuilder {
    provider: Arc<NodeProviderKind>,
    new_blocks: bool,
    pending_transactions: Option<Vec<String>>,
    events: Option<Vec<Option<Filter>>>,
}

impl NetworkStreamManagerBuilder {
    pub fn new(provider: Arc<NodeProviderKind>) -> Self {
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
        self.pending_transactions = filter_to_address;
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
        match self.provider.deref() {
            NodeProviderKind::Normal(p) => NetworkStreamsManager::new(
                p.clone(),
                event_sender,
                self.new_blocks,
                self.pending_transactions.clone(),
                self.events.clone(),
            ),
            NodeProviderKind::DebugTraceCall(p) => NetworkStreamsManager::new(
                p.clone(),
                event_sender,
                self.new_blocks,
                self.pending_transactions.clone(),
                self.events.clone(),
            ),
        }
    }
}
