use std::{str::FromStr, sync::Arc};

use anyhow::Result;
use ethers::providers::{Http, Ipc, Provider, Ws};

use crate::network::NetworkKind;

use super::NodeProviderNetworkInfo;

#[derive(Clone)]
pub struct NodeProvider {
    name: String,
    network: NetworkKind,
    http_provider: Option<Arc<Provider<Http>>>,
    ws_provider: Option<Arc<Provider<Ws>>>,
    ipc_provider: Option<Arc<Provider<Ipc>>>,
}

impl NodeProvider {
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn network(&self) -> &NetworkKind {
        &self.network
    }

    pub fn raw_http_provider(&self) -> &Arc<Provider<Http>> {
        self.http_provider.as_ref().unwrap()
    }

    pub fn raw_ws_provider(&self) -> &Arc<Provider<Ws>> {
        self.ws_provider.as_ref().unwrap()
    }

    pub fn raw_ipc_provider(&self) -> &Arc<Provider<Ipc>> {
        self.ipc_provider.as_ref().unwrap()
    }

    pub async fn new(name: impl Into<String>, network_info: NodeProviderNetworkInfo) -> Result<Self> {
        let http: Option<Provider<Http>> = network_info
            .http_url
            .map(|url| Provider::<Http>::new(Http::from_str(&url).unwrap()));
        
        let ws: Option<Provider<Ws>> = if let Some(url) = network_info.ws_url {
            Some(Provider::<Ws>::new(Ws::connect(url).await?))
        } else {
            None
        };

        let ipc: Option<Provider<Ipc>> = if let Some(path) = network_info.ipc_path {
            Some(
                Provider::<Ipc>::connect_ipc(path)
                    .await
                    .expect("Failed to connect to IPC provider"),
            )
        } else {
            None
        };

        Ok(Self {
            name: name.into(),
            network: network_info.network,
            http_provider: http.map(Arc::new),
            ws_provider: ws.map(Arc::new),
            ipc_provider: ipc.map(Arc::new),
        })
    }
}
