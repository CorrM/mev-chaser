use std::sync::{Arc, RwLock};

use anyhow::Result;
use contracts::UniswapV2PairAbi;
use ethers::{
    abi::Token,
    providers::{Http, Provider},
    types::Bytes,
};
use ethers_contract::Multicall;

use shared::provider::{NodeProvider, NormalNodeProvider};

use crate::AmmPool;

// TODO: Should change `dyn AmmPool` to `UniswapV2Pool`

async fn get_uniswap_v2_reserves(provider: &NormalNodeProvider, pools: Vec<Arc<RwLock<dyn AmmPool>>>) {
    let client: &Arc<Provider<Http>> = provider.raw_http_provider();

    let mut multicall: Multicall<Provider<Http>> = Multicall::new(Arc::clone(client), None).await.unwrap();
    for pool in &pools {
        let contract = UniswapV2PairAbi::new(*pool.read().unwrap().address(), client.clone());
        multicall.add_call(contract.get_reserves(), false);
    }

    let result: Vec<Result<Token, Bytes>> = multicall.call_raw().await.unwrap();
    for i in 0..result.len() {
        let pool = &pools[i];
        let reserve: Result<Token, Bytes> = result[i].clone();

        if let Ok(Token::Tuple(response)) = reserve {
            pool.write().unwrap().update_reserve(
                &response[0].clone().into_uint().unwrap(),
                &response[1].clone().into_uint().unwrap(),
            );
        }
    }
}

pub async fn batch_update_uniswap_v2_pools(provider: &NormalNodeProvider, pools: &Vec<Arc<RwLock<dyn AmmPool>>>) {
    let pools_cnt: usize = pools.len();
    let batch: f32 = ((pools_cnt / 250) as f32).ceil();
    let pools_per_batch: usize = ((pools_cnt as f32) / batch).ceil() as usize;

    for i in 0..(batch as usize) {
        let start_idx: usize = i * pools_per_batch;
        let end_idx: usize = std::cmp::min(start_idx + pools_per_batch, pools_cnt);
        get_uniswap_v2_reserves(provider, pools[start_idx..end_idx].to_vec()).await;
    }
}
