use std::sync::Arc;

use anyhow::{anyhow, Result};
use ethers_core::{abi::Token, types::{Address, Bytes, TxHash, U256}};
use ethers_providers::{Provider, Ws};

use contracts::{BalancerFlashLoanRecipientAbi, OneSwapInfo};
pub struct SolidityBridge {
    contract: BalancerFlashLoanRecipientAbi<Provider<Ws>>,
}

impl SolidityBridge {
    pub fn new(address: Address, provider: Arc<Provider<Ws>>) -> Self {
        Self {
            contract: BalancerFlashLoanRecipientAbi::new(address, provider),
        }
    }

    pub fn make_uniswap_v2_protocol_swap_info(
        router: Address,
        path: Vec<Address>,
        amount_in: impl Into<U256>,
        amount_out_min: impl Into<U256>,
    ) -> Result<OneSwapInfo> {
        if path.len() < 2 {
            return Err(anyhow!("path must have at least 2 elements"));
        }

        let token_in: Address = path[0];
        let path_token: Vec<Token> = path.into_iter().map(Token::Address).collect();
        let encoded_path: Bytes = ethers::abi::encode(&path_token).into();

        Ok(OneSwapInfo {
            protocol: 0,
            router,
            token_in,
            path: encoded_path,
            amount_in: amount_in.into(),
            amount_out_min: amount_out_min.into(),
            deadline: U256::from(0)
        })
    }

    pub async fn estimate_get_loan_then_swap_chain(&self, swaps: Vec<OneSwapInfo>, chain_swaps: bool, return_output: bool) -> Result<U256> {
        Ok(self
            .contract
            .get_loan_then_multi_swap(swaps, chain_swaps, return_output)
            .estimate_gas()
            .await?)
    }

    pub async fn get_loan_then_swap_chain(
        &self,
        swaps: Vec<OneSwapInfo>,
        chain_swaps: bool,
        return_output: bool,
        gas_price: impl Into<U256>,
    ) -> Result<TxHash> {
        Ok(self
            .contract
            .get_loan_then_multi_swap(swaps, chain_swaps, return_output)
            .gas_price(gas_price)
            .send()
            .await?
            .tx_hash())
    }
}
