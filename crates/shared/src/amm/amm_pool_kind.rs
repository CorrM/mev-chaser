use std::fmt::{Display, Formatter};
use std::sync::Arc;

use ethers::types::Address;

use crate::amm::amm_pool::AmmPool;
use crate::amm::{AmmProtocolKind, UniswapV2Pool};
use crate::types::CryptoToken;

#[derive(Clone)]
pub enum AmmPoolKind {
    UniswapV2(UniswapV2Pool),
}

impl AmmPoolKind {
    #[inline]
    pub fn is_uniswap_v2(&self) -> bool {
        matches!(self, AmmPoolKind::UniswapV2(_))
    }

    #[inline]
    pub fn as_uniswap_v2(&self) -> Option<&UniswapV2Pool> {
        match self {
            AmmPoolKind::UniswapV2(pool) => Some(pool),
            _ => None,
        }
    }

    #[inline]
    pub fn as_uniswap_v2_mut(&mut self) -> Option<&mut UniswapV2Pool> {
        match self {
            AmmPoolKind::UniswapV2(pool) => Some(pool),
            _ => None,
        }
    }
}

impl AmmPoolKind {
    #[inline]
    pub fn address(&self) -> &Address {
        match self {
            AmmPoolKind::UniswapV2(pool) => pool.address(),
        }
    }

    #[inline]
    pub fn dex(&self) -> &Arc<AmmProtocolKind> {
        match self {
            AmmPoolKind::UniswapV2(pool) => pool.dex(),
        }
    }

    #[inline]
    pub fn token0(&self) -> &Arc<CryptoToken> {
        match self {
            AmmPoolKind::UniswapV2(pool) => pool.token0(),
        }
    }

    #[inline]
    pub fn token1(&self) -> &Arc<CryptoToken> {
        match self {
            AmmPoolKind::UniswapV2(pool) => pool.token1(),
        }
    }
}

impl Display for AmmPoolKind {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            AmmPoolKind::UniswapV2(_) => write!(f, "UniswapV2"),
        }
    }
}
