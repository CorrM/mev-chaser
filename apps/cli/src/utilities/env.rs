use std::str::FromStr;

use anyhow::{anyhow, Result};

fn get_env(key: &str) -> Result<String> {
    let result = std::env::var(key);
    if let Err(e) = &result {
        return Err(anyhow!("{}: {}", key, e));
    }

    Ok(result.unwrap())
}

#[derive(Debug, Clone)]
pub struct Env {
    pub https_url: String,
    pub wss_url: String,
    pub blockpi_api_key: String,
    pub chain_id: u32,
    pub private_key: String,
    pub bot_address: String,
    pub telegram_token_id: String,
    pub telegram_channel_id: String,
}

impl Env {
    pub fn new() -> Result<Self> {
        let https_url: String = get_env("HTTPS_URL")?;
        let wss_url: String = get_env("WSS_URL")?;
        let blockpi_api_key: String = get_env("BLOCKPI_API_KEY")?;
        let chain_id: u32 = u32::from_str(&get_env("CHAIN_ID")?).unwrap();
        let private_key: String = get_env("PRIVATE_KEY")?;
        let bot_address: String = get_env("BOT_ADDRESS")?;
        let telegram_token_id: String = get_env("TELEGRAM_TOKEN_ID")?;
        let telegram_channel_id: String = get_env("TELEGRAM_CHANNEL_ID")?;

        Ok(Env {
            https_url,
            wss_url,
            blockpi_api_key,
            chain_id,
            private_key,
            bot_address,
            telegram_token_id,
            telegram_channel_id,
        })
    }
}
