use crate::UniswapV2Pool;

#[derive(Clone)]
pub enum AmmPoolKind {
    UniswapV2(UniswapV2Pool),
}