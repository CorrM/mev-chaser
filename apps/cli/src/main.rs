use std::env;
use std::sync::Arc;
use std::{env::VarError, path::Path};

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

    let var_parse: Result<Env, VarError> = Env::new();
    if var_parse.is_err() {
        return Err(anyhow!("Error while parsing .env file: {:?}", var_parse.unwrap_err()));
    }

    Ok(var_parse.unwrap())
}

/*
async fn get_node_providers(env: &Env, target_network: &NetworkKind) -> Result<Vec<NodeProvider>> {
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

async fn create_node_provider_manager(env: &Env, target_network: &NetworkKind) -> Result<NodeProviderManager> {
    let providers: Vec<NodeProvider> = get_node_providers(env, target_network).await?;
    NodeProviderManager::new(providers, get_debug_node_providers(env, target_network).await?)
}

fn get_tokens(db: &Database, network: &NetworkKind) -> Result<Vec<CryptoToken>> {
    let db_tokens: Vec<(DbToken, DbTokenNetwork)> = db.get_tokens(network)?;
    let mut tokens: Vec<CryptoToken> = Vec::new();

    for (db_token, db_token_network) in db_tokens {
        tokens.push(CryptoToken::new(
            network,
            db_token_network.address,
            db_token.name,
            db_token.symbol,
            db_token.decimals as u8,
        )?);
    }

    Ok(tokens)
}

fn get_dexes(db: &Database, network: &NetworkKind, token_manager: &TokenManager) -> Result<Vec<Arc<dyn AmmProtocol>>> {
    let mut dexes: Vec<Arc<dyn AmmProtocol>> = Vec::new();

    let db_dexes: Vec<DbDex> = db.get_dexes_by_network(network)?;
    for db_dex in db_dexes {
        let Some(db_dex_protocol) = db.get_dex_protocol_by_id(db_dex.dex_protocol_id)? else {
            continue;
        };

        let Some(db_dex_network) = db.get_dex_network(db_dex.id, network)? else {
            continue;
        };

        let db_dex_pools: Vec<DbDexPool> = db.get_dex_pools_by_dex_id(db_dex.id, network, true)?;
        match db_dex_protocol.name.as_str() {
            "UniswapV2" => {
                let network_options: serde_json::Value = serde_json::from_str(&db_dex_network.options)?;
                let dex_options: serde_json::Value = serde_json::from_str(&db_dex.options)?;

                let mut uniswap_v2 = UniswapV2Protocol::new(
                    db_dex.name.clone(),
                    dex_options["fees"].as_u64().unwrap() as u32,
                    network_options["factory"].as_str().unwrap(),
                    network_options["router"].as_str().unwrap(),
                )?;

                for db_dex_pool in db_dex_pools {
                    let pool_address: Address = db_dex_pool.address.parse::<Address>()?;
                    if pool_address.is_zero() {
                        continue;
                    }

                    let token0: Option<DbToken> = db.get_token_by_id(db_dex_pool.token0_id)?;
                    let token1: Option<DbToken> = db.get_token_by_id(db_dex_pool.token1_id)?;
                    if token0.is_none() || token1.is_none() {
                        return Err(anyhow!("Token not found"));
                    }

                    let db_token0_network: DbTokenNetwork =
                        db.get_token_network_by_token(token0.unwrap().id, network)?.unwrap();
                    let db_token1_network: DbTokenNetwork =
                        db.get_token_network_by_token(token1.unwrap().id, network)?.unwrap();

                    let token0: Arc<CryptoToken> =
                        token_manager.get_by_address_str(&db_token0_network.address).unwrap();
                    let token1: Arc<CryptoToken> =
                        token_manager.get_by_address_str(&db_token1_network.address).unwrap();

                    uniswap_v2.add_pool(UniswapV2Pool::new(
                        pool_address,
                        Arc::new(uniswap_v2.clone()),
                        *network,
                        token0,
                        token1,
                    )?);
                }

                dexes.push(Arc::new(uniswap_v2));
            }
            _ => panic!("Unsupported dex protocol"),
        }
    }

    Ok(dexes)
}
*/

async fn get_node_provider(env: &Env, target_network: NetworkKind) -> Result<NodeProvider> {
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
    let provider: NodeProvider = get_debug_node_provider(env, target_network).await?;

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

async fn get_debug_node_provider(env: &Env, target_network: NetworkKind) -> Result<NodeProvider> {
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

    NodeProvider::new("blockpi", blockpi_net_info).await
}

#[tokio::main]
async fn main() -> Result<()> {
    let Ok(_) = Logger::setup_logger() else {
        return Err(anyhow!("Failed to setup logger"));
    };

    let env: Env = read_env_file()?;
    let db = Database::new(Path::new("./Main.db"))?;
    let target_network = NetworkKind::from(env.chain_id);

    let provider: NodeProvider = get_node_provider(&env, target_network.clone()).await?;

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
                GenPoolCommand::process(&db, &target_network, raw_provider).await?;
            }
            "add_token" => {
                let file_name: &String = &args[2];
                let tokens: String = std::fs::read_to_string(file_name).expect("Something went wrong reading the file");
                let tokens: Vec<&str> = tokens.lines().filter(|s| !s.is_empty()).collect();

                AddTokenCommand::process(tokens, &db, &target_network, raw_provider).await?;
            }
            _ => panic!("Invalid command"),
        }
    } else {
        RunCommand::process(&env, raw_provider).await?;
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
