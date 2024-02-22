use std::sync::{Arc, RwLock};

use anyhow::Result;
use contracts::uniswap_v2_pair::UniswapV2PairAbi;
use ethers::abi::Token;
use ethers::types::Bytes;
use ethers_contract::Multicall;
use ethers_providers::Middleware;

use crate::AmmPool;

// TODO: Should change `dyn AmmPool` to `UniswapV2Pool`

async fn get_uniswap_v2_reserves<M: Middleware>(provider: Arc<M>, pools: Vec<Arc<RwLock<dyn AmmPool>>>) {
    let mut multicall = Multicall::<M>::new(Arc::clone(&provider), None).await.unwrap();
    for pool in &pools {
        let contract = UniswapV2PairAbi::new(*pool.read().unwrap().address(), Arc::clone(&provider));
        multicall.add_call(contract.get_reserves(), false);
    }

    let result: Vec<Result<Token, Bytes>> = multicall.call_raw().await.unwrap();
    for i in 0..result.len() {
        let pool: &Arc<RwLock<dyn AmmPool>> = &pools[i];
        let reserve: &Result<Token, Bytes> = &result[i];

        if let Ok(Token::Tuple(response)) = reserve {
            pool.write().unwrap().update_reserve(
                &response[0].clone().into_uint().unwrap(),
                &response[1].clone().into_uint().unwrap(),
            );
        }
    }
}

pub async fn batch_update_uniswap_v2_pools<M: Middleware>(provider: Arc<M>, pools: &[Arc<RwLock<dyn AmmPool>>]) {
    let pools_cnt = pools.len() as f32;
    let batch: f32 = (pools_cnt / 250_f32).ceil();
    let pools_per_batch: usize = (pools_cnt / batch).ceil() as usize;
    let pools_cnt: usize = pools_cnt as usize;

    for i in 0..(batch as usize) {
        let start_idx: usize = i * pools_per_batch;
        let end_idx: usize = std::cmp::min(start_idx + pools_per_batch, pools_cnt);
        get_uniswap_v2_reserves(Arc::clone(&provider), pools[start_idx..end_idx].to_vec()).await;
    }
}
