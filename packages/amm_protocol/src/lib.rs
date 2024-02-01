pub use protocol_uniswap_v2::*;

pub enum AmmProtocolContainer { // TODO: DELETE THIS PLS
    UniswapV2(UniswapV2Protocol),
    UniswapV3
}