use std::sync::Arc;

use anyhow::{anyhow, Result};
use ethers_core::{
    abi::Token,
    types::{Address, Bytes, U256},
    utils::to_checksum,
};

use amm::{AmmProtocol, UniswapV2Protocol, UniswapV2Simulator};
use contracts::balancer_flash_loan_recipient::OneSwapInfo;
use shared::token::CryptoToken;

use super::PoolPathItem;

fn make_uniswap_v2_protocol_swap_info(
    router: Address,
    path: Vec<Address>,
    amount_in: impl Into<U256>,
    amount_out_min: impl Into<U256>,
) -> Result<OneSwapInfo> {
    if path.len() < 2 {
        return Err(anyhow!("path must have at least 2 elements"));
    }

    let token_in: Address = path[0];
    let path_token: Vec<Token> = path.into_iter().map(Token::Address).collect();
    let addresses = Token::Array(path_token);
    let encoded_path: Bytes = ethers::abi::encode(&[addresses]).into();

    Ok(OneSwapInfo {
        protocol: 0,
        router,
        token_in,
        path: encoded_path,
        amount_in: amount_in.into(),
        amount_out_min: amount_out_min.into(),
        deadline: U256::from(0),
    })
}

#[derive(Debug, Clone)]
pub struct PoolPath {
    // TODO:
    //swaps: Vec<OneSwapInfo>,
    //chain_swaps: bool,
    path: Vec<PoolPathItem>,
    input_token: Arc<CryptoToken>,
}

impl PoolPath {
    pub fn new(path: Vec<PoolPathItem>) -> Self {
        let first_path_item: &PoolPathItem = &path[0];

        let input_token: Arc<CryptoToken> = if first_path_item.zero_are_input {
            Arc::clone(first_path_item.pool.read().unwrap().token0())
        } else {
            Arc::clone(first_path_item.pool.read().unwrap().token1())
        };

        Self { path, input_token }
    }

    pub fn path(&self) -> &Vec<PoolPathItem> {
        &self.path
    }

    pub fn contains_pool(&self, pool_address: &Address) -> bool {
        for path_item in &self.path {
            if *path_item.pool.read().unwrap().address() == *pool_address {
                return true;
            }
        }

        false
    }

    #[inline]
    pub fn get_input_token(&self) -> &CryptoToken {
        &self.input_token
    }

    #[inline]
    pub fn get_amount_out_v2(&self, amount_in: U256) -> Option<U256> {
        let mut amount_out: U256 = amount_in;

        for path_item in &self.path {
            if amount_out.is_zero() {
                break;
            }

            let reserve0: U256 = path_item.pool.read().unwrap().reserve0();
            let reserve1: U256 = path_item.pool.read().unwrap().reserve1();

            let reserve_in: U256;
            let reserve_out: U256;
            if path_item.zero_are_input {
                reserve_in = reserve0;
                reserve_out = reserve1;
            } else {
                reserve_in = reserve1;
                reserve_out = reserve0;
            }

            //let fee: U256 = U256::from(path_item.pool.read().unwrap().dex().fees);
            if reserve_in.is_zero() || reserve_out.is_zero() {
                let address: String = to_checksum(path_item.pool.read().unwrap().address(), None);
                panic!(
                    "get_amount_out_v2 => {}: amount_in: {}, reserve_in: {}, reserve_out: {}",
                    address, amount_in, reserve_in, reserve_out
                );
            }

            let fee: U256 = U256::from(300);
            amount_out = UniswapV2Simulator::get_amount_out(amount_out, reserve_in, reserve_out, fee)?;
        }

        Some(amount_out)
    }

    #[inline]
    pub fn find_optimal_input(&self, max_count_in: u64, step_size: usize) -> (U256, U256) {
        let input_token: &CryptoToken = self.get_input_token();
        let input_token_unit: U256 = input_token.input_token_unit();

        let mut optimized_in: U256 = U256::zero();
        let mut profit: i128 = 0;

        for amount_in in (0..max_count_in).step_by(step_size) {
            let amount_in: U256 = U256::from(amount_in) * input_token_unit;
            let Some(amount_out) = self.get_amount_out_v2(amount_in) else {
                continue;
            };

            let amount_out_i128: i128 = amount_out.as_u128() as i128;
            let amount_in_i128: i128 = amount_in.as_u128() as i128;
            let this_profit: i128 = amount_out_i128 - amount_in_i128;

            if this_profit >= profit {
                optimized_in = amount_in;
                profit = this_profit;
            } else {
                break;
            }
        }

        (optimized_in, U256::from(profit))
    }

