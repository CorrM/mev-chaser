use std::{str::FromStr, sync::Arc};

use anyhow::Result;
use ethers_core::types::{H160, U256};

use shared::{
    amm::AmmPool,
    network::NetworkKind,
    token::CryptoToken,
};
use shared::amm::AmmProtocol;

use crate::uniswap_v2_protocol::UniswapV2Protocol;

#[derive(Clone)]
pub struct UniswapV2Pool {
    address: H160,
    dex: Arc<UniswapV2Protocol>,
    //network: NetworkKind,
}

impl UniswapV2Pool {
    pub fn new(address: impl Into<String>, dex: Arc<UniswapV2Protocol>) -> Result<Self> {
        Ok(Self {
            address: H160::from_str(&address.into())?,
            dex: dex.clone(),
            //network: dex.network(),
        })
    }
}

impl AmmPool for UniswapV2Pool {
    fn address(&self) -> &H160 {
        &self.address
    }

    fn dex(&self) -> Arc<dyn AmmProtocol> {
        self.dex.clone()
    }

    fn network(&self) -> &NetworkKind {
        panic!()
    }

    fn token0(&self) -> &Arc<CryptoToken> {
        panic!()
    }

    fn token1(&self) -> &Arc<CryptoToken> {
        panic!()
    }

    fn reserve0(&self) -> U256 {
        panic!()
    }

    fn reserve1(&self) -> U256 {
        panic!()
    }

    fn update_reserve(&mut self) {
        panic!()
    }
}
