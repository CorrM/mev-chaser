use std::{str::FromStr, sync::Arc};

use anyhow::Result;
use ethers::providers::{Http, Provider, Ws};

use crate::network::NetworkKind;

use super::{NodeProvider, NodeProviderNetworkInfo};

#[derive(Clone)]
pub struct NormalNodeProvider {
    name: String,
    network: NetworkKind,
    http_provider: Arc<Provider<Http>>,
    ws_provider: Arc<Provider<Ws>>,
}

impl NormalNodeProvider {
    pub async fn new(
        name: impl Into<String>,
        network_info: NodeProviderNetworkInfo,
    ) -> Result<Self> {
        let ws: Ws = Ws::connect(&network_info.wss_url).await?;
        let http: Http = Http::from_str(&network_info.http_url)?;

        Ok(Self {
            name: name.into(),
            network: network_info.network,
            ws_provider: Arc::new(Provider::new(ws)),
            http_provider: Arc::new(Provider::new(http)),
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

    fn raw_http_provider(&self) -> &Arc<Provider<Http>> {
        &self.http_provider
    }

    fn raw_ws_provider(&self) -> &Arc<Provider<Ws>> {
        &self.ws_provider
    }
}
