use std::sync::{Arc, RwLock};

use amm::{AmmPool, UniswapV2Simulator};
use ethers_core::{
    types::{Address, U256},
    utils::to_checksum,
};
use shared::token::CryptoToken;

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

    pub fn optimize_amount_in(&self, max_count_in: u64, step_size: usize) -> (U256, U256) {
        let input_token: Arc<CryptoToken> = self.get_input_token();
        let input_token_unit: U256 = U256::from(10).pow(U256::from(input_token.decimals()));

        let mut optimized_in: U256 = U256::zero();
        let mut profit: i128 = 0;

        for amount_in in (0..max_count_in).step_by(step_size) {
            let amount_in: U256 = U256::from(amount_in) * input_token_unit;
            let Some(amount_out) = self.get_amount_out_v2(amount_in) else {
                continue;
            };

            let amount_out_i128: i128 = amount_out.as_u128() as i128;
            let amount_in_i128: i128 = (amount_in * input_token_unit).as_u128() as i128;
            let this_profit: i128 = amount_out_i128 - amount_in_i128;

            if this_profit >= profit {
                optimized_in = amount_in;
                profit = this_profit;
            } else if amount_out_i128 == 0 && amount_in_i128 == 0 {
                // mostly one of pools in the path is not enough liquidity
                break;
            }
        }

        (optimized_in, U256::from(profit))
    }
}
