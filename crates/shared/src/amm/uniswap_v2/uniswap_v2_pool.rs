use std::sync::Arc;

use anyhow::Result;
use ethers::types::Address;

use vidger::types::CryptoToken;

use crate::amm::{AmmPool, AmmProtocolKind};

#[derive(Clone)]
pub struct UniswapV2Pool {
    address: Address,
    dex: Arc<AmmProtocolKind>,
    token0: Arc<CryptoToken>,
    token1: Arc<CryptoToken>,
}

impl UniswapV2Pool {
    pub(crate) fn new(
        address: Address,
        dex: Arc<AmmProtocolKind>,
        token0: Arc<CryptoToken>,
        token1: Arc<CryptoToken>,
    ) -> Result<Self> {
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
