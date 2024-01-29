use crate::uniswap_v2_pool::UniswapV2Pool;
use shared::amm::{AmmPool, AmmProtocol, AmmProtocolKind};

pub struct UniswapV2Protocol {
    name: String,
    pools: Vec<UniswapV2Pool>,
}

impl UniswapV2Protocol {
    pub fn new(name: impl Into<String>, pools: Vec<UniswapV2Pool>) -> Self {
        Self {
            name: name.into(),
            pools,
        }
    }
}

impl AmmProtocol for UniswapV2Protocol {
    fn name(&self) -> &str {
        &self.name
    }

    fn kind(&self) -> AmmProtocolKind {
        AmmProtocolKind::UniswapV2
    }

    fn pools(&self) -> &Vec<impl AmmPool> {
        &self.pools
    }
}
