use crate::network::NetworkKind;

pub struct NodeProviderNetworkInfo {
    pub network: NetworkKind,
    pub http_url: String,
    pub wss_url: String,
}