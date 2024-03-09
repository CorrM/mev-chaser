use std::{str::FromStr, sync::Arc};

use anyhow::Result;
use ethers::{
    abi::Token,
    contract::Multicall,
    providers::Middleware,
    types::{Address, Bytes},
};

use contracts::erc20_token::ERC20TokenAbi;
use shared::database::Database;
use shared::types::CryptoToken;
use vidger::types::NetworkKind;
use vidger::utilities::block_on;

pub struct AddTokenCommand;

impl AddTokenCommand {
    fn add_token_info<M: Middleware>(
        tokens: &[&str],
        db: &Database,
        target_network: &NetworkKind,
        provider: &Arc<M>,
    ) -> Result<()> {
        let mut multicall: Multicall<M> = block_on(Multicall::new(Arc::clone(provider), None)).unwrap();

        for token_address in tokens {
            // Can't execlude tokens here, because it will cause an error in the next for loop
            let token_contract = ERC20TokenAbi::new(Address::from_str(token_address).unwrap(), Arc::clone(provider));

            multicall.add_call(token_contract.name(), false);
            multicall.add_call(token_contract.symbol(), false);
            multicall.add_call(token_contract.decimals(), false);
        }

        let result: Vec<Result<Token, Bytes>> = block_on(multicall.call_raw()).unwrap();
        for i in (0..result.len()).step_by(3) {
            let token_address: &str = tokens[i / 3];
            if db.get_token_by_address(token_address, target_network)?.is_some() {
                let token_symbol: &Result<Token, Bytes> = &result[i + 1];
                let Ok(Token::String(token_symbol)) = token_symbol else {
                    panic!("Failed to get token name");
                };

                println!("Token {} '{}' already exists", token_symbol, token_address);
                continue;
            }

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

            println!("Adding token {} '{}'", token_symbol, token_address);
            panic!("TODO"); // TODO
            let token_add = db.add_token(
                target_network,
                &CryptoToken::new(token_address, None, token_name, token_symbol, token_decimals, 0)?,
            );
            if token_add.is_err() {
                println!("Failed to add token {}", token_address);
                continue;
            }
        }

        Ok(())
    }

    pub fn process<M: Middleware>(
        tokens: Vec<&str>,
        db: &Database,
        target_network: &NetworkKind,
        provider: Arc<M>,
    ) -> Result<()> {
        let tokens_cnt: f32 = tokens.len() as f32;
        let batch: f32 = (tokens_cnt / 80_f32).ceil();
        let tokens_per_batch: usize = (tokens_cnt / batch).ceil() as usize;
        let tokens_cnt: usize = tokens_cnt as usize;

        for i in 0..(batch as usize) {
            let start_idx: usize = i * tokens_per_batch;
            let end_idx: usize = std::cmp::min(start_idx + tokens_per_batch, tokens_cnt);

            AddTokenCommand::add_token_info(&tokens[start_idx..end_idx], db, target_network, &provider)?;
        }

        Ok(())
    }
}
