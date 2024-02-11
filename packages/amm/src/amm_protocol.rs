use std::sync::{Arc, RwLock};

use crate::{AmmPool, AmmProtocolKind};

pub trait AmmProtocol : Send + Sync {
    fn kind(&self) -> AmmProtocolKind;
    fn name(&self) -> &str;
    fn pools(&self) -> Vec<Arc<RwLock<dyn AmmPool>>>;
}
