use super::AmmPool;
use std::sync::Arc;

pub trait AmmProtocol {
    fn name(&self) -> &str;
    fn pools(&self) -> Vec<Arc<dyn AmmPool>>;
}
