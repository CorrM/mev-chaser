use std::sync::Arc;

use ethers::{types::Address, utils::to_checksum};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use vidger::types::{CryptoToken, NetworkKind};

pub struct TokenManager {
    tokens: Vec<Arc<CryptoToken>>,
    native_token: Arc<CryptoToken>,
}

impl TokenManager {
    pub fn new(tokens: Vec<CryptoToken>, network: &NetworkKind) -> Self {
        let tokens: Vec<Arc<CryptoToken>> = tokens.into_iter().map(Arc::new).collect();
        let native_token_address: &str = match network {
            NetworkKind::Ethereum => "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2",
            NetworkKind::Polygon => "0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270",
        };

        let native_token: Arc<CryptoToken> = Self::get_by_address_str_impl(&tokens, native_token_address).unwrap();
        Self { tokens, native_token }
    }

    fn get_by_address_str_impl(tokens: &[Arc<CryptoToken>], address: &str) -> Option<Arc<CryptoToken>> {
        tokens
            .iter()
            .find(|token| to_checksum(token.address(), None) == address)
            .cloned()
    }
}

impl TokenManager {
    #[inline]
    pub fn tokens(&self) -> &Vec<Arc<CryptoToken>> {
        &self.tokens
    }

    #[inline]
    pub fn native_token(&self) -> &Arc<CryptoToken> {
        &self.native_token
    }

    pub fn get_by_address(&self, address: &Address) -> Option<Arc<CryptoToken>> {
        self.tokens
            .par_iter()
            .find_first(|token| token.address() == address)
            .cloned()
    }

    pub fn get_by_address_str(&self, address: &str) -> Option<Arc<CryptoToken>> {
        Self::get_by_address_str_impl(&self.tokens, address)
    }

    pub fn get_by_symbol(&self, symbol: &str) -> Option<Arc<CryptoToken>> {
        self.tokens
            .par_iter()
            .find_first(|token| token.symbol() == symbol)
            .cloned()
    }
}
