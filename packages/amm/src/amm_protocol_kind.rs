use std::fmt::{Display, Formatter, Result};

use crate::UniswapV2Protocol;

pub enum AmmProtocolKind {
    UniswapV2(UniswapV2Protocol),
}

impl Display for AmmProtocolKind {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            AmmProtocolKind::UniswapV2(_) => write!(f, "UniswapV2"),
        }
    }
}