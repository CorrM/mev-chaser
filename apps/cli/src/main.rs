use std::env;
use std::path::Path;
use std::sync::Arc;

use anyhow::{anyhow, Result};

use commands::{AddTokenCommand, GenPoolCommand};
use shared::database::Database;
use shared::types::{NodeProvider, NodeProviderNetworkInfo};
use utilities::env::Env;
use vidger::logger::Logger;
use vidger::types::NetworkKind;

use crate::commands::RunCommand;

mod commands;
mod strategy;
mod utilities;

fn read_env_file() -> Result<Env> {
    // Env
    if dotenv::dotenv().is_err() {
        return Err(anyhow!("No .env file found"));
    }

    let var_parse: Result<Env> = Env::new();
    if var_parse.is_err() {
        return Err(anyhow!("Error while parsing .env file: {:?}", var_parse.unwrap_err()));
    }

    Ok(var_parse.unwrap())
}

/*
fn get_node_providers(env: &Env, target_network: &NetworkKind) -> Result<Vec<NodeProvider>> {
    let providers: Vec<NodeProvider> = vec![
        NodeProvider::new(
            "Alchemy",
            NodeProviderNetworkInfo {
                network: *target_network,
                http_url: env.https_url.clone(),
                ws_url: env.wss_url.clone(),
            },
        )
        .await?,
        NodeProvider::new(
            "Local",
            NodeProviderNetworkInfo {
                network: *target_network,
                http_url: env.https_url.clone(),
                ws_url: env.wss_url.clone(),
            },
        )
        .await?,
    ];

    Ok(providers)
}

fn create_node_provider_manager(env: &Env, target_network: &NetworkKind) -> Result<NodeProviderManager> {
    let providers: Vec<NodeProvider> = get_node_providers(env, target_network).await?;
    NodeProviderManager::new(providers, get_debug_node_providers(env, target_network).await?)
}
*/

fn get_node_provider(env: &Env, target_network: NetworkKind) -> Result<NodeProvider> {
    //#[cfg(debug_assertions)]
    //let provider: NodeProvider = NodeProvider::new(
    //    "Alchemy",
    //    NodeProviderNetworkInfo {
    //        network: target_network,
    //        http_url: Some(env.https_url.clone()),
    //        ws_url: Some(env.wss_url.clone()),
    //        ipc_path: None,
    //    },
    //)
    //.await?;

    #[cfg(debug_assertions)]
    let provider: NodeProvider = get_debug_node_provider(env, target_network)?;

    #[cfg(not(debug_assertions))]
    let provider: NodeProvider = NodeProvider::new(
        "Local",
        NodeProviderNetworkInfo {
            network: target_network,
            http_url: None,
            ws_url: None,
            ipc_path: Some("/var/lib/bor/bor.ipc".to_string()),
        },
    )
    .await?;

    Ok(provider)
}

fn get_debug_node_provider(env: &Env, target_network: NetworkKind) -> Result<NodeProvider> {
    let blockpi_network_subdomain: String = match target_network {
        NetworkKind::Ethereum => "ethereum".to_string(),
        NetworkKind::Polygon => "polygon".to_string(),
    };

    let blockpi_net_info: NodeProviderNetworkInfo = NodeProviderNetworkInfo {
        network: target_network,
        http_url: Some(
            format!(
                "https://{}.blockpi.network/v1/rpc/{}",
                blockpi_network_subdomain, env.blockpi_api_key
            )
            .to_string(),
        ),
        ws_url: Some(
            format!(
                "wss://{}.blockpi.network/v1/ws/{}",
                blockpi_network_subdomain, env.blockpi_api_key
            )
            .to_string(),
        ),
        ipc_path: None,
    };

    NodeProvider::new("blockpi", blockpi_net_info)
}

#[tokio::main]
async fn main() -> Result<()> {
    let Ok(_) = Logger::setup_logger() else {
        return Err(anyhow!("Failed to setup logger"));
    };

    let env: Env = read_env_file()?;
    let db = Database::new(Path::new("./Main.db"))?;
    let target_network = NetworkKind::from(env.chain_id);

    let provider: NodeProvider = get_node_provider(&env, target_network.clone())?;

    #[cfg(debug_assertions)]
    let raw_provider = Arc::clone(provider.raw_ws_provider());

    #[cfg(not(debug_assertions))]
    let raw_provider = Arc::clone(provider.raw_ipc_provider());

    /*
    let debug_provider: NodeProvider = get_debug_node_provider(&env, target_network.clone()).await?;

    #[cfg(debug_assertions)]
    let debug_raw_provider = Arc::clone(provider.raw_ws_provider());

    #[cfg(not(debug_assertions))]
    let debug_raw_provider = Arc::clone(provider.raw_ipc_provider());
    */

    // CLI commands
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "gen_pools" => {
                GenPoolCommand::process(&db, &target_network, raw_provider)?;
            }
            "add_token" => {
                let file_name: &String = &args[2];
                let tokens: String = std::fs::read_to_string(file_name).expect("Something went wrong reading the file");
                let tokens: Vec<&str> = tokens.lines().filter(|s| !s.is_empty()).collect();

                AddTokenCommand::process(tokens, &db, &target_network, raw_provider)?;
            }
            _ => panic!("Invalid command"),
        }
    } else {
        RunCommand::process(&env, db, target_network, raw_provider).await?;
    }

    Ok(())
}

/*
let swaps: Vec<OneSwapInfo> = vec![
    make_uniswap_v2_protocol_swap_info(
        Address::from_str("0xa5E0829CaCEd8fFDD4De3c43696c57F7D7A678ff").unwrap(),
        vec![
            Address::from_str("0xc2132D05D31c914a87C6611C10748AEb04B58e8F").unwrap(),
            Address::from_str("0x346404079b3792a6c548B072B9C4DDdFb92948d5").unwrap(),
        ],
        10_000_000,
        0,
    )
    .unwrap(),
    make_uniswap_v2_protocol_swap_info(
        Address::from_str("0xa5E0829CaCEd8fFDD4De3c43696c57F7D7A678ff").unwrap(),
        vec![
            Address::from_str("0x346404079b3792a6c548B072B9C4DDdFb92948d5").unwrap(),
            Address::from_str("0xc2132D05D31c914a87C6611C10748AEb04B58e8F").unwrap(),
        ],
        0,
        1_000_000,
    )
    .unwrap(),
];

for _i in 0..20 {
    let result: Result<U256> = simulator
        .eth_call_simulate_multi_swap(block.number.unwrap(), swaps.clone(), true, slot_idx)
        .await;
    info!("Result: {:?}", result);
}
*/
