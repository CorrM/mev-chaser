use std::sync::Arc;

use crate::amm::AmmProtocolKind;

pub struct AmmManager {
    amms: Vec<Arc<AmmProtocolKind>>,
}

impl AmmManager {
    pub fn new(amms: Vec<Arc<AmmProtocolKind>>) -> Self {
        Self { amms }
    }
}

impl AmmManager {
    #[inline]
    pub fn amms(&self) -> &Vec<Arc<AmmProtocolKind>> {
        &self.amms
    }
}
