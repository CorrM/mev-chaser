use std::sync::{Arc, RwLock};

use anyhow::Result;
use contracts::simulator::SimulatorAbiErrors;
use dashmap::mapref::one::RefMut;
use dashmap::DashMap;
use ethers::prelude::H256;
use ethers::types::{Address, U256, U64};
use ethers::utils::{keccak256, to_checksum};
use ethers::{providers::Middleware, types::Log};
use hashbrown::HashSet;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

use vidger::logger::{error, info};
use vidger::types::NewBlock;

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
            input_to_output: (U256::from(0), U256::from(0)),
            top_profitable_paths: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

pub struct PoolManager<M> {
    provider: Arc<M>,
    simulator: Arc<RwLock<EvmSimulator<M>>>,
    pools: DashMap<Address, Arc<RwLock<PoolContainer>>>,
    pools_sync_events: Vec<H256>,
}

impl<M> PoolManager<M>
where
    M: Middleware + 'static,
{
    pub fn new(provider: Arc<M>, simulator: Arc<RwLock<EvmSimulator<M>>>, amm_manager: &AmmManager) -> Self {
        static UNI_V2_V3_SYNC_EVENT: &str = "Sync(uint112,uint112)";
        let pools_sync_events = vec![H256::from(keccak256(UNI_V2_V3_SYNC_EVENT))];

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
            pools_sync_events,
        }
    }

    pub fn setup(&mut self) -> Result<()> {
        Ok(())
    }
}

impl<M: Middleware + 'static> PoolManager<M> {
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

    #[inline]
    pub fn get_optimal_input_and_output(&self, pool: &AmmPoolKind) -> (U256, U256) {
        println!("pool: {}", to_checksum(pool.address(), None));
        println!(
            "{} balance: {:?}",
            pool.token0().symbol(),
            self.simulator
                .read()
                .unwrap()
                .get_token_balance(&pool.token0().address().0.into())
        );
        println!(
            "{} balance: {:?}",
            pool.token1().symbol(),
            self.simulator
                .read()
                .unwrap()
                .get_token_balance(&pool.token1().address().0.into())
        );

        let u256: Result<U256, SimulatorAbiErrors> =
            self.simulator
                .read()
                .unwrap()
                .get_amounts_out(pool, pool.token0(), pool.token0().convert_to_amount(1_f64));
        if let Ok(u256) = u256 {
            info!(
                "{} -> {} => {}",
                pool.token0().symbol(),
                pool.token1().symbol(),
                pool.token1().convert_to_decimal(u256)
            );
        } else {
            error!("Error get_amounts_out: {:?}", u256);
        }
        println!("==============================");
        (U256::from(0), U256::from(0))
    }

    pub fn on_new_block(&mut self, _new_block: &NewBlock, logs: &[Log]) {
        /*
        - Get touched pools then updates its tuple (optimal input, output)
        - Update most 50 profitable paths for touched pools
          - Profitable paths are path that have most output of every pool in the path
            (Most profitable output are the max input-to-output ratio)
        */

        // Get touched pools, Update optimal input to output, Must do it before update most profitable paths.
        // because `for_each` will call the callback on sequence, that's why use .collect() to execute the callback on
        // all touched pools, then we can get most profitable paths after
        let touched_pools: Vec<Arc<RwLock<PoolContainer>>> = logs
            .par_iter()
            .filter_map(|log: &Log| {
                self.pools.get(&log.address)?;

                // Check if this pool is touched
                if !self.pools_sync_events.contains(&log.topics[0]) {
                    return None;
                }

                Some(log.address)
            })
            .collect::<HashSet<_>>()
            .into_par_iter()
            .filter_map(|address: Address| {
                let pool_container: RefMut<Address, Arc<RwLock<PoolContainer>>> = self.pools.get_mut(&address)?;

                // Take care there a possibility of race condition between read and write, keep use try_write
                let input_output: (U256, U256) =
                    self.get_optimal_input_and_output(&pool_container.try_read().unwrap().pool);

                pool_container
                    .try_write()
                    .expect("Failed to get write lock")
                    .input_to_output = input_output;

                Some(Arc::clone(&*pool_container))
            })
            .collect();

        if touched_pools.is_empty() {
            return;
        }

        // Generate most profitable paths for touched pools
        touched_pools
            .into_par_iter()
            .for_each(|pool_container: Arc<RwLock<PoolContainer>>| {
                // Keep in mind that's block the lock, so you can't get write lock here only read lock or change your mind
                //let pool_container: &PoolContainer = &pool_container.read().unwrap();

                //let top_profitable_paths: Vec<Arc<RwLock<PoolPath>>> = pool_container
                //    .paths
                //    .read()
                //    .unwrap()
                //    .par_iter()
                //    .map(|path| path)
                //    .collect();
                //*pool_container.top_profitable_paths.write().unwrap() = top_profitable_paths;

                //info!(
                //    "Generate most profitable paths for pool '{}'",
                //    to_checksum(pool_container.pool.address(), None)
                //);
            });
    }
}
