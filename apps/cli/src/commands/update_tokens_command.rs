use std::str::FromStr;
use std::sync::Arc;

use anyhow::Result;
use ethers::{
    abi::Token, addressbook::Address, contract::Multicall, middleware::Middleware, types::Bytes, utils::to_checksum,
};
use hashbrown::HashMap;

use contracts::erc20_token::ERC20TokenAbi;
use shared::amm::{AmmPoolKind, AmmProtocolKind, UniswapV2Pool, UniswapV2Protocol};
use shared::{
    database::{Database, DbToken, DbTokenNetwork},
    managers::AmmManager,
    simulator::EvmSimulator,
    types::CryptoToken,
    utilities::get_proxy_implementation,
};
use vidger::{
    logger::{error, info},
    types::NetworkKind,
    utilities::block_on,
};

// TODO:
// I know that command not work with all tokens but, I only need to get info for starting tokens
// So, it is ok because they are stored in top of database

pub struct UpdateTokenCommand;

impl UpdateTokenCommand {
    fn update_token_info<M: Middleware + 'static>(
        ethers_api_key: &str,
        tokens: &[(DbToken, DbTokenNetwork)],
        db: &Database,
        target_network: &NetworkKind,
        provider: &Arc<M>,
    ) -> Result<()> {
        let amm_manager = AmmManager::new(Vec::new());
        let mut simulator = EvmSimulator::new(Arc::clone(provider), &amm_manager)?;
        let mut multicall: Multicall<M> = block_on(Multicall::new(Arc::clone(provider), None)).unwrap();

        for token in tokens {
            // Can't exclude tokens here, because it will cause an error in the next for loop
            let token_contract = ERC20TokenAbi::new(Address::from_str(&token.1.address).unwrap(), Arc::clone(provider));

            multicall.add_call(token_contract.name(), false);
            multicall.add_call(token_contract.symbol(), false);
            multicall.add_call(token_contract.decimals(), false);
        }

        let result: Vec<Result<Token, Bytes>> = block_on(multicall.call_raw()).unwrap();
        for i in (0..result.len()).step_by(3) {
            let token: &(DbToken, DbTokenNetwork) = &tokens[i / 3];
            let token_address_str: &String = &token.1.address;
            let token_address: Address = Address::from_str(token_address_str)?;

            info!("Updating token {}", token_address_str);

            let token_name: &Result<Token, Bytes> = &result[i];
            let token_symbol: &Result<Token, Bytes> = &result[i + 1];
            let token_decimals: &Result<Token, Bytes> = &result[i + 2];
            let Ok(Token::String(token_name)) = token_name else {
                panic!("Failed to get token name");
            };
            let Ok(Token::String(token_symbol)) = token_symbol else {
                panic!("Failed to get token symbol");
            };
            let Ok(Token::Uint(token_decimals)) = token_decimals else {
                panic!("Failed to get token decimals");
            };
            let token_decimals = token_decimals.as_u32() as u8;

            // Get code
            let token_code: Bytes = block_on(provider.get_code(token_address, None))?;

            // Get proxy
            let proxy_address: Option<Address> =
                get_proxy_implementation(ethers_api_key, provider, token_address).map(|kind_address| kind_address.1);

            // Get balance slot
            let proxy_or_address: Address = proxy_address.unwrap_or(token_address);
            let slot: HashMap<Address, Result<Option<i32>>> = simulator.get_tokens_balance_slot(&[proxy_or_address])?;
            let slot: &Result<Option<i32>> = slot.get(&proxy_or_address).unwrap();
            let slot: i32 = slot.as_ref().unwrap().unwrap_or(-1); // TODO: Should not just throw -1

            // Create new data
            let token_new_data = CryptoToken::new(
                token_address_str,
                proxy_address.map(|pa| to_checksum(&pa, None)),
                token_name,
                token_symbol,
                token_decimals,
                slot,
                token_code.0,
            )?;
            let token_add = db.update_token(target_network, &token_new_data);
            if token_add.is_err() {
                error!("Failed to update token {}", token_address_str);
                continue;
            }

            info!("Updating token {} ... Done", token_address_str);
        }

        Ok(())
    }

    pub fn process<M: Middleware + 'static>(
        ethers_api_key: String,
        db: &Database,
        target_network: &NetworkKind,
        provider: &Arc<M>,
    ) -> Result<()> {
        let tokens: Vec<(DbToken, DbTokenNetwork)> = db.get_tokens(target_network)?;

        let tokens_cnt: f32 = tokens.len() as f32;
        let batch: f32 = (tokens_cnt / 80_f32).ceil();
        let tokens_per_batch: usize = (tokens_cnt / batch).ceil() as usize;
        let tokens_cnt: usize = tokens_cnt as usize;

        for i in 0..(batch as usize) {
            let start_idx: usize = i * tokens_per_batch;
            let end_idx: usize = std::cmp::min(start_idx + tokens_per_batch, tokens_cnt);

            Self::update_token_info(
                &ethers_api_key,
                &tokens[start_idx..end_idx],
                db,
                target_network,
                provider,
            )?;
        }

        Ok(())
    }
}
