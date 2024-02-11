use std::sync::Arc;

use anyhow::Result;

use amm::AmmPoolKind;
use contracts::{ERC20TokenAbi, UniswapV2PairAbi};
use ethers_core::{types::Address, utils::to_checksum};
use ethers_providers::{Http, Provider};
use shared::{network::NetworkKind, token::CryptoToken};

use crate::database::Database;

pub struct AddPoolCommand;

impl AddPoolCommand {
    pub async fn process(
        pools_type: AmmPoolKind,
        pools: Vec<&str>,
        db: &Database,
        target_network: &NetworkKind,
        provider: Arc<Provider<Http>>,
    ) -> Result<()> {
        for pool in pools {
            if db.get_dex_pool(target_network, pool)?.is_some() {
                continue;
            }

            println!("Adding pool {}", pool);
            match pools_type {
                AmmPoolKind::UniswapV2 => {
                    let pair_contract = UniswapV2PairAbi::new(pool.parse::<Address>()?, Arc::clone(&provider));

                    let token0: Address = pair_contract.token_0().call_raw().await?;
                    let token0_str: String = to_checksum(&token0, None);
                    let token1: Address = pair_contract.token_1().call_raw().await?;
                    let token1_str: String = to_checksum(&token1, None);

                    if db.get_token_by_address(&token0_str, target_network).is_err() {
                        let token_contract = ERC20TokenAbi::new(token0, provider.clone());
                        let token_name: String = token_contract.name().call_raw().await?;
                        let token_symbol: String = token_contract.symbol().call_raw().await?;
                        let token_decimals: u8 = token_contract.decimals().call_raw().await?.as_u64() as u8;

                        println!("Adding token0 {} '{}'", token_symbol, &token0_str);
                        let token_add = db.add_token(&CryptoToken::new(
                            target_network,
                            &token0_str,
                            token_name,
                            token_symbol,
                            token_decimals,
                        )?);
                        if token_add.is_err() {
                            println!("Token {} already exists", &token0_str);
                        }
                    }

                    if db.get_token_by_address(&token1_str, target_network).is_err() {
                        let token_contract = ERC20TokenAbi::new(token1, provider.clone());
                        let token_name: String = token_contract.name().call_raw().await?;
                        let token_symbol: String = token_contract.symbol().call_raw().await?;
                        let token_decimals: u8 = token_contract.decimals().call_raw().await?.as_u64() as u8;

                        println!("Adding token1 {} {}", token_symbol, token1_str);
                        let token_add = db.add_token(&CryptoToken::new(
                            target_network,
                            &token1_str,
                            token_name,
                            token_symbol,
                            token_decimals,
                        )?);
                        if token_add.is_err() {
                            println!("Token {} already exists", &token1_str);
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
