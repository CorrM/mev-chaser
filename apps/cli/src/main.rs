mod utils;

use anyhow::{anyhow, Result};
use std::{env::VarError, sync::Arc};
use tokio::sync::broadcast::Receiver;

use crate::utils::env::Env;
use shared::{
    network::NetworkKind,
    network_streams::{NetworkEvent, NetworkStreamManagerBuilder, NetworkStreamsManager},
    provider::{NodeProvider, NodeProviderManager, NodeProviderNetworkInfo},
};

#[tokio::main]
async fn main() -> Result<()> {
    // Env
    if dotenv::dotenv().is_err() {
        return Err(anyhow!("No .env file found"));
    }

    let var_parse: Result<Env, VarError> = Env::new();
    if var_parse.is_err() {
        return Err(anyhow!("Error while parsing .env file: {:?}", var_parse.unwrap_err()));
    }

    let env: Env = var_parse.unwrap();

    // Node provider
    let target_network: NetworkKind = unsafe { ::std::mem::transmute(env.chain_id) };
    let providers: Vec<NodeProvider> = vec![
        NodeProvider::new(
            "Infura",
            NodeProviderNetworkInfo {
                network: target_network.clone(),
                http_url: env.https_url.clone(),
                wss_url: env.wss_url.clone(),
            },
        ).await?,
        NodeProvider::new(
            "Alchemy",
            NodeProviderNetworkInfo {
                network: target_network.clone(),
                http_url: env.https_url.clone(),
                wss_url: env.wss_url.clone(),
            },
        ).await?,
    ];
    let provider_manager = NodeProviderManager::new(providers)?;

    let provider: &Arc<NodeProvider> = provider_manager.get_next();
    let ns: NetworkStreamsManager = NetworkStreamManagerBuilder::new(provider)
        //.watch_new_blocks()
        .watch_pending_transactions()
        //.watch_log("Sync(uint112,uint112)")
        .build();

    //let dex = Arc::new(UniswapV2Protocol::new("UniswapV2"));

    let mut event_receiver: Receiver<NetworkEvent> = ns.subscribe();
    while let Ok(event) = event_receiver.recv().await {
        println!("{:?}", event);
    }
    //sb.0.spawn(event_handler(provider.clone(), sb.1.clone()));

    //while let Some(res) = ns.join_next().await {
    //    println!("{:?}", res);
    //}

    Ok(())
}
