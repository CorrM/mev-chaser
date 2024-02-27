use ethers::types::Address;
use std::sync::Arc;
use vidger::types::CryptoToken;

use crate::amm::AmmProtocol;

pub trait AmmPool: Send + Sync {
    fn address(&self) -> &Address;
    fn dex(&self) -> Arc<dyn AmmProtocol>;
    fn token0(&self) -> &Arc<CryptoToken>;
    fn token1(&self) -> &Arc<CryptoToken>;
}
