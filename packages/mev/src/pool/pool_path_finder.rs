use std::{ops::Deref, sync::Arc};

use amm::{AmmPool, AmmPoolKind};
use shared::token::CryptoToken;

use super::pool_path_item::PoolPathItem;

struct DfsParams {
    current_token: Arc<CryptoToken>,
    output_token: Arc<CryptoToken>,
    visited_pairs: Vec<Arc<AmmPoolKind>>, // TODO: Should use HashSet but with Arc its hard
    route: Vec<PoolPathItem>,
    hop_count: i32,
    max_multi_hop: i32,
}

// TODO: Get raid of DfsParams and make input_token and output_token just references
pub struct PoolPathFinder;

impl PoolPathFinder {
    pub fn generate_paths(
        pools: &Vec<AmmPoolKind>,
        input_token: Arc<CryptoToken>,
        output_token: Arc<CryptoToken>,
        max_multi_hop: i32,
    ) -> Vec<Vec<PoolPathItem>> {
        let mut arbitrage_paths: Vec<Vec<PoolPathItem>> = Vec::new();
        let visited_pairs: Vec<Arc<AmmPoolKind>> = Vec::new();
        let initial_route: Vec<PoolPathItem> = Vec::new();

        let mut dfs_params = DfsParams {
            current_token: input_token,
            output_token,
            visited_pairs,
            route: initial_route,
            hop_count: 0,
            max_multi_hop,
        };

        PoolPathFinder::dfs(pools, &mut dfs_params, &mut arbitrage_paths);

        arbitrage_paths
    }

    fn dfs(token_pools: &Vec<AmmPoolKind>, dfs_params: &mut DfsParams, arbitrage_paths: &mut Vec<Vec<PoolPathItem>>) {
        if dfs_params.hop_count > dfs_params.max_multi_hop {
            return;
        }

        for next_pool in token_pools {
            let (token0, token1) = match next_pool {
                AmmPoolKind::UniswapV2(pool) => (pool.token0(), pool.token1()),
            };

            if !Arc::ptr_eq(&dfs_params.current_token, token0) && !Arc::ptr_eq(&dfs_params.current_token, token1)
                || dfs_params
                    .visited_pairs
                    .iter()
                    .any(|x| std::ptr::eq(next_pool, x.deref()))
            {
                continue;
            }

            let next_token: Arc<CryptoToken> = if Arc::ptr_eq(&dfs_params.current_token, token0) {
                token1.clone()
            } else {
                token0.clone()
            };

            let next_pool = Arc::new(next_pool.clone());
            dfs_params.route.push(PoolPathItem::new(
                next_pool.clone(),
                Arc::ptr_eq(token0, &dfs_params.current_token),
            ));
            dfs_params.visited_pairs.push(next_pool.clone());

            if Arc::ptr_eq(&next_token, &dfs_params.output_token) && dfs_params.route.len() > 1 {
                arbitrage_paths.push(dfs_params.route.to_vec());
            } else {
                let cur_token: Arc<CryptoToken> = dfs_params.current_token.clone();

                dfs_params.current_token = next_token;
                dfs_params.hop_count += 1;

                PoolPathFinder::dfs(token_pools, dfs_params, arbitrage_paths);

                dfs_params.current_token = cur_token;
                dfs_params.hop_count -= 1;
            }
        }
    }
}
