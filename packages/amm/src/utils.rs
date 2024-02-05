use std::{collections::HashMap, sync::{Arc, RwLock}};

use anyhow::Result;
use ethers::types::{U256, U64};
use ethers_core::{abi::{decode, ParamType, Token}, types::{Address, Filter, Log, H160}};
use ethers_providers::{Middleware, Provider, Ws};

use crate::AmmPool;

pub async fn update_touched_pool_reserves(
    provider: &Arc<Provider<Ws>>,
    block_number: U64,
    pools: &mut HashMap<Address, Arc<RwLock<dyn AmmPool>>>
) -> Result<()> {
    let sync_event: &str = "Sync(uint112,uint112)";
    let event_filter: Filter = Filter::new()
        .from_block(block_number)
        .to_block(block_number)
        .event(sync_event);

    let mut tx_idx: HashMap<H160, U64> = HashMap::new();
    
    let logs: Vec<Log> = provider.get_logs(&event_filter).await?;
    for log in &logs {
        let pool: Option<&mut Arc<RwLock<dyn AmmPool>>> = pools.get_mut(&log.address);
        if pool.is_none() {
            continue;
        }
        
        let decoded: Result<Vec<Token>, ethers_core::abi::Error> = decode(&[ParamType::Uint(256), ParamType::Uint(256)], &log.data);
        let Ok(data) = decoded else { continue };

        let idx: U64 = log.transaction_index.unwrap_or_default();
        let prev_tx_idx: Option<&U64> = tx_idx.get(&log.address);

        let update: bool = (*prev_tx_idx.unwrap_or(&U64::zero())) <= idx;
        if update {
            let reserve0: U256 = match data[0] {
                Token::Uint(rs) => rs,
                _ => U256::zero(),
            };
            let reserve1: U256 = match data[1] {
                Token::Uint(rs) => rs,
                _ => U256::zero(),
            };
            pool.unwrap().write().unwrap().update_reserve(reserve0, reserve1);

            tx_idx.insert(log.address, idx);
        }
    }

    Ok(())
}
