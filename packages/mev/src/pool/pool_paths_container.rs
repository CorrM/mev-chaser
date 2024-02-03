use std::{collections::HashMap, sync::Arc};

use ethers_core::types::Address;

use super::PoolPath;

struct PoolPathsContainer {
    address_to_paths: HashMap<Address, Vec<Arc<PoolPath>>>,
}

impl PoolPathsContainer {
    fn new() -> Self {
        PoolPathsContainer {
            address_to_paths: HashMap::new(),
        }
    }

    fn add_path(&mut self, path: PoolPath) {
        let arc_path: Arc<PoolPath> = Arc::new(path);
        for path_item in arc_path.iter() {
            self.address_to_paths
                .entry(path_item.pool.address().clone())
                .or_insert_with(Vec::new)
                .push(Arc::clone(&arc_path));
        }
    }

    fn add_multi_path(&mut self, paths: Vec<PoolPath>) {
        for path in paths {
            self.add_path(path);
        }
    }

    fn get_paths_containing_pool(&self, address: &Address) -> Option<&Vec<Arc<PoolPath>>> {
        self.address_to_paths.get(address)
    }
}
