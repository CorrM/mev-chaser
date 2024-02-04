use std::{collections::HashMap, sync::Arc};

use ethers_core::types::Address;

use super::PoolPath;

pub struct PoolPathsContainer {
    address_to_paths: HashMap<Address, Vec<Arc<PoolPath>>>,
}

impl PoolPathsContainer {
    pub fn new() -> Self {
        PoolPathsContainer {
            address_to_paths: HashMap::new(),
        }
    }

    pub fn add_path(&mut self, path: PoolPath) {
        let arc_path: Arc<PoolPath> = Arc::new(path);
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

    pub fn get_paths_containing_pool(&self, address: &Address) -> Option<&Vec<Arc<PoolPath>>> {
        self.address_to_paths.get(address)
    }
}
