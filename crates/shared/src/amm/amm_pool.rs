use std::sync::Arc;

use ethers::types::Address;

use crate::amm::AmmProtocolKind;
use crate::types::CryptoToken;

pub(super) trait AmmPool: Send + Sync {
    fn address(&self) -> &Address;
    fn dex(&self) -> &Arc<AmmProtocolKind>;
    fn token0(&self) -> &Arc<CryptoToken>;
    fn token1(&self) -> &Arc<CryptoToken>;
}
