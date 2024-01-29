pub struct CryptoToken {
    address: String,
    name: String,
    symbol: String,
    decimals: u8,
    decimals_pow: f64,
}

impl CryptoToken {
    fn new(address: String, name: String, symbol: String, decimals: u8) -> Self {
        Self {
            address,
            name,
            symbol,
            decimals,
            decimals_pow: 10_i32.pow(decimals as u32) as f64,
        }
    }

    pub fn address(&self) -> &str {
        self.address.as_ref()
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn symbol(&self) -> &str {
        self.symbol.as_ref()
    }

    pub fn decimals(&self) -> u8 {
        self.decimals
    }

    fn convert_to_decimal(&self, value: i128) -> f64 {
        value as f64 / self.decimals_pow
    }

    fn convert_to_amount(&self, value: f64) -> i128 {
        (value * self.decimals_pow) as i128
    }
}
