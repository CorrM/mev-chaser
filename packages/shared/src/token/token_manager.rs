use std::sync::Arc;

use ethers_core::{types::Address, utils::to_checksum};

use super::CryptoToken;

pub struct TokenManager {
    tokens: Vec<Arc<CryptoToken>>,
}

impl TokenManager {
    pub fn new(tokens: Vec<CryptoToken>) -> Self {
        Self {
            tokens: tokens.into_iter().map(Arc::new).collect(),
        }
    }

    pub fn tokens(&self) -> &Vec<Arc<CryptoToken>> {
        &self.tokens
    }

    pub fn get_token_by_address(&self, address: &Address) -> Option<Arc<CryptoToken>> {
        self.tokens
            .iter()
            .find(|token| token.address() == address)
            .cloned()
    }
    
    pub fn get_token_by_address_str(&self, address: &str) -> Option<Arc<CryptoToken>> {
        self.tokens
            .iter()
            .find(|token| to_checksum(token.address(), None) == address)
            .cloned()
    }
}
