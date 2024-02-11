use anyhow::Result;
use ethers_core::types::{Address, U256};
use std::sync::Arc;

use shared::{network::NetworkKind, token::CryptoToken};

use crate::{uniswap_v2_protocol::UniswapV2Protocol, AmmPool, AmmPoolKind, AmmProtocol};

#[derive(Clone)]
pub struct UniswapV2Pool {
    address: Address,
    dex: Arc<UniswapV2Protocol>,
    network: NetworkKind,
    token0: Arc<CryptoToken>,
    token1: Arc<CryptoToken>,
    reserve0: U256,
    reserve1: U256,
}

impl UniswapV2Pool {
    pub fn new(
        address: Address,
        dex: Arc<UniswapV2Protocol>,
        network: NetworkKind,
        token0: Arc<CryptoToken>,
        token1: Arc<CryptoToken>,
    ) -> Result<Self> {
        Ok(Self {
            address,
            dex,
            network,
            token0,
            token1,
            reserve0: U256::zero(),
            reserve1: U256::zero(),
        })
    }
}

impl AmmPool for UniswapV2Pool {
    fn kind(&self) -> AmmPoolKind {
        AmmPoolKind::UniswapV2
    }

    fn address(&self) -> &Address {
        &self.address
    }

    fn dex(&self) -> Arc<dyn AmmProtocol> {
        self.dex.clone()
    }

    fn network(&self) -> &NetworkKind {
        &self.network
    }

    fn token0(&self) -> &Arc<CryptoToken> {
        &self.token0
    }

    fn token1(&self) -> &Arc<CryptoToken> {
        &self.token1
    }

    fn reserve0(&self) -> U256 {
        self.reserve0
    }

    fn reserve1(&self) -> U256 {
        self.reserve1
    }

    fn update_reserve(&mut self, reserve0: &U256, reserve1: &U256) {
        self.reserve0 = *reserve0;
        self.reserve1 = *reserve1;
    }
}
