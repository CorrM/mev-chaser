use crate::uniswap_v2_protocol::UniswapV2Protocol;
use shared::{
    amm::{AmmPool, AmmProtocol},
    network::NetworkKind,
    token::CryptoToken,
};
use std::sync::Arc;

pub struct UniswapV2Pool {
    address: String,
    dex: Arc<UniswapV2Protocol>,
    network: NetworkKind,
}

impl UniswapV2Pool {
    pub fn new(address: impl Into<String>, dex: Arc<UniswapV2Protocol>) -> Self {
        Self {
            address: address.into(),
            dex: dex.clone(),
            network: dex.network(),
        }
    }
}

impl AmmPool for UniswapV2Pool {
    fn address(&self) -> &str {
        &self.address
    }

    fn dex(&self) -> &impl AmmProtocol {
        self.dex.as_ref()
    }

    fn network(&self) -> NetworkKind {
        
    }

    fn token0(&self) -> &CryptoToken {
    }

    fn token1(&self) -> &CryptoToken {
    }

    fn reserve0(&self) -> i128 {
    }

    fn reserve1(&self) -> i128 {
    }

    fn update_reserve(&mut self) {
    }
}
