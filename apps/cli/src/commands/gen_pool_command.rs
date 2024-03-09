use std::{str::FromStr, sync::Arc};

use anyhow::Result;
use ethers::{
    abi::Token,
    contract::Multicall,
    providers::Middleware,
    types::{Address, Bytes},
    utils::to_checksum,
};

use contracts::{uniswap_v2_factory::UniswapV2FactoryAbi, uniswap_v2_pair::UniswapV2PairAbi};
use shared::database::{Database, DbDex, DbDexNetwork, DbDexProtocol, DbToken};
use vidger::types::NetworkKind;
use vidger::utilities::block_on;

fn generate_pairs<T>(list: &[T]) -> Vec<(&T, &T)> {
    let mut pairs: Vec<(&T, &T)> = Vec::new();
    for (i, fst) in list.iter().enumerate() {
        for snd in list.iter().skip(i + 1) {
            pairs.push((fst, snd));
        }
    }
    pairs
}

pub struct GenPoolCommand;

impl GenPoolCommand {
    fn add_uniswap_v2_pools<M: Middleware>(
        token_pairs: &[(&String, &String)],
        db_dex: &DbDex,
        db: &Database,
        target_network: &NetworkKind,
        provider: &Arc<M>,
    ) -> Result<()> {
        let db_dex_network: Option<DbDexNetwork> = db.get_dex_network(db_dex.id, target_network)?;
        if db_dex_network.is_none() {
            panic!("Failed to get dex network");
        }
        let db_dex_network: DbDexNetwork = db_dex_network.unwrap();
        let network_options: serde_json::Value = serde_json::from_str(&db_dex_network.options)?;

        let dex_factory_address = Address::from_str(network_options["factory"].as_str().unwrap()).unwrap();
        let factory_contract = UniswapV2FactoryAbi::new(dex_factory_address, Arc::clone(provider));

        let tokens_cnt = token_pairs.len() as f32;
        let batch: f32 = (tokens_cnt / 250_f32).ceil();
        let tokens_per_batch: usize = (tokens_cnt / batch).ceil() as usize;
        let tokens_cnt: usize = tokens_cnt as usize;

        for i in 0..(batch as usize) {
            let start_idx: usize = i * tokens_per_batch;
            let end_idx: usize = std::cmp::min(start_idx + tokens_per_batch, tokens_cnt);
            let pairs_chank: &[(&String, &String)] = &token_pairs[start_idx..end_idx];

            // Get pools addresses
            println!(
                "[-] Getting pools [{} -> {}] addresses for dex '{}'",
                start_idx, end_idx, db_dex.name
            );
            let mut multicall: Multicall<M> = block_on(Multicall::new(Arc::clone(provider), None)).unwrap();
            for (token_a, token_b) in pairs_chank {
                // Can't execlude pairs here, because it will cause an error in the next for loop
                let token_a: Address = Address::from_str(token_a).unwrap();
                let token_b: Address = Address::from_str(token_b).unwrap();

                multicall.add_call(factory_contract.get_pair(token_a, token_b), false);
            }

            let result: Vec<Result<Token, Bytes>> = block_on(multicall.call_raw()).unwrap();
            let mut pools_to_add: Vec<Address> = Vec::new();
            for i in 0..result.len() {
                let (token_a, token_b): (&String, &String) = pairs_chank[i];
                if db
                    .get_dex_pool_by_tokens(db_dex.id, target_network, token_a, token_b)?
                    .is_some()
                {
                    let pool_address: &Result<Token, Bytes> = &result[i];
                    let Ok(Token::Address(pool_address)) = pool_address else {
                        panic!("Failed to get token name");
                    };

                    let token0: DbToken = db.get_token_by_address(token_a, target_network)?.unwrap();
                    let token1: DbToken = db.get_token_by_address(token_b, target_network)?.unwrap();

                    println!(
                        "Dex pool ({})[{} - {}] '{}' already exists",
                        db_dex.name,
                        token0.symbol,
                        token1.symbol,
                        to_checksum(pool_address, None)
                    );
                    continue;
                }

                let pool_address: &Result<Token, Bytes> = &result[i];
                let Ok(Token::Address(pool_address)) = pool_address else {
                    panic!("Failed to get token name");
                };

                if pool_address.is_zero() {
                    if db
                        .add_dex_pool_empty(db_dex.id, target_network, token_a, token_b)
                        .is_err()
                    {
                        panic!("Failed to add empty pool '{}' '{}'", token_a, token_b);
                    }

                    continue;
                }

                pools_to_add.push(*pool_address);
            }

            // Add pools
            println!("[+] Adding {} pools for dex '{}'", pools_to_add.len(), db_dex.name);
            multicall.clear_calls();
            for pool_address in &pools_to_add {
                let pool_contract = UniswapV2PairAbi::new(*pool_address, Arc::clone(provider));

                // TODO: filter by check if pool is empty and check reserves
                // TODO: Maybe check if the reservers are worth 1000USDT

                multicall.add_call(pool_contract.token_0(), false);
                multicall.add_call(pool_contract.token_1(), false);
            }

            let result: Vec<Result<Token, Bytes>> = block_on(multicall.call_raw()).unwrap();
            for i in (0..result.len()).step_by(2) {
                let pool_address: &Address = &pools_to_add[i / 2];

                let token0: &Result<Token, Bytes> = &result[i];
                let token1: &Result<Token, Bytes> = &result[i + 1];
                let Ok(Token::Address(token0)) = token0 else {
                    panic!("Failed to get token0 from pool '{}'", to_checksum(pool_address, None));
                };
                let Ok(Token::Address(token1)) = token1 else {
                    panic!("Failed to get token1 from pool '{}'", to_checksum(pool_address, None));
                };

                let token0: String = to_checksum(token0, None);
                let token1: String = to_checksum(token1, None);

                let token0: DbToken = db.get_token_by_address(token0, target_network)?.unwrap();
                let token1: DbToken = db.get_token_by_address(token1, target_network)?.unwrap();

                if db
                    .add_dex_pool(pool_address, target_network, db_dex.id, token0.id, token1.id)
                    .is_err()
                {
                    panic!("Failed to add dex pool '{}'", pool_address);
                };

                println!(
                    "Added dex pool ({})[{} - {}] '{}'",
                    db_dex.name,
                    token0.symbol,
                    token1.symbol,
                    to_checksum(pool_address, None)
                );
            }
        }

        //db.add_dex_pool(pool_address, Arc::new(uniswap_v2.clone()), *network, token0, token1);

        Ok(())
    }

    pub fn process<M: Middleware>(db: &Database, target_network: &NetworkKind, provider: Arc<M>) -> Result<()> {
        let tokens_address: Vec<String> = db
            .get_tokens(target_network)?
            .iter()
            .map(|(_, tn)| tn.address.clone())
            .collect::<Vec<_>>();
        let pairs: Vec<(&String, &String)> = generate_pairs(&tokens_address);

        let db_dexes: Vec<DbDex> = db.get_dexes_by_network(target_network)?;
        for db_dex in &db_dexes {
            let db_dex_protocol: Option<DbDexProtocol> = db.get_dex_protocol_by_id(db_dex.dex_protocol_id)?;
            if db_dex_protocol.is_none() {
                panic!("No protocol found for dex {}", db_dex.name);
            }

            let db_dex_protocol: DbDexProtocol = db_dex_protocol.unwrap();
            let protocol_name: &str = db_dex_protocol.name.as_str();

            match protocol_name {
                "UniswapV2" => GenPoolCommand::add_uniswap_v2_pools(&pairs, db_dex, db, target_network, &provider)?,
                _ => panic!("Unsupported dex protocol"),
            }
        }

        Ok(())
    }
}
