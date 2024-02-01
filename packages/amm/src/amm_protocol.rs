use std::sync::Arc;

use crate::AmmPool;

pub trait AmmProtocol {
    type Pool: AmmPool;

    fn name(&self) -> &str;
    fn pools(&self) -> Vec<Arc<Self::Pool>>;
    fn options(&self) -> String;
    fn set_options(&mut self, options: String);
}
