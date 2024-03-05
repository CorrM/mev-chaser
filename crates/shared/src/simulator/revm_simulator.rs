use std::str::FromStr;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use ethers::prelude::U64;
use ethers::types::spoof;
use ethers::{
    abi::{self, AbiDecode, AbiEncode},
    providers::Middleware,
    types::spoof::State,
    types::{Address, H256, U256},
    utils::keccak256,
};
use hashbrown::HashMap;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use revm::{
    db::{CacheDB, EmptyDB, EthersDB, InMemoryDB},
    primitives::{AccountInfo, CfgEnv, ExecutionResult, Output, ResultAndState, TransactTo, TxEnv},
    ContextWithHandlerCfg, DatabaseRef, Evm,
};

use contracts::balancer_flash_loan_recipient::OneSwapInfo;
use contracts::erc20_token::{BalanceOfCall, BalanceOfReturn};
use contracts::simulator::SIMULATORABI_DEPLOYED_BYTECODE;
use vidger::types::CryptoToken;

use crate::amm::AmmPoolKind;

#[derive(Debug, Clone)]
pub struct TxResult {
    pub output: revm::primitives::Bytes,
    pub logs: Option<Vec<revm::primitives::Log>>,
    pub gas_used: u64,
    pub gas_refunded: u64,
}

pub struct RevmSimulator<M> {
    account: Address,
    state_override_set: State,
    provider: Arc<M>,
    ctx_with_handler: ContextWithHandlerCfg<(), InMemoryDB>,
}

