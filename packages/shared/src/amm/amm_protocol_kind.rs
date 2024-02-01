use std::fmt::{Display, Formatter, Result};

pub enum AmmProtocolKind {
    UniswapV2,
    UniswapV3,
}

impl Display for AmmProtocolKind {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            AmmProtocolKind::UniswapV2 => write!(f, "UniswapV2"),
            AmmProtocolKind::UniswapV3 => write!(f, "UniswapV3"),
        }
    }
}