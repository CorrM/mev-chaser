use std::{env::VarError, str::FromStr};

fn get_env(key: &str) -> Result<String, VarError> {
    std::env::var(key)
}

#[derive(Debug, Clone)]
pub struct Env {
    pub https_url: String,
    pub wss_url: String,
    pub blockpi_api_key: String,
    pub chain_id: u32,
    pub private_key: String,
    pub signing_key: String,
    pub bot_address: String,
}

impl Env {
    pub fn new() -> Result<Self, VarError> {
        let https_url: String = get_env("HTTPS_URL")?;
        let wss_url: String = get_env("WSS_URL")?;
        let blockpi_api_key: String = get_env("BLOCKPI_API_KEY")?;
        let chain_id: u32 = u32::from_str(&get_env("CHAIN_ID")?).unwrap();
        let private_key: String = get_env("PRIVATE_KEY")?;
        let signing_key: String = get_env("SIGNING_KEY")?;
        let bot_address: String = get_env("BOT_ADDRESS")?;

        Ok(Env {
            https_url,
            wss_url,
            blockpi_api_key,
            chain_id,
            private_key,
            signing_key,
            bot_address,
        })
    }
}