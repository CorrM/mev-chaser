use std::sync::Arc;

use anyhow::Result;
use ethers::types::Address;
use ethers::utils::to_checksum;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::database::{Database, DbToken, DbTokenNetwork};
use vidger::types::NetworkKind;

use crate::types::CryptoToken;

pub struct TokenManager {
    tokens: Vec<Arc<CryptoToken>>,
    native_token: Arc<CryptoToken>,
}

impl TokenManager {
    fn get_by_address_str_impl<'a>(tokens: &'a [Arc<CryptoToken>], address: &str) -> Option<&'a Arc<CryptoToken>> {
        tokens
            .iter()
            .find(|token| to_checksum(token.address(), None) == address)
    }

    fn get_tokens(db: &Database, network: &NetworkKind) -> Result<Vec<CryptoToken>> {
        let db_tokens: Vec<(DbToken, DbTokenNetwork)> = db.get_tokens(network)?;
        let mut tokens: Vec<CryptoToken> = Vec::new();

        for (db_token, db_token_network) in db_tokens {
            tokens.push(CryptoToken::new(
                db_token_network.address,
                db_token_network.proxy,
                db_token.name,
                db_token.symbol,
                db_token.decimals as u8,
                db_token_network.balance_contract_slot,
                db_token_network.code,
            )?);
        }

        Ok(tokens)
    }

    pub fn new(tokens: Vec<CryptoToken>, network: &NetworkKind) -> Self {
        let tokens: Vec<Arc<CryptoToken>> = tokens.into_iter().map(Arc::new).collect();

        let native_token_address: &str = match network {
            NetworkKind::Ethereum => "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2",
            NetworkKind::Polygon => "0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270",
        };

        let native_token: Arc<CryptoToken> =
            Arc::clone(Self::get_by_address_str_impl(&tokens, native_token_address).unwrap());
        Self { tokens, native_token }
    }

    pub fn new_by_db(db: &Database, network: &NetworkKind) -> Result<Self> {
        Ok(Self::new(Self::get_tokens(db, network)?, network))
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

    #[inline]
    pub fn get_by_address(&self, address: &Address) -> Option<&Arc<CryptoToken>> {
        self.tokens.par_iter().find_first(|token| *token.address() == *address)
    }

    #[inline]
    pub fn get_by_address_str(&self, address: &str) -> Option<&Arc<CryptoToken>> {
        Self::get_by_address_str_impl(&self.tokens, address)
    }

    #[inline]
    pub fn get_by_symbol(&self, symbol: &str) -> Option<&Arc<CryptoToken>> {
        self.tokens.par_iter().find_first(|token| token.symbol() == symbol)
    }
}
