use std::sync::Arc;

use anyhow::{anyhow, Result};
use ethers::{
    middleware::SignerMiddleware,
    signers::{LocalWallet, Signer, Wallet},
};
use ethers_core::{
    abi::Token,
    k256::ecdsa::SigningKey,
    types::{Address, Bytes, U256, TxHash},
};
use ethers_providers::{Middleware, Provider, Ws};

use contracts::{BalancerFlashLoanRecipientAbi, OneSwapInfo};
pub struct SolidityBridge {
    contract: BalancerFlashLoanRecipientAbi<SignerMiddleware<Arc<Provider<Ws>>, Wallet<SigningKey>>>,
}

impl SolidityBridge {
    pub async fn new(address: Address, provider: Arc<Provider<Ws>>, wallet_private_key: String) -> Result<Self> {
        let chain_id = provider.get_chainid().await?;

        let mut wallet_private_key = wallet_private_key;
        if wallet_private_key.starts_with("0x") {
            wallet_private_key = wallet_private_key.split_off(2);
        }

        let wallet: LocalWallet = wallet_private_key
            .parse::<LocalWallet>()?
            .with_chain_id(chain_id.as_u64());

        let signer = Arc::new(SignerMiddleware::new(provider, wallet.with_chain_id(chain_id.as_u64())));
        let contract = BalancerFlashLoanRecipientAbi::new(address, signer);

        Ok(Self { contract })
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
            deadline: U256::from(0),
        })
    }

    pub async fn estimate_get_loan_then_swap_chain(
        &self,
        swaps: Vec<OneSwapInfo>,
        chain_swaps: bool,
        return_output: bool,
    ) -> Result<U256> {
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
        gas_price: Option<U256>,
        max_fee_per_gas: Option<U256>,
        max_priority_fee_per_gas: Option<U256>,
    ) -> Result<TxHash> {
        let mut call = self
            .contract
            .get_loan_then_multi_swap(swaps, chain_swaps, return_output);

        if gas_price.is_some() {
            call = call.legacy().gas_price(gas_price.unwrap());
        } else {
            let tx = call.tx.as_eip1559_mut().unwrap();
            tx.max_fee_per_gas = max_fee_per_gas;
            tx.max_priority_fee_per_gas = max_priority_fee_per_gas;
        }

        let tx_hash: TxHash = call.send().await?.tx_hash();
        Ok(tx_hash)
    }
}
