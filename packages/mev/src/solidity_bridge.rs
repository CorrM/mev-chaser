use std::{str::FromStr, sync::Arc};

use anyhow::Result;
use ethers::{
    middleware::SignerMiddleware,
    signers::{LocalWallet, Signer, Wallet},
    types::{Bytes, Transaction, I256},
    utils,
};
use ethers_contract::{ContractError, FunctionCall};
use ethers_core::{
    k256::ecdsa::SigningKey,
    types::{Address, TxHash, U256},
};
use ethers_providers::Middleware;

use contracts::{BalancerFlashLoanRecipientAbi, FastLaneAuctionHandlerAbi, OneSwapInfo};

use crate::{fast_bundle_provider, BundleProvider};

type GetLoanThenSwapChainCall<M> =
    FunctionCall<Arc<SignerMiddleware<Arc<M>, Wallet<SigningKey>>>, SignerMiddleware<Arc<M>, Wallet<SigningKey>>, I256>;

pub struct SolidityBridge<M: Middleware> {
    contract: BalancerFlashLoanRecipientAbi<SignerMiddleware<Arc<M>, Wallet<SigningKey>>>,
    fast_lane_contract: FastLaneAuctionHandlerAbi<SignerMiddleware<Arc<M>, Wallet<SigningKey>>>,
    bundle_provider: Arc<BundleProvider>,
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
        let contract = BalancerFlashLoanRecipientAbi::new(address, Arc::clone(&signer));

        let fast_lane_address: Address = Address::from_str("0xf5DF545113DeE4DF10f8149090Aa737dDC05070a")?;
        let fast_lane_contract = FastLaneAuctionHandlerAbi::new(fast_lane_address, signer);

        Ok(Self {
            contract,
            fast_lane_contract,
            bundle_provider: fast_bundle_provider().await,
        })
    }

    fn get_loan_then_swap_chain_call(
        &self,
        swaps: Vec<OneSwapInfo>,
        chain_swaps: bool,
        return_output: bool,
        gas_price: Option<U256>,
        max_fee_per_gas: Option<U256>,
        max_priority_fee_per_gas: Option<U256>,
    ) -> GetLoanThenSwapChainCall<M> {
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

        //call.calldata();

        //let tx_hash = call.send().await?.await?.unwrap();
        //println!("Transaction Receipt: {}", serde_json::to_string(&tx_hash)?); // Use simd_json

        call
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
        let tx_hash: TxHash = self
            .get_loan_then_swap_chain_call(
                swaps,
                chain_swaps,
                return_output,
                gas_price,
                max_fee_per_gas,
                max_priority_fee_per_gas,
            )
            .send()
            .await?
            .tx_hash();
        Ok(tx_hash)
    }

    pub async fn get_loan_then_swap_chain_bundle(
        &self,
        opp_tx: &Transaction,
        bid_amount: U256,
        swaps: Vec<OneSwapInfo>,
        chain_swaps: bool,
        return_output: bool,
        gas_price: Option<U256>,
        max_fee_per_gas: Option<U256>,
        max_priority_fee_per_gas: Option<U256>,
    ) -> Result<TxHash> {
        let my_tx_call = self.get_loan_then_swap_chain_call(
            swaps,
            chain_swaps,
            return_output,
            gas_price,
            max_fee_per_gas,
            max_priority_fee_per_gas,
        );

        let signed_bytes: Bytes = my_tx_call.calldata().unwrap();
        let signed_string: String = format!("0x{}", utils::hex::encode(&signed_bytes));
        //let signed_tx: Transaction = utils::rlp::decode(&signed_bytes).expect("Failed to decode signed transaction");

        let tx_hash: TxHash = self
            .fast_lane_contract
            .submit_flash_bid(
                bid_amount,
                opp_tx.hash().to_fixed_bytes(),
                self.contract.address(),
                signed_bytes,
            )
            .send()
            .await?
            .tx_hash();

        let target_signed_string: String = format!("0x{}", utils::hex::encode(opp_tx.rlp()));
        self.bundle_provider
            .send_flashbid_bundle(vec![target_signed_string, signed_string])
            .await?;

        Ok(tx_hash)
    }
}
