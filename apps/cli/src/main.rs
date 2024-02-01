use anyhow::{anyhow, Result};
use database::Database;
use ethers::prelude::H256;
use ethers_contract::{Contract, Multicall};
use ethers_core::abi::{Abi, Token};
use ethers_core::types::Bytes;
use ethers_core::{
    abi::{AbiEncode, Log},
    types::H160,
};
use ethers_providers::{Http, Provider};
use mev::DexBackRunnerStragegy;
use std::{env::VarError, ops::Deref, path::Path, str::FromStr, sync::Arc};
use tokio::sync::broadcast::Receiver;

use amm_protocol::{AmmProtocolContainer, UniswapV2Protocol};
use shared::provider::NodeProvider;
use shared::token::CryptoToken;
use shared::{
    abi::ABI,
    network::NetworkKind,
    network_streams::{NetworkEvent, NetworkStreamManagerBuilder, NetworkStreamsManager},
    provider::{
        DebugTraceCallNodeProvider, NodeProviderKind, NodeProviderManager, NodeProviderNetworkInfo, NormalNodeProvider,
    },
    trace::{get_trace_all_logs, TraceLogData},
};

use crate::utils::env::Env;

mod database;
mod utils;

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

async fn get_tokens(provider: &impl NodeProvider, erc20_abi: &Abi) -> Result<Vec<CryptoToken>> {
    panic!("Not implemented");
    /*
    // TODO: depend on network
    let tokens = vec![
        "0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270",
        "0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270",
    ];

    let client: Provider<Http> = provider.raw_http_provider().clone();
    let client: Arc<Provider<Http>> = Arc::new(client);

    let mut multicall: Multicall<Provider<Http>> = Multicall::new(client.clone(), None).await?;
    for pool in tokens {
        let contract = Contract::<Provider<Http>>::new(
            *pool.address(),
            erc20_abi.clone(),
            client.clone(),
        );

        let call = contract.method::<_, H256>("getReserves", ())?;
        multicall.add_call(call, false);
    }

    let result: Vec<Result<Token, Bytes>> = multicall.call_raw().await?;

    let contract = Contract::<Provider<Http>>::new(
        Address::from_str(tokens[0]).unwrap(),
        erc20_abi.clone(),
        client.clone(),
    );

    let call = contract.method::<_, String>("name", ())?;
    let name = call.call_raw().await?;

    println!("TokenName: {}", name);

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
    */
}

fn get_amms(network: &NetworkKind) -> Result<Vec<AmmProtocolContainer>> {
    // TODO: depend on network
    Ok(vec![
        AmmProtocolContainer::UniswapV2(UniswapV2Protocol::new(
            "SushiSwap",
            300,
            "0xc35DADB65012eC5796536bD9864eD8773aBc74C4",
            "0x1b02dA8Cb0d097eB8D57A175b88c7D8b47997506",
        )?),
        AmmProtocolContainer::UniswapV2(UniswapV2Protocol::new(
            "QuickSwapV2",
            300,
            "0x5757371414417b8C6CAad45bAeF941aBc7d3Ab32",
            "0xa5E0829CaCEd8fFDD4De3c43696c57F7D7A678ff",
        )?),
    ])
}

#[tokio::main]
async fn main() -> Result<()> {
    let env: Env = get_env()?;
    let abi = ABI::new(Path::new("./abi"))?;
    let db = Database::new(Path::new("./Main.db"))?;

    let target_network: NetworkKind = unsafe { std::mem::transmute(env.chain_id) };
    let provider_manager: NodeProviderManager = create_node_provider_manager(&env, &target_network).await?;

    //let tokens: Vec<CryptoToken> = get_tokens(provider.deref(), &abi.erc20).await?;
    let amms: Vec<AmmProtocolContainer> = get_amms(&target_network)?;

    let strategy = DexBackRunnerStragegy::new(abi, provider_manager, amms);
    strategy.run().await?;

    //sb.0.spawn(event_handler(provider.clone(), sb.1.clone()));

    //while let Some(res) = ns.join_next().await {
    //    println!("{:?}", res);
    //}

    Ok(())
}