    #[inline]
    pub fn make_swaps(&self, input_amount: U256, output_amount: U256) -> Result<(Vec<OneSwapInfo>, bool)> {
        if self.path.len() < 2 {
            return Err(anyhow!("Not enough paths"));
        }

        let first_path_dex: Arc<dyn AmmProtocol> = self.path[0].pool.read().unwrap().dex();
        let all_are_same_dex: bool = self.path.iter().all(|p| {
            let pool_read_lock = p.pool.read().unwrap();
            Arc::ptr_eq(&pool_read_lock.dex(), &first_path_dex)
        });

        let mut swaps: Vec<OneSwapInfo> = Vec::new();
        let mut chain_swaps: bool = false;
        if all_are_same_dex {
            let v2_dex_ptr: *mut UniswapV2Protocol = &*first_path_dex as *const _ as *mut UniswapV2Protocol;
            let router: Address = unsafe { *(*v2_dex_ptr).router() };

            let mut path: Vec<Address> = self
                .path
                .iter()
                .map(|p| {
                    let pool_read_lock = p.pool.read().unwrap();
                    if p.zero_are_input {
                        *pool_read_lock.token0().address()
                    } else {
                        *pool_read_lock.token1().address()
                    }
                })
                .collect();

            // Add Output token
            let last_path_item = &self.path[self.path.len() - 1];
            if last_path_item.zero_are_input {
                path.push(*last_path_item.pool.read().unwrap().token1().address());
            } else {
                path.push(*last_path_item.pool.read().unwrap().token0().address());
            }

            let Ok(swap) = make_uniswap_v2_protocol_swap_info(router, path, input_amount, output_amount) else {
                return Err(anyhow!("Failed to make UniswapV2ProtocolSwapInfo"));
            };

            swaps.push(swap);
        } else {
            for (idx, path_item) in self.path.iter().enumerate() {
                let v2_dex_ptr: *mut UniswapV2Protocol =
                    &*path_item.pool.read().unwrap().dex() as *const _ as *mut UniswapV2Protocol;
                let router: Address = unsafe { *(*v2_dex_ptr).router() };

                let path: Vec<Address> = if path_item.zero_are_input {
                    let pool_read_lock = path_item.pool.read().unwrap();
                    vec![*pool_read_lock.token0().address(), *pool_read_lock.token1().address()]
                } else {
                    let pool_read_lock = path_item.pool.read().unwrap();
                    vec![*pool_read_lock.token1().address(), *pool_read_lock.token0().address()]
                };

                // Its chain swap, so only first swap needs input amount
                let cur_intput_amount: U256 = if idx == 0 { input_amount } else { U256::zero() };

                // Its chain swap, so only last swap needs output amount
                let cur_output_amount: U256 = if idx == self.path.len() - 1 {
                    output_amount
                } else {
                    U256::zero()
                };

                let Ok(swap) = make_uniswap_v2_protocol_swap_info(router, path, cur_intput_amount, cur_output_amount)
                else {
                    return Err(anyhow!("Failed to make UniswapV2ProtocolSwapInfo"));
                };

                swaps.push(swap);
            }

            chain_swaps = true;
        }

        Ok((swaps, chain_swaps))
    }
}

/*
async fn test_contract(env: &Env, provider_manager: &NodeProviderManager) -> Result<()> {
    //let bot_address = SolidityBridge::deploy(provider_manager.get_next().raw_ws_provider().clone(), env.private_key.clone()).await;
    //let Ok(bot_address) = bot_address else { return Err(anyhow!("Failed to deploy")); };

    let solidity_bridge = SolidityBridge::new(
        Address::from_str(&env.bot_address).unwrap(),
        Arc::clone(provider_manager.get_next().raw_ws_provider()),
        env.private_key.clone(),
    )
    .await?;

    let swaps: Vec<OneSwapInfo> = vec![
        PoolPath::make_uniswap_v2_protocol_swap_info(
            Address::from_str("0xa5E0829CaCEd8fFDD4De3c43696c57F7D7A678ff").unwrap(),
            vec![
                Address::from_str("0xc2132D05D31c914a87C6611C10748AEb04B58e8F").unwrap(),
                Address::from_str("0x346404079b3792a6c548B072B9C4DDdFb92948d5").unwrap(),
            ],
            10000000,
            0,
        )
        .unwrap(),
        PoolPath::make_uniswap_v2_protocol_swap_info(
            Address::from_str("0xa5E0829CaCEd8fFDD4De3c43696c57F7D7A678ff").unwrap(),
            vec![
                Address::from_str("0x346404079b3792a6c548B072B9C4DDdFb92948d5").unwrap(),
                Address::from_str("0xc2132D05D31c914a87C6611C10748AEb04B58e8F").unwrap(),
            ],
            0,
            1000000,
        )
        .unwrap(),
    ];

    println!(
        "{:?}",
        solidity_bridge
            .estimate_get_loan_then_swap_chain(swaps, true, false)
            .await
            .unwrap()
    );

    Ok(())
}
*/
