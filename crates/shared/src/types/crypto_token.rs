use std::str::FromStr;

use anyhow::Result;
use ethers::types::{Address, U256};
use revm::primitives::bytes;

#[derive(Debug, Clone)]
pub struct CryptoToken {
    address: Address,
    proxy_address: Option<Address>,
    name: String,
    symbol: String,
    decimals: u8,
    decimals_pow: f64,
    one_token_amount: U256,
    input_token_unit: U256,
    balance_contract_slot: i32,
    code: bytes::Bytes,
}

impl CryptoToken {
    pub fn new(
        address: impl Into<String>,
        proxy_address: Option<impl Into<String>>,
        name: impl Into<String>,
        symbol: impl Into<String>,
        decimals: u8,
        balance_contract_slot: i32,
        code: bytes::Bytes,
    ) -> Result<Self> {
        let decimals_pow: f64 = 10_f64.powi(decimals as i32);

        Ok(Self {
            address: Address::from_str(&address.into())?,
            proxy_address: proxy_address.map(|a| Address::from_str(&a.into()).unwrap()),
            name: name.into(),
            symbol: symbol.into(),
            decimals,
            decimals_pow,
            one_token_amount: U256::from((1_f64 * decimals_pow) as i128),
            input_token_unit: U256::from(10).pow(U256::from(decimals)),
            balance_contract_slot,
            code,
        })
    }
}

impl CryptoToken {
    #[inline]
    pub fn address(&self) -> &Address {
        &self.address
    }

    #[inline]
    pub fn proxy_address(&self) -> &Option<Address> {
        &self.proxy_address
    }

    #[inline]
    pub fn proxy_or_address(&self) -> &Address {
        match self.proxy_address {
            Some(ref proxy) => proxy,
            None => &self.address,
        }
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
    pub fn balance_contract_slot(&self) -> i32 {
        self.balance_contract_slot
    }

    #[inline]
    pub fn code(&self) -> &bytes::Bytes {
        &self.code
    }

    #[inline]
    pub fn convert_to_decimal(&self, value: U256) -> f64 {
        ethers::utils::format_units(value, self.decimals as u32)
            .unwrap()
            .parse::<f64>()
            .unwrap()
        //(value.as_u64() as f64) / self.decimals_pow
    }

    #[inline]
    pub fn convert_to_amount(&self, value: f64) -> U256 {
        U256::from((value * self.decimals_pow) as i128)
    }
}
