use anyhow::Result;
use ethers_core::types::{Address, U256};
use std::str::FromStr;

use crate::network::NetworkKind;

#[derive(Debug, Clone, PartialEq)]
pub struct CryptoToken {
    network: NetworkKind,
    address: Address,
    name: String,
    symbol: String,
    decimals: u8,
    decimals_pow: f64,
    one_token_amount: U256,
    input_token_unit: U256,
}

impl CryptoToken {
    pub fn new(
        network: &NetworkKind,
        address: impl Into<String>,
        name: impl Into<String>,
        symbol: impl Into<String>,
        decimals: u8,
    ) -> Result<Self> {
        let decimals_pow: f64 = 10_f64.powi(decimals as i32);

        Ok(Self {
            network: *network,
            address: Address::from_str(&address.into())?,
            name: name.into(),
            symbol: symbol.into(),
            decimals,
            decimals_pow,
            one_token_amount: U256::from((1_f64 * decimals_pow) as i128),
            input_token_unit: U256::from(10).pow(U256::from(decimals)),
        })
    }

    #[inline]
    pub fn network(&self) -> &NetworkKind {
        &self.network
    }

    #[inline]
    pub fn address(&self) -> &Address {
        &self.address
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    #[inline]
    pub fn decimals(&self) -> u8 {
        self.decimals
    }

    #[inline]
    pub fn one_token_amount(&self) -> U256 {
        self.one_token_amount
    }
    
    #[inline]
    pub fn input_token_unit(&self) -> U256 {
        self.input_token_unit
    }

    #[inline]
    pub fn convert_to_decimal(&self, value: U256) -> f64 {
        (value.as_u64() as f64) / self.decimals_pow
    }

    #[inline]
    pub fn convert_to_amount(&self, value: f64) -> U256 {
        U256::from((value * self.decimals_pow) as i128)
    }
}
