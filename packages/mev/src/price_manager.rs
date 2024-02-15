use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use amm::{AmmPool, UniswapV2Simulator};
use anyhow::{anyhow, Result};
use ethers_core::types::Address;

pub struct PriceManager {
    network_native_token: Address,
    price_calc_pools: HashMap<Address, Arc<RwLock<dyn AmmPool>>>,
}

impl PriceManager {
    pub fn new(
        network_native_token: Address,
        tokens_to_hold_price: Vec<Address>,
        pools: &Vec<Arc<RwLock<dyn AmmPool>>>,
    ) -> Self {
        let price_calc_pools: HashMap<Address, Arc<RwLock<dyn AmmPool>>> = tokens_to_hold_price
            .iter()
            .map(|t| {
                for pool in pools {
                    let pool_read_lock = pool.read().unwrap();

                    if *pool_read_lock.token0().address() != network_native_token
                        && *pool_read_lock.token1().address() != network_native_token
                    {
                        continue;
                    }

                    if pool_read_lock.token0().address() != t && pool_read_lock.token1().address() != t {
                        continue;
                    }

                    return (*t, Arc::clone(pool));
                }

                panic!("Could not find pool for native token with token({})", t);
            })
            .collect();

        Self {
            network_native_token,
            price_calc_pools,
        }
    }

    pub fn get_native_token_price(&self, in_term_of_token: &Address) -> Result<f64> {
        let price_pool: Option<&Arc<RwLock<dyn AmmPool>>> = self.price_calc_pools.get(in_term_of_token);
        if price_pool.is_none() {
            return Err(anyhow!("Could not find pool for token({})", in_term_of_token));
        }
        
        let price_pool: &dyn AmmPool = &*price_pool.unwrap().read().unwrap();
        let token_price: f64 = UniswapV2Simulator::reserves_to_price(
            price_pool,
            *price_pool.token0().address() == self.network_native_token,
        );

        Ok(token_price)
    }
}
