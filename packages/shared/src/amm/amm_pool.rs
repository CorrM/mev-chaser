use crate::{network::network_kind::NetworkKind, token::crypto_token::CryptoToken};

use super::AmmProtocol;

pub trait AmmPool {
    fn address(&self) -> &str;
    fn dex(&self) -> &impl AmmProtocol;
    fn network(&self) -> NetworkKind;
    fn token0(&self) -> &CryptoToken;
    fn token1(&self) -> &CryptoToken;
    fn reserve0(&self) -> i128;
    fn reserve1(&self) -> i128;
    fn update_reserve(&mut self);
}