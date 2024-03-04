use std::sync::Arc;

use crate::amm::AmmPoolKind;

pub(super) trait AmmProtocol: Send + Sync {
    fn name(&self) -> &str;
    fn pools(&self) -> &Vec<Arc<AmmPoolKind>>;
    fn add_pool(&mut self, pool: AmmPoolKind);
}
