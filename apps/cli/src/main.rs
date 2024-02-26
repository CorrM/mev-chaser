use std::env;
use std::str::FromStr;
use std::sync::Arc;
use std::{env::VarError, path::Path};

use anyhow::{anyhow, Result};
use ethers::prelude::H256;
use ethers::types::U256;
use ethers_core::types::{Address, Block, BlockNumber, spoof, TransactionRequest};
use ethers_core::utils::Geth;
use ethers_providers::{Http, Middleware, Provider, RawCall, Ws};

use amm::{AmmProtocol, UniswapV2Pool, UniswapV2Protocol};
use commands::{AddTokenCommand, GenPoolCommand};
use contracts::balancer_flash_loan_recipient::OneSwapInfo;
use database::{Database, DbDex, DbDexPool, DbToken, DbTokenNetwork};
use evm_simulator::EvmSimulator;
use mev::{make_uniswap_v2_protocol_swap_info, BackRunnerStrategy, SolidityBridge};
use shared::logger::{error, info, Logger};
use shared::{
    network::NetworkKind,
    provider::{NodeProvider, NodeProviderManager, NodeProviderNetworkInfo},
    token::{CryptoToken, TokenManager},
};
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

async fn create_node_provider_manager(env: &Env, target_network: &NetworkKind) -> Result<NodeProviderManager> {
    let providers: Vec<NodeProvider> = get_node_providers(env, target_network).await?;
    NodeProviderManager::new(providers, get_debug_node_providers(env, target_network).await?)
}
*/

async fn get_debug_node_providers(env: &Env, target_network: &NetworkKind) -> Result<Vec<NodeProvider>> {
    let blockpi_network_subdomain: String = match target_network {
        NetworkKind::Ethereum => "ethereum".to_string(),
        NetworkKind::Polygon => "polygon".to_string(),
    };

    let blockpi_net_info: NodeProviderNetworkInfo = NodeProviderNetworkInfo {
        network: *target_network,
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

    Ok(vec![NodeProvider::new("blockpi", blockpi_net_info).await?])
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

#[tokio::main]
async fn main() -> Result<()> {
    let env: Env = read_env_file()?;
    let db = Database::new(Path::new("./Main.db"))?;
    let target_network = NetworkKind::from(env.chain_id);
    //let provider_manager: NodeProviderManager = create_node_provider_manager(&env, &target_network).await?;

    #[cfg(debug_assertions)]
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

    let provider_manager = NodeProviderManager::new(vec![provider.clone()], vec![provider])?;

    #[cfg(debug_assertions)]
    let raw_provider = Arc::clone(provider_manager.get_next().raw_ws_provider());

    #[cfg(not(debug_assertions))]
    let raw_provider = Arc::clone(provider_manager.get_next().raw_ipc_provider());

    let Ok(_) = Logger::setup_logger() else {
        return Err(anyhow!("Failed to setup logger"));
    };

    let debug_provider: Arc<Provider<Ws>> =
        Arc::clone(get_debug_node_providers(&env, &target_network).await?[0].raw_ws_provider());
    let simulator = EvmSimulator::new(debug_provider).await;

    let user = Address::from_str("0x9cf277A22EB4c551c6E18F7a6C0ee1893bcB034f").unwrap();
    let weth = Address::from_str("0x7ceb23fd6bc0add59e62ac25578270cff1b9f619").unwrap();
    let usdc = Address::from_str("0x3c499c542cef5e3811e1192ce70d8cc03d5c3359").unwrap();
    let usdt = Address::from_str("0xc2132D05D31c914a87C6611C10748AEb04B58e8F").unwrap();

    let block: Block<H256> = raw_provider
        .get_block(BlockNumber::Latest)
        .await?
        .ok_or(anyhow!("failed to retrieve block"))?;

    //let weth_balance: Result<U256> = simulator.get_token_balance(weth, user);
    //info!("WETH balance: {:?}", weth_balance);

    //let slot_idx: i32 = match simulator.get_token_balance_slot(weth, user).await {
    //    Ok(idx) => idx,
    //    Err(e) => panic!("Tracing error: {e:?}"),
    //};

    let slot_idx: i32 = 0;
    info!("Balance storage slot: {:?}", slot_idx);

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

    match simulator.get_proxy_implementation(usdc, block.number.unwrap()).await {
        Ok(implementation) => info!("Proxy implementation: {:?}", implementation),
        Err(e) => error!("Proxy implementation error: {e:?}"),
    }

    match simulator.get_proxy_implementation(usdt, block.number.unwrap()).await {
        Ok(implementation) => info!("Proxy implementation: {:?}", implementation),
        Err(e) => error!("Proxy implementation error: {e:?}"),
    }

    return Ok(());

    // CLI commands
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        if args[1] == "gen_pools" {
            GenPoolCommand::process(&db, &target_network, raw_provider).await?;

            return Ok(());
        }

        if args[1] == "add_token" {
            let file_name: &String = &args[2];
            let tokens: String = std::fs::read_to_string(file_name).expect("Something went wrong reading the file");
            let tokens: Vec<&str> = tokens.lines().filter(|s| !s.is_empty()).collect();

            AddTokenCommand::process(tokens, &db, &target_network, raw_provider).await?;

            return Ok(());
        }
    }

    let token_manager = TokenManager::new(get_tokens(&db, &target_network)?, &target_network);
    let solidity_bridge = SolidityBridge::new(
        Address::from_str(&env.bot_address).unwrap(),
        Arc::clone(&raw_provider),
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
    let mut strategy =
        BackRunnerStrategy::new(solidity_bridge, &raw_provider, token_manager, amms, 3, start_tokens).await;

    println!("[+] Start strategy");

    #[cfg(debug_assertions)]
    let debug_raw_provider = provider_manager.get_next_debug_trace_call().raw_ws_provider();

    #[cfg(not(debug_assertions))]
    let debug_raw_provider = provider_manager.get_next_debug_trace_call().raw_ipc_provider();

    strategy.run(raw_provider, Arc::clone(debug_raw_provider)).await?;

    println!("[+] Done");

    Ok(())
}
