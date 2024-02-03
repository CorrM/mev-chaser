use amm::{AmmPool, AmmProtocol, AmmProtocolKind, UniswapV2Pool, UniswapV2Protocol};
use anyhow::{anyhow, Result};
use contracts::{UniswapV2FactoryAbi, UniswapV2PairAbi};
use database::{Database, DbDex, DbDexNetwork, DbDexPool, DbDexProtocol, DbToken, DbTokenNetwork};
use ethers_core::types::Address;

use ethers_core::utils::to_checksum;
use mev::BackRunnerStragegy;
use shared::provider::NodeProvider;
use std::ops::Deref;
use std::sync::Arc;
use std::{env::VarError, path::Path};

use shared::token::{CryptoToken, TokenManager};
use shared::{
    network::NetworkKind,
    provider::{DebugTraceCallNodeProvider, NodeProviderManager, NodeProviderNetworkInfo, NormalNodeProvider},
};

use crate::utils::env::Env;

mod database;
mod utils;

fn generate_pairs<T>(list: &[T]) -> Vec<(&T, &T)> {
    let mut pairs: Vec<(&T, &T)> = Vec::new();
    for (i, fst) in list.iter().enumerate() {
        for snd in list.iter().skip(i + 1) {
            pairs.push((fst, snd));
        }
    }
    pairs
}

fn get_env() -> Result<Env> {
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

async fn get_node_providers(env: &Env, target_network: &NetworkKind) -> Result<Vec<NormalNodeProvider>> {
    let providers: Vec<NormalNodeProvider> = vec![
        NormalNodeProvider::new(
            "Alchemy",
            NodeProviderNetworkInfo {
                network: *target_network,
                http_url: env.https_url.clone(),
                wss_url: env.wss_url.clone(),
            },
        )
        .await?,
        NormalNodeProvider::new(
            "Infura",
            NodeProviderNetworkInfo {
                network: *target_network,
                http_url: env.https_url.clone(),
                wss_url: env.wss_url.clone(),
            },
        )
        .await?,
    ];

    Ok(providers)
}

async fn get_debug_node_providers(env: &Env, target_network: &NetworkKind) -> Result<Vec<DebugTraceCallNodeProvider>> {
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

async fn create_node_provider_manager(env: &Env, target_network: &NetworkKind) -> Result<NodeProviderManager> {
    let providers: Vec<NormalNodeProvider> = get_node_providers(env, target_network).await?;
    NodeProviderManager::new(providers, get_debug_node_providers(env, target_network).await?)
}

fn get_tokens(db: &Database, network: &NetworkKind) -> Result<Vec<CryptoToken>> {
    let db_tokens: Vec<(database::DbToken, database::DbTokenNetwork)> = db.get_tokens(network)?;
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

async fn get_amms(
    db: &Database,
    network: &NetworkKind,
    provider: &impl NodeProvider,
    token_manager: &TokenManager,
) -> Result<Vec<AmmProtocolKind>> {
    let pairs: Vec<(&Arc<CryptoToken>, &Arc<CryptoToken>)> = generate_pairs(token_manager.tokens());
    let mut amms: Vec<AmmProtocolKind> = Vec::new();

    let db_dexes: Vec<DbDex> = db.get_dexes_by_network(network)?;
    for db_dex in &db_dexes {
        let db_dex_protocol: Option<DbDexProtocol> = db.get_dex_protocol_by_id(db_dex.dex_protocol_id)?;
        if db_dex_protocol.is_none() {
            continue;
        }
        let db_dex_protocol: DbDexProtocol = db_dex_protocol.unwrap();

        match db_dex_protocol.name.as_str() {
            "UniswapV2" => {
                let db_dex_network = db.get_dex_network(db_dex.id, network)?;
                if db_dex_network.is_none() {
                    continue;
                }
                let db_dex_network: DbDexNetwork = db_dex_network.unwrap();
                let network_options: serde_json::Value = serde_json::from_str(&db_dex_network.options)?;

                let dex_options: serde_json::Value = serde_json::from_str(&db_dex.options)?;
                let mut uniswap_v2 = UniswapV2Protocol::new(
                    db_dex.name.clone(),
                    dex_options["fees"].as_u64().unwrap() as u32,
                    network_options["factory"].as_str().unwrap(),
                    network_options["router"].as_str().unwrap(),
                )?;
                let factory_contract =
                    UniswapV2FactoryAbi::new(*uniswap_v2.factory(), provider.raw_http_provider().clone());

                for (token_a, token_b) in &pairs {
                    let db_pool: Option<DbDexPool> = db.get_dex_pool_by_tokens(db_dex.id, network, token_a, token_b)?;
                    if let Some(db_pool) = db_pool {
                        let pool_address: Address = db_pool.address.parse::<Address>()?;

                        let token0: Option<DbToken> = db.get_token_by_id(db_pool.token0_id)?;
                        let token1: Option<DbToken> = db.get_token_by_id(db_pool.token1_id)?;
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
                        continue;
                    }

                    let pool_address: Address = factory_contract
                        .get_pair(*token_a.address(), *token_b.address())
                        .call_raw()
                        .await?;

                    if pool_address.is_zero() {
                        let db_dex: DbDex = db.get_dex_by_name(uniswap_v2.name())?.unwrap();

                        if db.add_dex_pool_empty(db_dex.id, network, token_a, token_b).is_err() {
                            panic!("Failed to add dex empty pool");
                        };
                        continue;
                    }

                    let pair_contract = UniswapV2PairAbi::new(pool_address, provider.raw_http_provider().clone());
                    let token0: Address = pair_contract.token_0().call_raw().await?;
                    let token1: Address = pair_contract.token_1().call_raw().await?;

                    let token0: Arc<CryptoToken> = token_manager.get_by_address(&token0).unwrap();
                    let token1: Arc<CryptoToken> = token_manager.get_by_address(&token1).unwrap();

                    let pool: UniswapV2Pool =
                        UniswapV2Pool::new(pool_address, Arc::new(uniswap_v2.clone()), *network, token0, token1)?;

                    if db.add_dex_pool(&pool).is_err() {
                        panic!("Failed to add dex pool {}", to_checksum(pool.address(), None));
                    };
                    uniswap_v2.add_pool(pool);
                }

                amms.push(AmmProtocolKind::UniswapV2(uniswap_v2));
            }
            _ => panic!("Unsupported dex protocol"),
        }
    }

    Ok(amms)
}

#[tokio::main]
async fn main() -> Result<()> {
    let env: Env = get_env()?;
    let db = Database::new(Path::new("./Main.db"))?;
    let target_network: NetworkKind = unsafe { std::mem::transmute(env.chain_id) };
    let provider_manager: NodeProviderManager = create_node_provider_manager(&env, &target_network).await?;
    let token_manager: TokenManager = TokenManager::new(get_tokens(&db, &target_network)?);

    let amms: Vec<AmmProtocolKind> = get_amms(
        &db,
        &target_network,
        provider_manager.get_next().deref(),
        &token_manager,
    )
    .await?;

    let start_tokens: Vec<Arc<CryptoToken>> = vec![
        token_manager.get_by_symbol("WETH").unwrap(),
        token_manager.get_by_symbol("WMATIC").unwrap(),
        token_manager.get_by_symbol("USDT").unwrap(),
        token_manager.get_by_symbol("USDC").unwrap(),
        token_manager.get_by_symbol("DAI").unwrap(),
    ];

    // 2 are traingle arbitrage
    let strategy = BackRunnerStragegy::new(provider_manager, amms, 2, start_tokens);
    strategy.run().await?;

    Ok(())
}
