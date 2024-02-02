use amm::{AmmPool, AmmProtocolKind, UniswapV2Pool, UniswapV2Protocol};
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

use shared::token::CryptoToken;
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

/*
async fn get_tokens(db: &Database, provider: &impl NodeProvider, erc20_abi: &Abi) -> Result<Vec<CryptoToken>> {
    // TODO: depend on network
    let tokens = vec![
        "0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270",
        "0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270",
    ];

    let client: Provider<Http> = provider.raw_http_provider().clone();
    let client: Arc<Provider<Http>> = Arc::new(client);

    let mut multicall: Multicall<Provider<Http>> = Multicall::new(client.clone(), None).await?;
    for token in &tokens {
        let contract = Contract::<Provider<Http>>::new(
            Address::from_str(token).unwrap(),
            erc20_abi.clone(),
            client.clone(),
        );

        let call = contract.method::<_, String>("name", ())?;
        multicall.add_call(call, false);
    }

    let mut reserves = HashMap::new();

    let result: Vec<Result<Token, Bytes>> = multicall.call_raw().await?;
    for i in 0..result.len() {
        let pool = tokens[i];
        let reserve = result[i].clone();
        match reserve.unwrap() {
            Token::Tuple(response) => {
                let reserve_data = Reserve {
                    reserve0: response[0].clone().into_uint().unwrap(),
                    reserve1: response[1].clone().into_uint().unwrap(),
                };
                reserves.insert(pool.address.clone(), reserve_data);
            }
            _ => {}
        }
    }

    /*
    let contract = Contract::<Provider<Http>>::new(
        Address::from_str(tokens[0]).unwrap(),
        erc20_abi.clone(),
        client.clone(),
    );

    let call = contract.method::<_, String>("name", ())?;
    let name = call.call_raw().await?;

    println!("TokenName: {}", name);
    */

    Ok(vec![
        CryptoToken::new(
            "0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270",
            "Wrapped Matic",
            "WMATIC",
            18
        )?,
        CryptoToken::new(
            "0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270",
            "Wrapped Ether",
            "WETH",
            18
        )?,
    ])
}
*/

fn get_token_from_db(db: &Database, token_id: i64, network: &NetworkKind) -> Result<CryptoToken> {
    let db_token: Option<DbToken> = db.get_token_by_id(token_id)?;
    if db_token.is_none() {
        return Err(anyhow!("Token not found"));
    }

    let db_token_network: Option<DbTokenNetwork> = db.get_token_network_by_token(token_id, network)?;
    if db_token_network.is_none() {
        return Err(anyhow!("Token network not found"));
    }

    let db_token_network: DbTokenNetwork = db_token_network.unwrap();
    let db_token: DbToken = db_token.unwrap();

    CryptoToken::new(
        network,
        db_token_network.address,
        db_token.name,
        db_token.symbol,
        db_token.decimals as u8,
    )
}

fn token_from_db_token(db: &Database, db_token: DbToken, network: &NetworkKind) -> Result<CryptoToken> {
    get_token_from_db(db, db_token.id, network)
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

async fn get_amms(db: &Database, network: &NetworkKind, provider: &impl NodeProvider) -> Result<Vec<AmmProtocolKind>> {
    let tokens: Vec<CryptoToken> = get_tokens(db, network)?;
    let pairs: Vec<(&CryptoToken, &CryptoToken)> = generate_pairs(&tokens);

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

                        let token0: CryptoToken = get_token_from_db(db, token0.unwrap().id, network)?;
                        let token1: CryptoToken = get_token_from_db(db, token1.unwrap().id, network)?;

                        uniswap_v2.add_pool(UniswapV2Pool::new(
                            pool_address,
                            Arc::new(uniswap_v2.clone()),
                            *network,
                            Arc::new(token0),
                            Arc::new(token1),
                        )?);
                        continue;
                    }

                    let pool_address: Address = factory_contract
                        .get_pair(*token_a.address(), *token_b.address())
                        .call_raw()
                        .await?;

                    if pool_address.is_zero() {
                        continue;
                    }

                    let pair_contract = UniswapV2PairAbi::new(pool_address, provider.raw_http_provider().clone());
                    let token0: Address = pair_contract.token_0().call_raw().await?;
                    let token1: Address = pair_contract.token_1().call_raw().await?;

                    let token0: Option<DbToken> = db.get_token_by_address(to_checksum(&token0, None), network)?;
                    let token1: Option<DbToken> = db.get_token_by_address(to_checksum(&token1, None), network)?;
                    if token0.is_none() || token1.is_none() {
                        return Err(anyhow!("Token not found"));
                    }

                    let token0: CryptoToken = token_from_db_token(db, token0.unwrap(), network)?;
                    let token1: CryptoToken = token_from_db_token(db, token1.unwrap(), network)?;

                    let pool: UniswapV2Pool = UniswapV2Pool::new(
                        pool_address,
                        Arc::new(uniswap_v2.clone()),
                        *network,
                        Arc::new(token0),
                        Arc::new(token1),
                    )?;

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

    let amms: Vec<AmmProtocolKind> = get_amms(&db, &target_network, provider_manager.get_next().deref()).await?;

    // 2 are traingle arbitrage
    let strategy = BackRunnerStragegy::new(provider_manager, amms, 2, vec![]);
    strategy.run().await?;

    //sb.0.spawn(event_handler(provider.clone(), sb.1.clone()));

    //while let Some(res) = ns.join_next().await {
    //    println!("{:?}", res);
    //}

    Ok(())
}
