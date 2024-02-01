use amm::{AmmProtocolKind, UniswapV2Pool, UniswapV2Protocol};
use anyhow::{anyhow, Result};
use database::Database;
use ethers_contract::Contract;
use ethers_core::types::Address;
use ethers_providers::{Http, Provider};
use mev::BackRunnerStragegy;
use shared::provider::NodeProvider;
use std::ops::Deref;
use std::sync::Arc;
use std::{env::VarError, path::Path};

use shared::token::CryptoToken;
use shared::{
    abi::ABI,
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

async fn get_amms(abi: &ABI, provider: &impl NodeProvider, tokens: &[CryptoToken]) -> Result<Vec<AmmProtocolKind>> {
    let mut amms: Vec<AmmProtocolKind> = vec![
        AmmProtocolKind::UniswapV2(UniswapV2Protocol::new(
            "SushiSwap",
            300,
            "0xc35DADB65012eC5796536bD9864eD8773aBc74C4",
            "0x1b02dA8Cb0d097eB8D57A175b88c7D8b47997506",
        )?),
        AmmProtocolKind::UniswapV2(UniswapV2Protocol::new(
            "QuickSwapV2",
            300,
            "0x5757371414417b8C6CAad45bAeF941aBc7d3Ab32",
            "0xa5E0829CaCEd8fFDD4De3c43696c57F7D7A678ff",
        )?),
    ];

    let pairs: Vec<(&CryptoToken, &CryptoToken)> = generate_pairs(tokens);
    for (token_a, token_b) in pairs {
        for amm in &mut amms {
            match amm {
                AmmProtocolKind::UniswapV2(v2) => {
                    let contract = Contract::<Provider<Http>>::new(
                        *v2.factory(),
                        abi.uniswap_v2_factory.clone(),
                        provider.raw_http_provider().clone(),
                    );

                    let call = contract.method::<_, Address>("getPair", ())?;
                    let pool_address: Address = call.call_raw().await?;
                    
                    v2.add_pool(UniswapV2Pool::new(pool_address, Arc::new(v2.clone()))?)
                },
            }
        }
    }

    Ok(amms)
}

#[tokio::main]
async fn main() -> Result<()> {
    let env: Env = get_env()?;
    let abi = ABI::new(Path::new("./abi"))?;
    let db = Database::new(Path::new("./Main.db"))?;

    let target_network: NetworkKind = unsafe { std::mem::transmute(env.chain_id) };
    let provider_manager: NodeProviderManager = create_node_provider_manager(&env, &target_network).await?;

    //let tokens: Vec<CryptoToken> = get_tokens(db, provider_manager.get_next().deref(), &abi.erc20).await?;
    let tokens: Vec<CryptoToken> = get_tokens(&db, &target_network)?;
    let amms: Vec<AmmProtocolKind> = get_amms(&abi, provider_manager.get_next().deref(), &tokens).await?;

    // 2 are traingle arbitrage
    let strategy = BackRunnerStragegy::new(abi, provider_manager, amms, 2, vec![]);
    strategy.run().await?;

    //sb.0.spawn(event_handler(provider.clone(), sb.1.clone()));

    //while let Some(res) = ns.join_next().await {
    //    println!("{:?}", res);
    //}

    Ok(())
}
