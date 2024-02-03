use std::fmt::{Display, Formatter, Result};

pub enum AmmProtocolKind {
    UniswapV2,
}

impl Display for AmmProtocolKind {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            AmmProtocolKind::UniswapV2 => write!(f, "UniswapV2"),
        }
    }
}