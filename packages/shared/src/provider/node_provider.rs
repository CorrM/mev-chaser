use anyhow::Result;
use ethers::providers::{Http, Provider, Ws};
use std::{str::FromStr, sync::Arc};

use crate::network::NetworkKind;

use super::NodeProviderNetworkInfo;

pub struct NodeProvider {
    name: String,
    network: NetworkKind,
    http_provider: Provider<Http>,
    ws_provider: Provider<Ws>,
}

impl NodeProvider {
    pub async fn new(name: impl Into<String>, network_info: NodeProviderNetworkInfo) -> Result<Self> {
        let ws: Ws = Ws::connect(network_info.wss_url).await?;
        let http: Http = Http::from_str(&network_info.http_url)?;

        Ok(Self {
            name: name.into(),
            network: network_info.network,
            ws_provider: Provider::new(ws),
            http_provider: Provider::new(http),
        })
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
    
    pub fn network(&self) -> &NetworkKind {
        &self.network
    }

    pub fn http_provider(&self) -> &Provider<Http> {
        &self.http_provider
    }

    pub fn ws_provider(&self) -> &Provider<Ws> {
        &self.ws_provider
    }

    pub fn http_provider_arc(&self) -> Arc<Provider<Http>> {
        Arc::new(self.http_provider.clone())
    }

    pub fn ws_provider_arc(&self) -> Arc<Provider<Ws>> {
        Arc::new(self.ws_provider.clone())
    }
}
