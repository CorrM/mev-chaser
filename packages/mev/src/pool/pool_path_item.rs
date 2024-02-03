use std::{fmt::Debug, sync::{Arc, RwLock}};

use amm::AmmPool;

#[derive(Clone)]
pub struct PoolPathItem {
    pub pool: Arc<RwLock<dyn AmmPool>>,
    pub zero_are_input: bool,
}

impl PoolPathItem {
    pub(crate) fn new(pool: Arc<RwLock<dyn AmmPool>>, zero_are_input: bool) -> Self {
        Self { pool, zero_are_input }
    }
}

impl Debug for PoolPathItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PoolPathItem")
            .field("pool", self.pool.read().unwrap().address())
            .field("zero_are_input", &self.zero_are_input)
            .finish()
    }
}
