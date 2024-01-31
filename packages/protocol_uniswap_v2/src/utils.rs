use std::{collections::HashMap, sync::Arc};

use anyhow::Result;
use ethers::{
    abi::Token,
    providers::{Http, Provider},
    types::{Bytes, H160, H256, U256},
};
use ethers_contract::{Contract, Multicall};

use shared::{abi::ABI, amm::AmmPool, provider::NodeProvider};

use crate::UniswapV2Pool;

#[derive(Default, Debug, Clone)]
pub struct Reserve {
    pub reserve0: U256,
    pub reserve1: U256,
}

pub async fn get_uniswap_v2_reserves<T: NodeProvider>(
    provider: T,
    abi: ABI,
    pools: Vec<UniswapV2Pool>,
) -> Result<HashMap<H160, Reserve>> {
    let client: Provider<Http> = provider.raw_http_provider().clone();
    let client: Arc<Provider<Http>> = Arc::new(client);

    let mut multicall: Multicall<Provider<Http>> = Multicall::new(client.clone(), None).await?;
    for pool in &pools {
        let contract = Contract::<Provider<Http>>::new(
            *pool.address(),
            abi.uniswap_v2_pair.clone(),
            client.clone(),
        );
        let call = contract.method::<_, H256>("getReserves", ())?;
        multicall.add_call(call, false);
    }

    let result: Vec<Result<Token, Bytes>> = multicall.call_raw().await?;

    let mut reserves: HashMap<H160, Reserve> = HashMap::new();
    for i in 0..result.len() {
        let pool = &pools[i];
        let reserve = result[i].clone();
        if let Ok(Token::Tuple(response)) = reserve {
            let reserve_data = Reserve {
                reserve0: response[0].clone().into_uint().unwrap(),
                reserve1: response[1].clone().into_uint().unwrap(),
            };
            reserves.insert(*pool.address(), reserve_data);
        }
    }

    Ok(reserves)
}

pub async fn batch_get_uniswap_v2_reserves<T: 'static + NodeProvider>(
    provider: T,
    abi: &ABI,
    pools: Vec<UniswapV2Pool>,
) -> HashMap<H160, Reserve> {
    let pools_cnt: usize = pools.len();
    let batch: f32 = ((pools_cnt / 250) as f32).ceil();
    let pools_per_batch: usize = ((pools_cnt as f32) / batch).ceil() as usize;

    let mut handles = vec![];

    for i in 0..(batch as usize) {
        let start_idx: usize = i * pools_per_batch;
        let end_idx: usize = std::cmp::min(start_idx + pools_per_batch, pools_cnt);
        let handle = tokio::spawn(get_uniswap_v2_reserves(
            provider.clone(),
            abi.clone(),
            pools[start_idx..end_idx].to_vec(),
        ));
        handles.push(handle);
    }

    let mut reserves: HashMap<H160, Reserve> = HashMap::new();

    for handle in handles {
        let result = handle.await.unwrap();
        reserves.extend(result.unwrap());
    }

    reserves
}
