use ethers_core::types::{Address, U256};
use shared::{network::NetworkKind, token::CryptoToken};
use std::sync::Arc;

use crate::AmmProtocol;

pub trait AmmPool {
    type Protocol: AmmProtocol;

    fn address(&self) -> &Address;
    fn dex(&self) -> Arc<dyn AmmProtocol<Pool = Self>>;
    fn network(&self) -> &NetworkKind;
    fn token0(&self) -> &Arc<CryptoToken>;
    fn token1(&self) -> &Arc<CryptoToken>;
    fn reserve0(&self) -> U256;
    fn reserve1(&self) -> U256;
    fn update_reserve(&mut self, reserve0: U256, reserve1: U256);
}
