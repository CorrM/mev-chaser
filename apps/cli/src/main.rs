use std::env;
use std::str::FromStr;
use std::sync::Arc;
use std::{env::VarError, path::Path};

use anyhow::{anyhow, Result};
use ethers_core::types::Address;

use amm::{AmmProtocol, UniswapV2Pool, UniswapV2Protocol};
use mev::{BackRunnerStrategy, SolidityBridge};
use shared::{
    network::NetworkKind,
    provider::{NodeProvider, NodeProviderManager, NodeProviderNetworkInfo},
    token::{CryptoToken, TokenManager},
};

use commands::{AddTokenCommand, GenPoolCommand};
use database::{Database, DbDex, DbDexPool, DbToken, DbTokenNetwork};
use utils::env::Env;

mod commands;
mod database;
mod utils;

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

async fn get_debug_node_providers(env: &Env, target_network: &NetworkKind) -> Result<Vec<NodeProvider>> {
    let blockpi_network_subdomain: String = match target_network {
        NetworkKind::Ethereum => "ethereum".to_string(),
        NetworkKind::Polygon => "polygon".to_string(),
    };

    let blockpi_net_info: NodeProviderNetworkInfo = NodeProviderNetworkInfo {
        network: *target_network,
        http_url: format!(
            "https://{}.blockpi.network/v1/rpc/{}",
            blockpi_network_subdomain, env.blockpi_api_key
        )
        .to_string(),
        ws_url: format!(
            "wss://{}.blockpi.network/v1/ws/{}",
            blockpi_network_subdomain, env.blockpi_api_key
        )
        .to_string(),
    };

    Ok(vec![
        NodeProvider::new("blockpi", blockpi_net_info).await?,
    ])
}

async fn create_node_provider_manager(env: &Env, target_network: &NetworkKind) -> Result<NodeProviderManager> {
    let providers: Vec<NodeProvider> = get_node_providers(env, target_network).await?;
    NodeProviderManager::new(providers, get_debug_node_providers(env, target_network).await?)
}
*/

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

#[tokio::main]
async fn main() -> Result<()> {
    let env: Env = read_env_file()?;
    let db = Database::new(Path::new("./Main.db"))?;
    let target_network: NetworkKind = unsafe { std::mem::transmute(env.chain_id) };
    //let provider_manager: NodeProviderManager = create_node_provider_manager(&env, &target_network).await?;

    //let provider: NodeProvider = NodeProvider::new(
    //    "Local",
    //    NodeProviderNetworkInfo {
    //        network: target_network,
    //        http_url: None,
    //        ws_url: None,
    //        ipc_path: Some("/var/lib/bor/bor.ipc".to_string()),
    //    },
    //)
    //.await?;
    let provider: NodeProvider = NodeProvider::new(
        "Alchemy",
        NodeProviderNetworkInfo {
            network: target_network,
            http_url: Some(env.https_url.clone()),
            ws_url: Some(env.wss_url.clone()),
            ipc_path: None,
        },
    )
    .await?;
    let provider_manager = NodeProviderManager::new(vec![provider.clone()], vec![provider])?;

    //test_contract(&env, &provider_manager).await;
    //return Ok(());

    // CLI commands
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        if args[1] == "gen_pools" {
            GenPoolCommand::process(
                &db,
                &target_network,
                Arc::clone(provider_manager.get_next().raw_ws_provider()),
            )
            .await?;

            return Ok(());
        }

        if args[1] == "add_token" {
            let file_name: &String = &args[2];
            let tokens: String = std::fs::read_to_string(file_name).expect("Something went wrong reading the file");
            let tokens: Vec<&str> = tokens.lines().filter(|s| !s.is_empty()).collect();

            AddTokenCommand::process(
                tokens,
                &db,
                &target_network,
                Arc::clone(provider_manager.get_next().raw_http_provider()),
            )
            .await?;

            return Ok(());
        }
    }

    let token_manager = TokenManager::new(get_tokens(&db, &target_network)?, &target_network);
    let solidity_bridge = SolidityBridge::new(
        Address::from_str(&env.bot_address).unwrap(),
        Arc::clone(provider_manager.get_next().raw_ipc_provider()),
        env.private_key,
    )
    .await?;

    println!("[-] Getting amms");
    let amms: Vec<Arc<dyn AmmProtocol>> = get_dexes(&db, &target_network, &token_manager)?;

    let start_tokens: Vec<Arc<CryptoToken>> = vec![
        //token_manager.get_by_symbol("WMATIC").unwrap(), // TODO: Test => IDK but mostly it needs swapTokenForEth v2 function
        token_manager.get_by_symbol("USDT").unwrap(),
        token_manager.get_by_symbol("USDC").unwrap(),
        token_manager.get_by_symbol("DAI").unwrap(),
    ];

    // 2 are triangle arbitrage
    println!("[-] Prepare strategy");
    let mut strategy = BackRunnerStrategy::new(
        solidity_bridge,
        provider_manager.get_next().raw_ipc_provider(),
        token_manager,
        amms,
        3,
        start_tokens,
    )
    .await;

    println!("[+] Start strategy");
    strategy
        .run(
            Arc::clone(provider_manager.get_next().raw_ipc_provider()),
            Arc::clone(provider_manager.get_next_debug_trace_call().raw_ipc_provider()),
        )
        .await?;

    println!("[+] Done");

    Ok(())
}
