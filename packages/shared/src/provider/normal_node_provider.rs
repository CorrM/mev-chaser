use anyhow::Result;
use ethers::providers::{Http, Provider, Ws};
use std::str::FromStr;

use crate::network::NetworkKind;

use super::{NodeProvider, NodeProviderNetworkInfo};

#[derive(Clone)]
pub struct NormalNodeProvider {
    name: String,
    network: NetworkKind,
    http_provider: Provider<Http>,
    ws_provider: Provider<Ws>,
}

impl NormalNodeProvider {
    pub async fn new(name: impl Into<String>, network_info: NodeProviderNetworkInfo) -> Result<Self> {
        let ws: Ws = Ws::connect(&network_info.wss_url).await?;
        let http: Http = Http::from_str(&network_info.http_url)?;

        Ok(Self {
            name: name.into(),
            network: network_info.network,
            ws_provider: Provider::new(ws),
            http_provider: Provider::new(http),
        })
    }
}

impl NodeProvider for NormalNodeProvider {
    fn name(&self) -> &str {
        self.name.as_ref()
    }

    fn network(&self) -> &NetworkKind {
        &self.network
    }

    fn raw_http_provider(&self) -> &Provider<Http> {
        &self.http_provider
    }

    fn raw_ws_provider(&self) -> &Provider<Ws> {
        &self.ws_provider
    }
}