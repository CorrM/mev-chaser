use std::sync::Arc;

use alloy_primitives::Address;
use anyhow::{anyhow, Result};

use crate::amm::{AmmPool, AmmProtocolKind};
use crate::types::CryptoToken;

#[derive(Clone)]
pub struct UniswapV2Pool {
    address: Address,
    dex: Arc<AmmProtocolKind>,
    token0: Arc<CryptoToken>,
    token1: Arc<CryptoToken>,
}

impl UniswapV2Pool {
    pub fn new(
        address: Address,
        dex: Arc<AmmProtocolKind>,
        token0: Arc<CryptoToken>,
        token1: Arc<CryptoToken>,
    ) -> Result<Self> {
        if !dex.is_uniswap_v2() {
            return Err(anyhow!("UniswapV2Pool must be created with UniswapV2Protocol"));
        }

        Ok(Self {
            address,
            dex,
            token0,
            token1,
        })
    }
}

impl AmmPool for UniswapV2Pool {
    #[inline]
    fn address(&self) -> &Address {
        &self.address
    }

    #[inline]
    fn dex(&self) -> &Arc<AmmProtocolKind> {
        &self.dex
    }

    #[inline]
    fn token0(&self) -> &Arc<CryptoToken> {
        &self.token0
    }

    #[inline]
    fn token1(&self) -> &Arc<CryptoToken> {
        &self.token1
    }
}
