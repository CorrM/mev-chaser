use std::{str::FromStr, sync::Arc};

use anyhow::Result;
use ethers_core::types::{Address, U256};

use shared::{
    network::NetworkKind,
    token::CryptoToken,
};

use crate::{uniswap_v2_protocol::UniswapV2Protocol, AmmPool, AmmProtocol};

#[derive(Clone)]
pub struct UniswapV2Pool {
    address: Address,
    dex: Arc<UniswapV2Protocol>,
    //network: NetworkKind,
}

impl UniswapV2Pool {
    pub fn new(address: Address, dex: Arc<UniswapV2Protocol>) -> Result<Self> {
        Ok(Self {
            address,
            dex: dex.clone(),
            //network: dex.network(),
        })
    }
}

impl AmmPool for UniswapV2Pool {
    type Protocol = UniswapV2Protocol;

    fn address(&self) -> &Address {
        &self.address
    }
    
    fn dex(&self) -> Arc<dyn AmmProtocol<Pool = Self>> {
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
