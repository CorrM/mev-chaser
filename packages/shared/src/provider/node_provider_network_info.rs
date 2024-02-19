use crate::network::NetworkKind;

pub struct NodeProviderNetworkInfo {
    pub network: NetworkKind,
    pub http_url: Option<String>,
    pub ws_url: Option<String>,
    pub ipc_path: Option<String>,
}
