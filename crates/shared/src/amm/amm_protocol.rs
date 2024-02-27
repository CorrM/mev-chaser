use std::sync::{Arc, RwLock};

use crate::amm::AmmPool;

pub trait AmmProtocol : Send + Sync {
    fn name(&self) -> &str;
    fn pools(&self) -> Vec<Arc<RwLock<dyn AmmPool>>>;
}
