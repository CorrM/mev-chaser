use ethers::types::{Filter, Log, Transaction};
use std::sync::Arc;
use tokio::sync::{broadcast, broadcast::Sender};

use crate::provider::NodeProviderKind;

use super::{network_event::NetworkEvent, network_streams_manager::NetworkStreamsManager, new_block_stream::NewBlock};

pub struct NetworkStreamManagerBuilder {
    provider: Arc<NodeProviderKind>,
    events: Vec<(NetworkEvent, Option<Filter>)>,
}

impl NetworkStreamManagerBuilder {
    pub fn new(provider: &Arc<NodeProviderKind>) -> Self {
        Self {
            provider: provider.clone(),
            events: Vec::new(),
        }
    }

    pub fn watch_new_blocks(&mut self) -> &mut Self {
        let event: (NetworkEvent, Option<Filter>) = (NetworkEvent::Block(NewBlock::default()), None);
        self.events.push(event);
        self
    }

    pub fn watch_pending_transactions(&mut self) -> &mut Self {
        let event: (NetworkEvent, Option<_>) = (NetworkEvent::PendingTx(Transaction::default()), None);
        self.events.push(event);
        self
    }

    pub fn watch_event(&mut self, filter: Option<Filter>) -> &mut Self {
        let event: (NetworkEvent, Option<Filter>) = (NetworkEvent::Log(Log::default()), filter);
        self.events.push(event);
        self
    }

    pub fn build(&self) -> NetworkStreamsManager {
        let (event_sender, _): (Sender<NetworkEvent>, _) = broadcast::channel(512);
        match (*self.provider).clone() {
            NodeProviderKind::Normal(p) => NetworkStreamsManager::new(p.clone(), &self.events, event_sender),
            NodeProviderKind::DebugTraceCall(p) => NetworkStreamsManager::new(p.clone(), &self.events, event_sender),
        }
    }
}
