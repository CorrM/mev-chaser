use amm::AmmPool;
use ethers_core::utils::to_checksum;
use shared::token::CryptoToken;
use std::{
    collections::HashSet,
    sync::{Arc, RwLock},
};
use std::fmt::Debug;

use super::{pool_path_item::PoolPathItem, PoolPath};

#[derive(Clone)]
pub struct PoolPathItem {
    pub pool: Arc<RwLock<dyn AmmPool>>,
    pub zero_are_input: bool,
}

impl PoolPathItem {
    pub(crate) fn new(pool: Arc<RwLock<dyn AmmPool>>, zero_are_input: bool) -> Self {
        Self { pool, zero_are_input }
    }
}

impl Debug for PoolPathItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PoolPathItem")
            .field("pool", self.pool.read().unwrap().address())
            .field("zero_are_input", &self.zero_are_input)
            .finish()
    }
}

fn dfs(
    token_pools: &Vec<Arc<RwLock<dyn AmmPool>>>,
    current_token: &Arc<CryptoToken>,
    output_token: &Arc<CryptoToken>,
    visited_pools: &mut HashSet<usize>,
    route: &mut Vec<PoolPathItem>,
    hop_count: i32,
    max_multi_hop: i32,
    arbitrage_paths: &mut Vec<PoolPath>,
) {
    if hop_count > max_multi_hop {
        return;
    }

    for (idx, next_pool) in token_pools.iter().enumerate() {
        if visited_pools.contains(&idx) {
            continue;
        }
        
        let next_pool_read_lock = next_pool.read().unwrap();
        let token0: &Arc<CryptoToken> = next_pool_read_lock.token0();
        let token1: &Arc<CryptoToken> = next_pool_read_lock.token1();

        if !Arc::ptr_eq(current_token, token0) && !Arc::ptr_eq(current_token, token1) {
            continue;
        }

        let next_token: &Arc<CryptoToken> = if Arc::ptr_eq(current_token, token0) {
            token1
        } else {
            token0
        };

        route.push(PoolPathItem::new(
            Arc::clone(next_pool),
            Arc::ptr_eq(token0, current_token),
        ));
        visited_pools.insert(idx);

        let all_is_same_pool: bool = route.len() > 1
            && route
                .iter()
                .all(|r| route[0].pool.read().unwrap().address() == r.pool.read().unwrap().address());

        if all_is_same_pool {
            println!("{:?}", to_checksum(token_pools[37].read().unwrap().address(), None));
            println!("{:?}", to_checksum(token_pools[83].read().unwrap().address(), None));
            println!("{:?}", route);
            panic!("Found a path are all the same pool");
        }

        if !all_is_same_pool && Arc::ptr_eq(next_token, output_token) && route.len() > 1 {
            arbitrage_paths.push(PoolPath::new(route.to_vec()));
        } else {
            dfs(
                token_pools,
                next_token,
                output_token,
                visited_pools,
                route,
                hop_count + 1,
                max_multi_hop,
                arbitrage_paths,
            );
        }

        route.pop();
        visited_pools.remove(&idx);
    }
}

pub fn generate_pool_paths(
    pools: &Vec<Arc<RwLock<dyn AmmPool>>>,
    input_token: &Arc<CryptoToken>,
    output_token: &Arc<CryptoToken>,
    max_multi_hop: i32,
) -> Vec<PoolPath> {
    let mut arbitrage_paths: Vec<PoolPath> = Vec::new();
    let mut visited_pairs: HashSet<usize> = HashSet::new();
    let mut initial_route: Vec<PoolPathItem> = Vec::new();

    dfs(
        pools,
        input_token,
        output_token,
        &mut visited_pairs,
        &mut initial_route,
        0,
        max_multi_hop,
        &mut arbitrage_paths,
    );

    arbitrage_paths
}
