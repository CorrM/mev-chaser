use std::collections::HashSet;
use crate::amm::AmmPool;

use crate::token::CryptoToken;

#[derive(Clone, Debug)]
struct DfsParams {
    current_token: CryptoToken,
    output_token: CryptoToken,
    visited_pairs: HashSet<impl AmmPool>,
    route: Vec<DexTokenPathItem>,
    hop_count: i32,
    max_multi_hop: i32,
}

pub struct PoolPathFinder {
    token_pools: Vec<DexTokenPoolBase>,
}

impl PoolPathFinder {
    pub fn new(token_pools: Vec<DexTokenPoolBase>) -> Self {
        PoolPathFinder { token_pools }
    }

    pub fn generate_paths(
        &self,
        input_token: CryptoToken,
        output_token: CryptoToken,
        max_multi_hop: i32,
    ) -> Vec<Vec<DexTokenPathItem>> {
        let mut arbitrage_paths = Vec::new();
        let mut visited_pairs = HashSet::new();
        let initial_route = Vec::new();

        let dfs_params = DfsParams {
            current_token: input_token.clone(),
            output_token,
            visited_pairs: visited_pairs.clone(),
            route: initial_route.clone(),
            hop_count: 0,
            max_multi_hop,
        };

        self.dfs(dfs_params, &mut arbitrage_paths);

        arbitrage_paths
    }

    fn dfs(&self, dfs_params: DfsParams, arbitrage_paths: &mut Vec<Vec<DexTokenPathItem>>) {
        if dfs_params.hop_count > dfs_params.max_multi_hop {
            return;
        }

        for next_pool in &self.token_pools {
            if (!dfs_params.current_token.eq(&next_pool.token0)
                && !dfs_params.current_token.eq(&next_pool.token1))
                || dfs_params.visited_pairs.contains(next_pool)
            {
                continue;
            }

            let next_token = if dfs_params.current_token == next_pool.token0 {
                next_pool.token1.clone()
            } else {
                next_pool.token0.clone()
            };

            let mut new_route = dfs_params.route.clone();
            new_route.push(DexTokenPathItem::new(
                next_pool.clone(),
                next_pool.token0 == dfs_params.current_token,
            ));

            let mut new_visited_pairs = dfs_params.visited_pairs.clone();
            new_visited_pairs.insert(next_pool.clone());

            if next_token == dfs_params.output_token && new_route.len() > 1 {
                arbitrage_paths.push(new_route.clone());
            } else {
                let new_dfs_params = DfsParams {
                    current_token: next_token,
                    visited_pairs: new_visited_pairs,
                    route: new_route,
                    hop_count: dfs_params.hop_count + 1,
                    ..dfs_params.clone()
                };
                self.dfs(new_dfs_params, arbitrage_paths);
            }
        }
    }
}
