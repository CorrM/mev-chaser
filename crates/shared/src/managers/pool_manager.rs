use std::sync::{Arc, RwLock};

use anyhow::Result;
use dashmap::DashMap;
use ethers::providers::Middleware;
use ethers::types::{Address, Filter, Log, U256, U64};
use ethers::utils::to_checksum;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

use vidger::logger::{error, info};
use vidger::utilities::block_on;

use crate::amm::{AmmPoolKind, AmmProtocolKind};
use crate::managers::AmmManager;
use crate::simulator::EvmSimulator;
use crate::types::PoolPath;

pub type SafePoolPathVec = RwLock<Vec<Arc<RwLock<PoolPath>>>>;

struct PoolContainer {
    /// Pool
    pub pool: Arc<AmmPoolKind>,
    /// Pool paths
    pub paths: Arc<SafePoolPathVec>, // TODO: Delete this as top_profitable_paths will be generated and updated
    /// Optimal input to output
    pub input_to_output: (U256, U256),
    /// Top profitable paths
    pub top_profitable_paths: Arc<SafePoolPathVec>,
}

impl PoolContainer {
    fn new(pool: Arc<AmmPoolKind>) -> Self {
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
    simulator: Arc<RwLock<EvmSimulator<M>>>,
    pools: DashMap<Address, Arc<RwLock<PoolContainer>>>,
    pools_sync_filter: Filter,
}

impl<M> PoolManager<M>
where
    M: Middleware + 'static,
{
    pub fn new(provider: Arc<M>, simulator: Arc<RwLock<EvmSimulator<M>>>, amm_manager: &AmmManager) -> Self {
        const UNI_V2_V3_SYNC_EVENT: &str = "Sync(uint112,uint112)";
        let pools_sync_filter: Filter = Filter::new().events(vec![UNI_V2_V3_SYNC_EVENT]);

        let pools: DashMap<Address, Arc<RwLock<PoolContainer>>> = amm_manager
            .amms()
            .par_iter()
            .flat_map(|dex: &Arc<AmmProtocolKind>| dex.pools())
            .map(|pool: &Arc<AmmPoolKind>| {
                (
                    *pool.address(),
                    Arc::new(RwLock::new(PoolContainer::new(Arc::clone(pool)))),
                )
            })
            .collect();

        Self {
            provider,
            simulator,
            pools,
            pools_sync_filter,
        }
    }

    pub fn setup(&mut self) -> Result<()> {
        Ok(())
    }
}

impl<M> PoolManager<M>
where
    M: Middleware + 'static,
{
    #[inline]
    pub fn get_optimal_input_and_output(&self, pool: &AmmPoolKind) -> (U256, U256) {
        // TODO: Simulation needed here
        self.simulator
            .read()
            .unwrap()
            .get_amounts_out(pool, pool.token0().convert_to_amount(1_f64))
            .unwrap();
        (0.into(), 0.into())
    }

    pub fn add_path(&mut self, path: PoolPath) {
        for path_item in path.path() {
            self.pools
                .entry(*path_item.pool.address())
                .or_insert_with(|| Arc::new(RwLock::new(PoolContainer::new(Arc::clone(&path_item.pool)))))
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

    pub fn on_new_block(&mut self, block_number: U64) {
        /*
        - Get touched pools then updates its tuple (optimal input, output)
        - Update most 50 profitable paths for touched pools
          - Profitable paths are path that have most output of every pool in the path
            (Most profitable output are the max input-to-output ratio)
        */

        // Get touched pools
        let event_filter: Filter = self
            .pools_sync_filter
            .clone()
            .from_block(block_number)
            .to_block(block_number);

        let logs: Result<Vec<Log>, <M as Middleware>::Error> = block_on(self.provider.get_logs(&event_filter));
        let Ok(logs) = logs else {
            error!("failed to get logs for block {}", block_number);
            return;
        };

        if logs.is_empty() {
            return;
        }

        // Get touched pools, Update optimal input to output, Must do it before update most profitable paths.
        // because `for_each` will call the callback on sequence, that's why use .collect() to execute the callback on
        // all touched pools, then we can get most profitable paths after
        let touched_pools: Vec<Arc<RwLock<PoolContainer>>> = logs
            .par_iter()
            .filter_map(|log| {
                let pool_address = self.pools.get(&log.address).map(|_| log.address);
                let Some(pool_address) = pool_address else {
                    return None;
                };

                let pool_container = self.pools.get_mut(&pool_address);
                let Some(pool_container_ref) = pool_container else {
                    return None;
                };

                // Take care there a possibility of race condition between read and write, keep use try_write
                let input_output: (U256, U256) =
                    self.get_optimal_input_and_output(&pool_container_ref.try_read().unwrap().pool);

                pool_container_ref
                    .try_write()
                    .expect("Failed to get write lock")
                    .input_to_output = input_output;

                Some(Arc::clone(&*pool_container_ref))
            })
            .collect();

        if touched_pools.is_empty() {
            return;
        }

        // Generate most profitable paths for touched pools
        touched_pools.into_par_iter().for_each(|pool_container| {
            // Keep in mind that's block the lock, so you can't get write lock here only read lock or change your mind
            let pool_container: &PoolContainer = &pool_container.read().unwrap();

            //let top_profitable_paths: Vec<Arc<RwLock<PoolPath>>> = pool_container
            //    .paths
            //    .read()
            //    .unwrap()
            //    .par_iter()
            //    .map(|path| path)
            //    .collect();
            //*pool_container.top_profitable_paths.write().unwrap() = top_profitable_paths;

            info!("syncing pool {}", to_checksum(pool_container.pool.address(), None));
        });
    }
}
