use anyhow::Result;
use ethers_core::types::{Address, U256};
use std::str::FromStr;

use crate::network::NetworkKind;

#[derive(Clone, PartialEq)]
pub struct CryptoToken {
    network: NetworkKind,
    address: Address,
    name: String,
    symbol: String,
    decimals: u8,
    decimals_pow: f64,
}

impl CryptoToken {
    pub fn new(
        network: &NetworkKind,
        address: impl Into<String>,
        name: impl Into<String>,
        symbol: impl Into<String>,
        decimals: u8,
    ) -> Result<Self> {
        Ok(Self {
            network: *network,
            address: Address::from_str(&address.into())?,
            name: name.into(),
            symbol: symbol.into(),
            decimals,
            decimals_pow: (10 as f64).powi(decimals as i32),
        })
    }

    pub fn network(&self) -> &NetworkKind {
        &self.network
    }

    pub fn address(&self) -> &Address {
        &self.address
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn decimals(&self) -> u8 {
        self.decimals
    }

    pub fn convert_to_decimal(&self, value: U256) -> f64 {
        (value.as_u64() as f64) / self.decimals_pow
    }

    pub fn convert_to_amount(&self, value: f64) -> U256 {
        U256::from((value * self.decimals_pow) as u64)
    }
}
