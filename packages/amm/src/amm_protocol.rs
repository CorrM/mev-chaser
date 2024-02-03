use std::sync::Arc;

use crate::{AmmPool, AmmProtocolKind};

pub trait AmmProtocol {
    fn kind(&self) -> AmmProtocolKind;
    fn name(&self) -> &str;
    fn pools(&self) -> Vec<Arc<dyn AmmPool>>;
}
