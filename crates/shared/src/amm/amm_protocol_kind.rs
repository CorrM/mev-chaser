use std::fmt::{Display, Formatter, Result};

use ethers::types::Address;

use crate::amm::UniswapV2Protocol;

pub enum AmmProtocolKind {
    UniswapV2(UniswapV2Protocol),
}

impl AmmProtocolKind {
    #[inline]
    pub fn is_uniswap_v2(&self) -> bool {
        matches!(self, AmmProtocolKind::UniswapV2(_))
    }

    #[inline]
    pub fn as_uniswap_v2(&self) -> Option<&UniswapV2Protocol> {
        match self {
            AmmProtocolKind::UniswapV2(protocol) => Some(protocol),
            _ => None,
        }
    }

    #[inline]
    pub fn as_uniswap_v2_mut(&mut self) -> Option<&mut UniswapV2Protocol> {
        match self {
            AmmProtocolKind::UniswapV2(protocol) => Some(protocol),
            _ => None,
        }
    }
}

impl AmmProtocolKind {
    #[inline]
    pub fn name(&self) -> &str {
        match self {
            AmmProtocolKind::UniswapV2(protocol) => protocol.name(),
        }
    }

    #[inline]
    pub fn router(&self) -> &Address {
        match self {
            AmmProtocolKind::UniswapV2(protocol) => protocol.router(),
        }
    }

    #[inline]
    pub fn factory(&self) -> &Address {
        match self {
            AmmProtocolKind::UniswapV2(protocol) => protocol.factory(),
        }
    }
}

impl Display for AmmProtocolKind {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            AmmProtocolKind::UniswapV2(_) => write!(f, "UniswapV2"),
        }
    }
}