impl<M> RevmSimulator<M>
where
    M: Middleware,
{
    pub(super) fn new(provider: Arc<M>, tokens_to_override_balance: &[CryptoToken]) -> Self {
        let ten_eth: U256 = U256::from(10).checked_mul(U256::from(10).pow(U256::from(18))).unwrap();

        let mut state_override_set: State = spoof::state();
        let account = Address::from_str("0x9cf277A22EB4c551c6E18F7a6C0ee1893bcB034f").unwrap();

        // Spoof user balance with 10 ETH (for gas fees)
        state_override_set.account(account).balance(ten_eth).nonce(0.into());

        // Create Simulator contract with bytecode injection
        let simulator_address = Address::from_str("0xF2d01Ee818509a9540d8324a5bA52329af27D19E").unwrap();
        state_override_set
            .account(simulator_address)
            .code(SIMULATORABI_DEPLOYED_BYTECODE.clone());

        // Spoof tokens balance for the user
        for crypto_token in tokens_to_override_balance {
            let input_balance_slot: [u8; 32] = keccak256(abi::encode(&[
                abi::Token::Address(simulator_address),
                abi::Token::Uint(U256::from(crypto_token.balance_contract_slot())),
            ]));

            state_override_set
                .account(account)
                .store(input_balance_slot.into(), H256::from_low_u64_be(ten_eth.as_u64()));
        }

        // https://github.com/bluealloy/revm/issues/1062
        let db: InMemoryDB = CacheDB::new(EmptyDB::default());
        let mut evm: Evm<'static, (), InMemoryDB> = Evm::builder().with_db(db).build();

        // overriding some default env values to make it more efficient for testing
        let evm_cfg: &mut CfgEnv = evm.cfg_mut();
        evm_cfg.limit_contract_code_size = None;
        evm_cfg.disable_block_gas_limit = true;
        evm_cfg.disable_base_fee = true;

        let ctx_with_handler: ContextWithHandlerCfg<(), InMemoryDB> = evm.into_context_with_handler_cfg();
        Self {
            account,
            state_override_set,
            provider,
            ctx_with_handler,
        }
    }

    pub fn provider(&self) -> &Arc<M> {
        &self.provider
    }
}
impl<M> RevmSimulator<M>
where
    M: Middleware + 'static,
{
    #[inline]
    fn get_evm(&self) -> Evm<(), InMemoryDB> {
        let cfg = ContextWithHandlerCfg::new(self.ctx_with_handler.context.clone(), self.ctx_with_handler.cfg);
        Evm::builder().with_context_with_handler_cfg(cfg).build()
    }

    #[inline]
    fn clone_evm(&self, context_with_handler_cfg: ContextWithHandlerCfg<(), InMemoryDB>) -> Evm<(), InMemoryDB> {
        Evm::builder()
            .with_context_with_handler_cfg(context_with_handler_cfg)
            .build()
    }

    pub fn update_block(&mut self) {
        // Update all AccountInfo in the db
    }

    pub fn get_token_balance(&self, token: Address) -> Result<U256> {
        let calldata: Vec<u8> = AbiEncode::encode(BalanceOfCall { who: self.account });

        let mut evm: Evm<(), InMemoryDB> = self.get_evm();
        let tx: &mut TxEnv = evm.tx_mut();
        tx.caller = self.account.0.into();
        tx.transact_to = TransactTo::Call(token.0.into());
        tx.data = calldata.into();

        // This will fail, because the token contract has not been deployed yet
        let result_and_state: ResultAndState = match evm.transact_preverified() {
            Ok(result) => result,
            Err(e) => return Err(anyhow!("EVM call failed: {e:?}")),
        };
        let tx_result = match result_and_state.result {
            ExecutionResult::Success {
                gas_used,
                gas_refunded,
                output,
                logs,
                ..
            } => match output {
                Output::Call(o) => TxResult {
                    output: o,
                    logs: Some(logs),
                    gas_used,
                    gas_refunded,
                },
                Output::Create(o, _) => TxResult {
                    output: o,
                    logs: Some(logs),
                    gas_used,
                    gas_refunded,
                },
            },
            ExecutionResult::Revert { gas_used, output } => {
                return Err(anyhow!("EVM REVERT: {:?} / Gas used: {:?}", output, gas_used));
            }
            ExecutionResult::Halt { reason, gas_used } => {
                return Err(anyhow!("EVM HALT: {:?} / Gas used: {:?}", reason, gas_used));
            }
        };

        let Ok(decoded_output) = BalanceOfReturn::decode(&tx_result.output) else {
            return Err(anyhow!("Failed to decode output"));
        };

        Ok(decoded_output.0)
    }

    pub fn get_tokens_balance_slot(
        &self,
        tokens: &[Address],
        block_number: U64,
    ) -> Result<HashMap<Address, Result<Option<i32>>>> {
        let mut evm: Evm<(), InMemoryDB> = self.get_evm();

        // Get token account info from ethers middleware and insert it into EVM
        let ethers_db: EthersDB<M> = EthersDB::new(Arc::clone(&self.provider), Some(block_number.into())).unwrap();

        let tokens_accounts: Vec<(AccountInfo, revm::primitives::Address)> = tokens
            .par_iter()
            .map(|token| {
                let token: revm::primitives::Address = token.0.into();
                let token_acc_info: AccountInfo = ethers_db.basic_ref(token).unwrap().unwrap();

                (token_acc_info, token)
            })
            .collect();

        for (token_acc_info, token) in tokens_accounts {
            evm.context.evm.db.insert_account_info(token, token_acc_info);
        }

        // Call balanceOf
        let calldata: revm::primitives::Bytes = AbiEncode::encode(BalanceOfCall { who: self.account }).into();

        let handler_cfg: &ContextWithHandlerCfg<(), InMemoryDB> = &evm.into_context_with_handler_cfg();
        let ret: HashMap<Address, Result<Option<i32>>> = tokens
            .par_iter()
            .map(|token| {
                let cfg = ContextWithHandlerCfg::new(handler_cfg.context.clone(), handler_cfg.cfg);
                let mut evm: Evm<(), InMemoryDB> = self.clone_evm(cfg);

                let _token: revm::primitives::Address = token.0.into();

                let tx: &mut TxEnv = evm.tx_mut();
                tx.caller = self.account.0.into();
                tx.transact_to = TransactTo::Call(_token);
                tx.data = calldata.clone();

                let result_and_state: ResultAndState = match evm.transact_preverified() {
                    Ok(result) => result,
                    Err(e) => {
                        return (*token, Err(anyhow!("EVM call failed: {e:?}")));
                    }
                };

                // Get touched storage
                let token_acc: &revm::primitives::Account = result_and_state.state.get(&_token).unwrap();
                let touched_storage: &revm::primitives::Storage = &token_acc.storage;
                println!("Touched storage slots: {:?}", touched_storage);

                for i in 0..20 {
                    let slot: [u8; 32] = keccak256(&abi::encode(&[
                        abi::Token::Address(self.account),
                        abi::Token::Uint(U256::from(i)),
                    ]));

                    let slot: revm::primitives::U256 = revm::primitives::U256::from_be_bytes(slot);
                    if touched_storage.get(&slot).is_none() {
                        continue;
                    };

                    println!("Balance storage slot: {:?} ({:?})", i, slot);
                    return (*token, Ok(Some(i)));
                }

                (*token, Ok(None))
            })
            .collect();

        Ok(ret)
    }

    pub fn get_amounts_out(&self, pool: &AmmPoolKind, amount_in: U256) -> Result<U256> {
        todo!()
    }

    pub fn multi_swaps(&self, block_number: U64, swaps: Vec<OneSwapInfo>, chain_swaps: bool) -> Result<U256> {
        Ok(0.into())
    }
}
