use std::sync::Arc;

use amm::{AmmPool, AmmProtocol, UniswapV2Protocol, UniswapV2Simulator};
use anyhow::{anyhow, Result};
use contracts::OneSwapInfo;
use ethers_core::{
    types::{Address, U256},
    utils::to_checksum,
};
use shared::{solidity_bridge::SolidityBridge, token::CryptoToken};

use super::PoolPathItem;

#[derive(Debug)]
pub struct PoolPath {
    path: Vec<PoolPathItem>,
}

impl PoolPath {
    pub fn new(path: Vec<PoolPathItem>) -> Self {
        Self { path }
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

    pub fn get_input_token(&self) -> Arc<CryptoToken> {
        let first_pool = &self.path[0].pool;
        if self.path[0].zero_are_input {
            Arc::clone(first_pool.read().unwrap().token0())
        } else {
            Arc::clone(first_pool.read().unwrap().token1())
        }
    }

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

    pub fn optimize_amount_in(&self, max_count_in: u64, step_size: usize) -> (U256, U256, U256) {
        let input_token: Arc<CryptoToken> = self.get_input_token();
        let input_token_unit: U256 = U256::from(10).pow(U256::from(input_token.decimals()));

        let mut optimized_in: U256 = U256::zero();
        let mut amount_min_out: U256 = U256::zero();
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
                amount_min_out = amount_out;
                profit = this_profit;
            } else {
                break;
            }
        }

        (optimized_in, amount_min_out, U256::from(profit))
    }

    pub fn make_swaps(&self, input_amount: U256, output_amount: U256) -> Result<(Vec<OneSwapInfo>, bool)> {
        if self.path.len() < 2 {
            return Err(anyhow!("Not enough paths"));
        }

        let first_path_dex: Arc<dyn AmmProtocol> = self.path[0].pool.read().unwrap().dex();
        let all_are_same_dex: bool = self.path.iter().all(|p| {
            let pool_read_lock = p.pool.read().unwrap();
            Arc::ptr_eq(&pool_read_lock.dex(), &first_path_dex)
        });

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

            let Ok(swap) =
                SolidityBridge::make_uniswap_v2_protocol_swap_info(router, path, input_amount, output_amount)
            else {
                return Err(anyhow!("Failed to make UniswapV2ProtocolSwapInfo"));
            };

            return Ok((vec![swap], false));
        } else {
            let mut swaps: Vec<OneSwapInfo> = Vec::new();

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
                let cur_intput_amount: U256 = if idx == 0 {
                    input_amount
                } else {
                    U256::zero()
                };

                // Its chain swap, so only last swap needs output amount
                let cur_output_amount: U256 = if idx == self.path.len() - 1 {
                    output_amount
                } else {
                    U256::zero()
                };

                let Ok(swap) =
                    SolidityBridge::make_uniswap_v2_protocol_swap_info(router, path, cur_intput_amount, cur_output_amount)
                else {
                    return Err(anyhow!("Failed to make UniswapV2ProtocolSwapInfo"));
                };

                swaps.push(swap);
            }

            Ok((swaps, true))
        }
    }
}
