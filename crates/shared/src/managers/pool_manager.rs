use std::sync::Arc;

use ethers::providers::Middleware;
use ethers::types::Address;
use hashbrown::HashMap;

use crate::amm::AmmPoolKind;
use crate::types::PoolPath;

pub struct PoolManager<M> {
    /// Provider
    provider: Arc<M>,
    /// Maps pool address to pool
    pools: HashMap<Address, AmmPoolKind>,
    /// Maps pool address to paths
    address_to_paths: HashMap<Address, Vec<Arc<PoolPath>>>,
}

impl<M> PoolManager<M>
where
    M: Middleware + 'static,
{
    pub fn new(provider: Arc<M>) -> Self {
        PoolManager {
            provider,
            pools: HashMap::new(),
            address_to_paths: HashMap::new(),
        }
    }

    pub fn add_path(&mut self, path: PoolPath) {
        let arc_path = Arc::new(path);

        for path_item in arc_path.path().iter() {
            self.address_to_paths
                .entry(*path_item.pool.read().unwrap().address())
                .or_default()
                .push(Arc::clone(&arc_path));
        }
    }

    pub fn add_multi_path(&mut self, paths: Vec<PoolPath>) {
        for path in paths {
            self.add_path(path);
        }
    }

    pub fn get_paths_containing_pool(&self, pool_address: &Address) -> Option<&Vec<Arc<PoolPath>>> {
        self.address_to_paths.get(pool_address)
    }
}
