use ethers_core::types::{H160, U256};
use std::sync::Arc;

use crate::{network::network_kind::NetworkKind, token::crypto_token::CryptoToken};

use super::AmmProtocol;

pub trait AmmPool {
    fn address(&self) -> &H160;
    fn dex(&self) -> Arc<dyn AmmProtocol>;
    fn network(&self) -> &NetworkKind;
    fn token0(&self) -> &Arc<CryptoToken>;
    fn token1(&self) -> &Arc<CryptoToken>;
    fn reserve0(&self) -> U256;
    fn reserve1(&self) -> U256;
    fn update_reserve(&mut self);
}
