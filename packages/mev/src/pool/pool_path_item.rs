use crate::amm::AmmPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct PoolPathItem {
    pub pool: Arc<dyn AmmPool>,
    pub zero_are_input: bool,
}

impl PoolPathItem {
    pub(crate) fn new(pool: Arc<dyn AmmPool>, zero_are_input: bool) -> Self {
        Self {
            pool,
            zero_are_input,
        }
    }
}
