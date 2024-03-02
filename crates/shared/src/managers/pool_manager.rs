use std::sync::{Arc, RwLock, RwLockReadGuard};

use anyhow::Result;
use dashmap::mapref::one::RefMut;
use dashmap::DashMap;
use ethers::providers::Middleware;
use ethers::types::{Address, Filter, Log, U256, U64};
use ethers::utils::to_checksum;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use vidger::logger::{error, info};

use crate::amm::AmmPoolKind;
use crate::types::PoolPath;

pub type SafePoolPathVec = RwLock<Vec<Arc<RwLock<PoolPath>>>>;

struct PoolContainer {
    /// Pool
    pub pool: AmmPoolKind,
    /// Pool paths
    pub paths: Arc<SafePoolPathVec>, // TODO: Delete this as top_profitable_paths will be generated and updated
    /// Optimal input to output
    pub input_to_output: (U256, U256),
    /// Top profitable paths
    pub top_profitable_paths: Arc<SafePoolPathVec>,
}

impl PoolContainer {
    fn new(pool: AmmPoolKind) -> Self {
        Self {
            pool,
            paths: Arc::new(RwLock::new(Vec::new())),
            input_to_output: (0.into(), 0.into()),
            top_profitable_paths: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

pub struct PoolManager<M> {
    provider: Arc<M>,
    pools: DashMap<Address, Arc<RwLock<PoolContainer>>>,
    pools_sync_filter: Filter,
}

impl<M> PoolManager<M>
where
    M: Middleware + 'static,
{
    pub fn new(provider: Arc<M>) -> Self {
        const UNI_V2_V3_SYNC_EVENT: &str = "Sync(uint112,uint112)";
        let event_filter: Filter = Filter::new().events(vec![UNI_V2_V3_SYNC_EVENT]);

        PoolManager {
            provider,
            pools: DashMap::new(),
            pools_sync_filter: event_filter,
        }
    }

    pub async fn setup(&mut self) -> Result<()> {
        Ok(())
    }
}

impl<M> PoolManager<M>
where
    M: Middleware + 'static,
{
    pub fn get_optimal_input_to_output(&self, pool: &AmmPoolKind) -> (U256, U256) {
        (0.into(), 0.into())
    }

    pub fn add_path(&mut self, path: PoolPath) {
        for path_item in path.path() {
            let pool_lock: RwLockReadGuard<AmmPoolKind> = path_item.pool.read().unwrap();
            self.pools
                .entry(*pool_lock.address())
                .or_insert_with(|| Arc::new(RwLock::new(PoolContainer::new((*pool_lock).clone()))))
                .write()
                .unwrap()
                .paths
                .write()
                .unwrap()
                .push(Arc::new(RwLock::new(path.clone())));
        }
    }

    pub fn add_multi_path(&mut self, paths: Vec<PoolPath>) {
        for path in paths {
            self.add_path(path);
        }
    }

    #[inline]
    pub fn get_paths_containing_pool(&self, pool_address: &Address) -> Option<Arc<SafePoolPathVec>> {
        self.pools
            .get(pool_address)
            .map(|pool| Arc::clone(&pool.read().unwrap().paths))
    }

    pub async fn on_new_block(&mut self, block_number: U64) {
        /*
        - Get touched pools updates its tuple (optimal input, output)
        - Update most 50 profitable paths for touched pools
          - Profitable paths are path that have most output of every pool in the path
            (Most profitable output are the max input-to-output ratio)
        */
        let event_filter: Filter = self
            .pools_sync_filter
            .clone()
            .from_block(block_number)
            .to_block(block_number);

        let logs: Result<Vec<Log>, <M as Middleware>::Error> = self.provider.get_logs(&event_filter).await;
        let Ok(logs) = logs else {
            error!("failed to get logs for block {}", block_number);
            return;
        };

        // Get touched pools
        let touched_pool_addresses = logs
            .par_iter()
            .filter_map(|log| self.pools.get(&log.address).map(|_| log.address));

        // Update optimal input to output, Must do it before update most profitable paths
        let pools_refs = touched_pool_addresses.filter_map(|pool_address| {
            let pool_container = self.pools.get_mut(&pool_address);
            let Some(pool_container_rwlock) = pool_container else {
                return None;
            };

            let pool_container: &PoolContainer = &pool_container_rwlock.read().unwrap();
            pool_container_rwlock.write().unwrap().input_to_output =
                self.get_optimal_input_to_output(&pool_container.pool);

            Some(pool_container_rwlock)
        });

        // Generate most profitable paths for touched pools
        pools_refs.for_each(|pool_item| {
            let pool_container: &PoolContainer = &pool_item.read().unwrap();

            let top_profitable_paths: Vec<Arc<RwLock<PoolPath>>> = pool_container
                .paths
                .read()
                .unwrap()
                .par_iter()
                .map(|path| path)
                .collect();
            *pool_container.top_profitable_paths.write().unwrap() = top_profitable_paths;

            info!("syncing pool {}", to_checksum(&pool_item.key(), None));
        });
    }
}
