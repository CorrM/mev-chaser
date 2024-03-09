use std::{str::FromStr, sync::Arc};

use anyhow::Result;
use ethers::providers::{Http, Ipc, Provider, Ws};
use futures::executor::block_on;

use vidger::types::NetworkKind;

pub struct NodeProviderNetworkInfo {
    pub network: NetworkKind,
    pub http_url: Option<String>,
    pub ws_url: Option<String>,
    pub ipc_path: Option<String>,
}

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

    pub fn new(name: impl Into<String>, network_info: NodeProviderNetworkInfo) -> Result<Self> {
        let http: Option<Provider<Http>> = network_info
            .http_url
            .map(|url| Provider::<Http>::new(Http::from_str(&url).unwrap()));

        let ws: Option<Provider<Ws>> = if let Some(url) = network_info.ws_url {
            let result: Ws = block_on(Ws::connect(url))?;
            Some(Provider::<Ws>::new(result))
        } else {
            None
        };

        let ipc: Option<Provider<Ipc>> = network_info.ipc_path.map(|path: String| {
            block_on(Provider::<Ipc>::connect_ipc(path)).expect("Failed to connect to IPC provider")
        });

        Ok(Self {
            name: name.into(),
            network: network_info.network,
            http_provider: http.map(Arc::new),
            ws_provider: ws.map(Arc::new),
            ipc_provider: ipc.map(Arc::new),
        })
    }
}
