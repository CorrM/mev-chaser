use protocol_uniswap_v2::UniswapV2Protocol;

pub enum AmmProtocolKind {
    UniswapV2(UniswapV2Protocol),
    UniswapV3,
}