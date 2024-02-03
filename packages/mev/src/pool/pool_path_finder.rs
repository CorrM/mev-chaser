use std::{ops::Deref, sync::Arc};

use amm::{AmmPool, AmmPoolKind};
use shared::token::CryptoToken;

use super::pool_path_item::PoolPathItem;

fn dfs(
    token_pools: &Vec<AmmPoolKind>,
    current_token: &Arc<CryptoToken>,
    output_token: &Arc<CryptoToken>,
    visited_pairs: &mut Vec<Arc<AmmPoolKind>>,
    route: &mut Vec<PoolPathItem>,
    hop_count: i32,
    max_multi_hop: i32,
    arbitrage_paths: &mut Vec<Vec<PoolPathItem>>,
) {
    if hop_count > max_multi_hop {
        return;
    }

    for next_pool in token_pools {
        let (token0, token1) = match next_pool {
            AmmPoolKind::UniswapV2(pool) => (pool.token0(), pool.token1()),
        };

        if !Arc::ptr_eq(current_token, token0) && !Arc::ptr_eq(current_token, token1)
            || visited_pairs.iter().any(|x| std::ptr::eq(next_pool, x.deref()))
        {
            continue;
        }

        let next_token: &Arc<CryptoToken> = if Arc::ptr_eq(current_token, token0) {
            token1
        } else {
            token0
        };

        let next_pool = Arc::new(next_pool.clone());
        route.push(PoolPathItem::new(
            next_pool.clone(),
            Arc::ptr_eq(token0, current_token),
        ));
        visited_pairs.push(next_pool);

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
    pools: &Vec<AmmPoolKind>,
    input_token: &Arc<CryptoToken>,
    output_token: &Arc<CryptoToken>,
    max_multi_hop: i32,
) -> Vec<Vec<PoolPathItem>> {
    let mut arbitrage_paths: Vec<Vec<PoolPathItem>> = Vec::new();
    let mut visited_pairs: Vec<Arc<AmmPoolKind>> = Vec::new();
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
