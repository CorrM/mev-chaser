use std::{str::FromStr, sync::Arc};

use anyhow::Result;
use ethers::{
    middleware::SignerMiddleware,
    signers::{LocalWallet, Signer, Wallet},
    types::{transaction::eip2718::TypedTransaction, Bytes, Signature, Transaction, I256},
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
    signer: Arc<SignerMiddleware<Arc<M>, Wallet<SigningKey>>>,
    chain_id: u64,
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

        let mut wallet_private_key: String = wallet_private_key;
        if wallet_private_key.starts_with("0x") {
            wallet_private_key = wallet_private_key.split_off(2);
        }

        let wallet: LocalWallet = wallet_private_key
            .parse::<LocalWallet>()?
            .with_chain_id(chain_id.as_u64());

        let signer = Arc::new(SignerMiddleware::new(provider, wallet));
        let contract = BalancerFlashLoanRecipientAbi::new(address, Arc::clone(&signer));

        let fast_lane_address: Address = Address::from_str("0xCACe8D78269ba00f1C4D5Fc3B1228C7DF0a7C8BA")?;
        let fast_lane_contract = FastLaneAuctionHandlerAbi::new(fast_lane_address, Arc::clone(&signer));

        Ok(Self {
            signer,
            chain_id: chain_id.as_u64(),
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
        let my_contract_call = self.get_loan_then_swap_chain_call(
            swaps,
            chain_swaps,
            return_output,
            gas_price,
            max_fee_per_gas,
            max_priority_fee_per_gas,
        );
        let function_call_data: Bytes = my_contract_call.calldata().unwrap();

        let mut submit_flash_bid_call = self.fast_lane_contract.submit_flash_bid(
            bid_amount,
            opp_tx.hash().to_fixed_bytes(),
            self.contract.address(),
            function_call_data,
        );

        if gas_price.is_some() {
            submit_flash_bid_call = submit_flash_bid_call.legacy().gas_price(gas_price.unwrap());
        } else {
            let tx = submit_flash_bid_call.tx.as_eip1559_mut().unwrap();
            tx.max_fee_per_gas = max_fee_per_gas;
            tx.max_priority_fee_per_gas = max_priority_fee_per_gas;
        }

        let mut submit_flash_bid_tx: TypedTransaction = submit_flash_bid_call.tx;
        submit_flash_bid_tx.set_chain_id(self.chain_id);

        let sig: Signature = self.signer.sign_transaction(&submit_flash_bid_tx, self.signer.address()).await?;
        let signed_bytes: Bytes = submit_flash_bid_tx.rlp_signed(&sig);
        let signed_string: String = format!("0x{}", utils::hex::encode(&signed_bytes));
        //let signed_tx: Transaction = utils::rlp::decode(&signed_bytes).expect("Failed to decode signed transaction");

        let target_signed_string: String = format!("0x{}", utils::hex::encode(opp_tx.rlp()));
        let relay_response: String = self
            .bundle_provider
            .send_flashbid_bundle(vec![target_signed_string, signed_string])
            .await?;

        println!("relay_response: {}", relay_response);

        Ok(submit_flash_bid_tx.hash(&sig))
    }
}
