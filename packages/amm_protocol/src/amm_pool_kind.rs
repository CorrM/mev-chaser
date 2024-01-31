use protocol_uniswap_v2::UniswapV2Pool;

pub enum AmmPoolKind {
    UniswapV2(UniswapV2Pool),
    UniswapV3,
}