use crate::amm::UniswapV2Pool;

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

    #[inline]
    pub fn router(&self) -> &str {
        match self {
            AmmPoolKind::UniswapV2(pool) => pool.dex().router(),
        }
    }

    #[inline]
    pub fn factory(&self) -> &str {
        match self {
            AmmPoolKind::UniswapV2(pool) => pool.dex().factory(),
        }
    }
}
