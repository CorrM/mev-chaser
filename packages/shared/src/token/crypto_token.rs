use anyhow::Result;
use std::str::FromStr;
use ethers_core::types::{H160, U256};

pub struct CryptoToken {
    address: H160,
    name: String,
    symbol: String,
    decimals: u8,
    decimals_pow: U256,
}

impl CryptoToken {
    pub fn new(address: impl Into<String>, name: impl Into<String>, symbol: impl Into<String>, decimals: u8) -> Result<Self> {
        //ethers_core::utils::;

        Ok(Self {
            address: H160::from_str(&address.into())?,
            name: name.into(),
            symbol: symbol.into(),
            decimals,
            decimals_pow: U256::exp10(decimals as usize),
        })
    }

    pub fn address(&self) -> &H160 {
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

    fn convert_to_decimal(&self, value: U256) -> String {
        let integer = value / self.decimals_pow;
        (value % self.decimals_pow).to_string()
    }

    fn convert_to_amount(&self, value: f64) -> U256 {
        panic!("Not implemented");
        //(value * self.decimals_pow)
    }
}
