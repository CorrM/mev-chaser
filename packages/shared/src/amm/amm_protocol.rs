use super::{AmmPool, AmmProtocolKind};
use std::sync::Arc;

pub trait AmmProtocol {
    fn name(&self) -> &str;
    fn protocol(&self) -> AmmProtocolKind;
    fn pools(&self) -> Vec<Arc<dyn AmmPool>>;
    fn options(&self) -> String;
    fn set_options(&mut self, options: String);
}
