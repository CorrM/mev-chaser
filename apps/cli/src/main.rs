use std::{env::VarError, ops::Deref, path::Path, str::FromStr, sync::Arc};

use anyhow::{anyhow, Result};
use ethers_core::{
    abi::{AbiEncode, Log},
    types::H160,
};
use tokio::sync::broadcast::Receiver;

use amm_protocol::{AmmProtocolKind, UniswapV2Protocol};
use shared::{
    abi::ABI,
    network::NetworkKind,
    network_streams::{NetworkEvent, NetworkStreamManagerBuilder, NetworkStreamsManager},
    provider::{
        DebugTraceCallNodeProvider, NodeProviderKind, NodeProviderManager, NodeProviderNetworkInfo,
        NormalNodeProvider,
    },
    trace::{get_trace_all_logs, TraceLogData},
};

use crate::utils::env::Env;

mod utils;

async fn get_node_providers(
    env: &Env,
    target_network: &NetworkKind,
) -> Result<Vec<NormalNodeProvider>> {
    let providers: Vec<NormalNodeProvider> = vec![
        NormalNodeProvider::new(
            "Alchemy",
            NodeProviderNetworkInfo {
                network: target_network.clone(),
                http_url: env.https_url.clone(),
                wss_url: env.wss_url.clone(),
            },
        )
            .await?,
        NormalNodeProvider::new(
            "Infura",
            NodeProviderNetworkInfo {
                network: target_network.clone(),
                http_url: env.https_url.clone(),
                wss_url: env.wss_url.clone(),
            },
        )
            .await?,
    ];

    Ok(providers)
}

async fn get_debug_trace_call_node_providers(
    env: &Env,
    target_network: &NetworkKind,
) -> Result<Vec<DebugTraceCallNodeProvider>> {
    let blockpi_network_subdomain: String = match target_network {
        NetworkKind::Ethereum => "ethereum".to_string(),
        NetworkKind::Polygon => "polygon".to_string(),
    };

    let blockpi_net_info: NodeProviderNetworkInfo = NodeProviderNetworkInfo {
        network: target_network.clone(),
        http_url: format!(
            "https://{}.blockpi.network/v1/rpc/{}",
            blockpi_network_subdomain, env.blockpi_api_key
        )
            .to_string(),
        wss_url: format!(
            "wss://{}.blockpi.network/v1/ws/{}",
            blockpi_network_subdomain, env.blockpi_api_key
        )
            .to_string(),
    };

    Ok(vec![
        DebugTraceCallNodeProvider::new("blockpi", blockpi_net_info).await?,
    ])
}

fn get_amms() -> Vec<AmmProtocolKind> {
    vec![AmmProtocolKind::UniswapV2(UniswapV2Protocol::new(
        "SushiSwap",
        300,
    ))]
}

async fn create_node_provider_manager(
    env: &Env,
    target_network: &NetworkKind,
) -> Result<NodeProviderManager> {
    let providers: Vec<NormalNodeProvider> = get_node_providers(env, target_network).await?;
    let debug_trace_call_providers: Vec<DebugTraceCallNodeProvider> =
        get_debug_trace_call_node_providers(env, target_network).await?;
    NodeProviderManager::new(providers, debug_trace_call_providers)
}

#[tokio::main]
async fn main() -> Result<()> {
    // Env
    if dotenv::dotenv().is_err() {
        return Err(anyhow!("No .env file found"));
    }

    let var_parse: Result<Env, VarError> = Env::new();
    if var_parse.is_err() {
        return Err(anyhow!(
            "Error while parsing .env file: {:?}",
            var_parse.unwrap_err()
        ));
    }

    let env: Env = var_parse.unwrap();
    let target_network: NetworkKind = unsafe { ::std::mem::transmute(env.chain_id) };
    let provider_manager: NodeProviderManager =
        create_node_provider_manager(&env, &target_network).await?;

    let abi = ABI::new(Path::new("./abi"));

    let provider: &Arc<NodeProviderKind> = &Arc::new(NodeProviderKind::Normal(
        provider_manager.get_next().deref().clone(),
    ));

    let router_addresses: Vec<String> = vec![
        "0x1b02dA8Cb0d097eB8D57A175b88c7D8b47997506".to_string(),
        "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D".to_string(),
    ];

    let ns: NetworkStreamsManager = NetworkStreamManagerBuilder::new(provider.clone())
        //.watch_new_blocks()
        .watch_pending_transactions(Some(router_addresses))
        //.watch_log("Sync(uint112,uint112)")
        .build();

    let filters: Vec<H160> = vec![
        H160::from_str("0x1b02dA8Cb0d097eB8D57A175b88c7D8b47997506").unwrap(),
        H160::from_str("0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D").unwrap(),
    ];

    let debug_provider: &Arc<DebugTraceCallNodeProvider> =
        provider_manager.get_next_debug_trace_call();
    let mut event_receiver: Receiver<NetworkEvent> = ns.subscribe();

    while let Ok(event) = event_receiver.recv().await {
        if let NetworkEvent::PendingTx(tx) = &event {
            if let Some(to) = tx.to {
                if !filters.iter().any(|&f| f == to) {
                    continue;
                }

                let tx_hash: String = tx.hash.encode_hex();
                println!("decoded_log_tx: {}", tx_hash);

                let trace_logs: Vec<TraceLogData> =
                    get_trace_all_logs(debug_provider.debug_trace_call(tx.clone(), None).await?);

                for trace_log in trace_logs {
                    let decoded_log: Vec<(String, Log)> =
                        UniswapV2Protocol::decode_trace_pair_logs(&abi.uniswap_v2_pair, trace_log);

                    println!("decoded_log: {:#?}", decoded_log);
                }

                println!("=============");
            }
        }
    }
    //sb.0.spawn(event_handler(provider.clone(), sb.1.clone()));

    //while let Some(res) = ns.join_next().await {
    //    println!("{:?}", res);
    //}

    Ok(())
}
