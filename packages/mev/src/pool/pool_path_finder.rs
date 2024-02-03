use std::sync::Arc;

use amm::AmmPool;
use shared::token::CryptoToken;

use super::{pool_path_item::PoolPathItem, PoolPath};

fn dfs(
    token_pools: &Vec<Arc<dyn AmmPool>>,
    current_token: &Arc<CryptoToken>,
    output_token: &Arc<CryptoToken>,
    visited_pairs: &mut Vec<Arc<dyn AmmPool>>,
    route: &mut PoolPath,
    hop_count: i32,
    max_multi_hop: i32,
    arbitrage_paths: &mut Vec<PoolPath>,
) {
    if hop_count > max_multi_hop {
        return;
    }

    for next_pool in token_pools {
        let token0: &Arc<CryptoToken> = next_pool.token0();
        let token1: &Arc<CryptoToken> = next_pool.token1();

        if !Arc::ptr_eq(current_token, token0) && !Arc::ptr_eq(current_token, token1)
            || visited_pairs
                .iter()
                .any(|x| Arc::ptr_eq(next_pool, x))
        {
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
        visited_pairs.push(Arc::clone(next_pool));

        if Arc::ptr_eq(next_token, output_token) && route.len() > 1 {
            arbitrage_paths.push(route.to_vec());
        } else {
            dfs(
                token_pools,
                next_token,
                output_token,
                visited_pairs,
                route,
                hop_count + 1,
                max_multi_hop,
                arbitrage_paths,
            );
        }

        route.pop();
        visited_pairs.pop();
    }
}

pub fn generate_pool_paths(
    pools: &Vec<Arc<dyn AmmPool>>,
    input_token: &Arc<CryptoToken>,
    output_token: &Arc<CryptoToken>,
    max_multi_hop: i32,
) -> Vec<PoolPath> {
    let mut arbitrage_paths: Vec<PoolPath> = Vec::new();
    let mut visited_pairs: Vec<Arc<dyn AmmPool>> = Vec::new();
    let mut initial_route: PoolPath = Vec::new();

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
