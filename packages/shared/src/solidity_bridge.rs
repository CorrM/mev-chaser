use std::sync::Arc;

use anyhow::Result;
use ethers::{
    middleware::SignerMiddleware,
    signers::{LocalWallet, Signer, Wallet},
};
use ethers_contract::ContractError;
use ethers_core::{
    k256::ecdsa::SigningKey,
    types::{Address, TxHash, U256},
};
use ethers_providers::Middleware;

use contracts::{BalancerFlashLoanRecipientAbi, OneSwapInfo};

pub struct SolidityBridge<M: Middleware> {
    contract: BalancerFlashLoanRecipientAbi<SignerMiddleware<Arc<M>, Wallet<SigningKey>>>,
}

impl<M> SolidityBridge<M>
where
    M: Middleware + 'static,
{
    pub async fn new(address: Address, provider: Arc<M>, wallet_private_key: String) -> Result<Self> {
        let chain_id: U256 = provider.get_chainid().await?;

        let mut wallet_private_key = wallet_private_key;
        if wallet_private_key.starts_with("0x") {
            wallet_private_key = wallet_private_key.split_off(2);
        }

        let wallet: LocalWallet = wallet_private_key
            .parse::<LocalWallet>()?
            .with_chain_id(chain_id.as_u64());

        let signer = Arc::new(SignerMiddleware::new(provider, wallet));
        let contract = BalancerFlashLoanRecipientAbi::new(address, signer);

        Ok(Self { contract })
    }

    pub async fn deploy(provider: Arc<M>, wallet_private_key: String) -> Result<Address> {
        let chain_id: U256 = provider.get_chainid().await?;

        let mut wallet_private_key = wallet_private_key;
        if wallet_private_key.starts_with("0x") {
            wallet_private_key = wallet_private_key.split_off(2);
        }

        let wallet: LocalWallet = wallet_private_key
            .parse::<LocalWallet>()?
            .with_chain_id(chain_id.as_u64());

        let signer = Arc::new(SignerMiddleware::new(provider, wallet));
        let gg = BalancerFlashLoanRecipientAbi::deploy(signer, ())
            .expect("deploy failed")
            .send()
            .await;
        Ok(gg.unwrap().address())
    }

    pub async fn estimate_get_loan_then_swap_chain(
        &self,
        swaps: Vec<OneSwapInfo>,
        chain_swaps: bool,
        return_output: bool,
    ) -> Result<U256, ContractError<SignerMiddleware<Arc<M>, Wallet<SigningKey>>>> {
        self.contract
            .get_loan_then_multi_swap(swaps, chain_swaps, return_output)
            .estimate_gas()
            .await
    }

    pub async fn get_loan_then_swap_chain(
        &self,
        swaps: Vec<OneSwapInfo>,
        chain_swaps: bool,
        return_output: bool,
        gas_price: Option<U256>,
        max_fee_per_gas: Option<U256>,
        max_priority_fee_per_gas: Option<U256>,
    ) -> Result<TxHash, ContractError<SignerMiddleware<Arc<M>, Wallet<SigningKey>>>> {
        let mut call = self
            .contract
            .get_loan_then_multi_swap(swaps, chain_swaps, return_output)
            .gas(2_000_000);

        if gas_price.is_some() {
            call = call.legacy().gas_price(gas_price.unwrap());
        } else {
            let tx = call.tx.as_eip1559_mut().unwrap();
            tx.max_fee_per_gas = max_fee_per_gas;
            tx.max_priority_fee_per_gas = max_priority_fee_per_gas;
        }

        //let tx_hash = call.send().await?.await?.unwrap();
        //println!("Transaction Receipt: {}", serde_json::to_string(&tx_hash)?); // Use simd_json

        let tx_hash: TxHash = call.send().await?.tx_hash();
        Ok(tx_hash)
    }
}
