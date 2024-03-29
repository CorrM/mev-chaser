use std::sync::Arc;

use anyhow::{anyhow, Result};
use ethers::types::Address;

use crate::amm::{AmmPool, AmmProtocolKind};
use crate::types::CryptoToken;

/// UniswapV2 Pool layout
/// | Name                 | Type                                            | Slot | Offset | Bytes |
/// |----------------------|-------------------------------------------------|------|--------|-------|
/// | totalSupply          | uint256                                         | 0    | 0      | 32    |
/// | balanceOf            | mapping(address => uint256)                     | 1    | 0      | 32    |
/// | allowance            | mapping(address => mapping(address => uint256)) | 2    | 0      | 32    |
/// | DOMAIN_SEPARATOR     | bytes32                                         | 3    | 0      | 32    |
/// | nonces               | mapping(address => uint256)                     | 4    | 0      | 32    |
/// | factory              | address                                         | 5    | 0      | 20    |
/// | token0               | address                                         | 6    | 0      | 20    |
/// | token1               | address                                         | 7    | 0      | 20    |
/// | reserve0             | uint112                                         | 8    | 0      | 14    |
/// | reserve1             | uint112                                         | 8    | 14     | 14    |
/// | blockTimestampLast   | uint32                                          | 8    | 28     | 4     |
/// | price0CumulativeLast | uint256                                         | 9    | 0      | 32    |
/// | price1CumulativeLast | uint256                                         | 10   | 0      | 32    |
/// | kLast                | uint256                                         | 11   | 0      | 32    |
/// | unlocked             | uint256                                         | 12   | 0      | 32    |
///
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
