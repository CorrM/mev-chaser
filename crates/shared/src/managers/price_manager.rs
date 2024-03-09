use std::{
    sync::{Arc, RwLock},
    time::Duration,
};

use anyhow::Result;
use dashmap::DashMap;
use ethers::types::Address;
use hashbrown::HashMap;

use crate::amm::AmmPoolKind;

pub struct PriceManager {
    network_native_token: Address,
    price_calc_pools: HashMap<Address, Arc<RwLock<AmmPoolKind>>>,
    price_cache: Arc<DashMap<Address, Arc<RwLock<f64>>>>,
}

impl PriceManager {
    pub fn new(
        network_native_token: Address,
        tokens_to_hold_price: Vec<Address>,
        pools: &Vec<Arc<RwLock<AmmPoolKind>>>,
    ) -> Arc<Self> {
        let price_calc_pools: HashMap<Address, Arc<RwLock<AmmPoolKind>>> = tokens_to_hold_price
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

        let price_cache: Arc<DashMap<Address, Arc<RwLock<f64>>>> = Arc::new(
            tokens_to_hold_price
                .iter()
                .map(|t| (*t, Arc::new(RwLock::new(0.0))))
                .collect(),
        );

        let ret = Arc::new(Self {
            network_native_token,
            price_calc_pools,
            price_cache: Arc::clone(&price_cache),
        });

        let ret_copy: Arc<PriceManager> = Arc::clone(&ret);
        tokio::spawn(async move {
            loop {
                for item in price_cache.iter() {
                    let k: &Address = item.key();
                    let v: &Arc<RwLock<f64>> = item.value();

                    *v.write().unwrap() = ret_copy.get_native_token_price_impl(k).unwrap();
                }

                tokio::time::sleep(Duration::from_secs(5 * 60)).await;
            }
        });

        Arc::clone(&ret)
    }

    fn get_native_token_price_impl(&self, in_term_of_token: &Address) -> Result<f64> {
        panic!("Use simulator to get price");
        /*
        let price_pool: Option<&Arc<RwLock<AmmPoolKind>>> = self.price_calc_pools.get(in_term_of_token);
        if price_pool.is_none() {
            return Err(anyhow!("Could not find pool for token({})", in_term_of_token));
        }

        let price_pool: &AmmPoolKind = &*price_pool.unwrap().read().unwrap();
        let token_price: f64 = UniswapV2Simulator::reserves_to_price(
            price_pool,
            *price_pool.token0().address() == self.network_native_token,
        );

        Ok(token_price)
        */
    }

    pub fn get_native_token_price(&self, in_term_of_token: &Address) -> Result<f64> {
        if let Some(price) = self.price_cache.get(in_term_of_token) {
            return Ok(*price.read().unwrap());
        }

        panic!("Could not find price for token({})", in_term_of_token);
    }
}
