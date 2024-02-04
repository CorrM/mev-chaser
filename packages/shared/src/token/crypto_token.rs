use anyhow::Result;
use ethers_core::types::{Address, U256};
use std::str::FromStr;

use crate::network::NetworkKind;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CryptoToken {
    network: NetworkKind,
    address: Address,
    name: String,
    symbol: String,
    decimals: u8,
    decimals_pow: U256,
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
            decimals_pow: U256::from(10).pow(U256::from(decimals)),
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
        (value.as_u64() as f64) / (self.decimals_pow.as_u64() as f64)
    }

    pub fn convert_to_amount(&self, value: f64) -> U256 {
        U256::from((value.powi(self.decimals as i32)) as u64) * self.decimals_pow
    }
}
