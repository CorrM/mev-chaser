use ethers::providers::{Http, Provider, Ws};

use crate::network::NetworkKind;

pub trait NodeProvider: Sync + Send + Clone {
    fn name(&self) -> &str;
    fn network(&self) -> &NetworkKind;
    fn raw_http_provider(&self) -> &Provider<Http>;
    fn raw_ws_provider(&self) -> &Provider<Ws>;
    //fn raw_ipc_provider(&self) -> &Provider<Ipc> {
    //    let path = "/home/user/.local/share/reth/reth.ipc";
    //    let ipc = Ipc::connect(path).await?;
    //}
}
