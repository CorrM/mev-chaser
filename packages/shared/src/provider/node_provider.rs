use crate::network_streams::NetworkStreamsManager;

use super::NodeProviderNetworkInfo;

pub struct NodeProvider {
    name: String,
    network_info: NodeProviderNetworkInfo,
    network_streams: NetworkStreamsManager,
}

impl NodeProvider {
    pub fn new(
        name: impl Into<String>,
        network_info: NodeProviderNetworkInfo,
        network_streams: NetworkStreamsManager,
    ) -> Self {
        Self {
            name: name.into(),
            network_info,
            network_streams,
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn network_info(&self) -> &NodeProviderNetworkInfo {
        &self.network_info
    }
}
