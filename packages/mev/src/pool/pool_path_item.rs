use std::sync::Arc;

use amm::AmmPoolKind;

#[derive(Clone)]
pub struct PoolPathItem {
    pub pool: Arc<AmmPoolKind>,
    pub zero_are_input: bool,
}

impl PoolPathItem {
    pub(crate) fn new(pool: Arc<AmmPoolKind>, zero_are_input: bool) -> Self {
        Self {
            pool,
            zero_are_input,
        }
    }
}
