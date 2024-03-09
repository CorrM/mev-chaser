use std::str::FromStr;
use std::sync::{Arc, RwLock};

use anyhow::{anyhow, Result};
use ethers::abi::{AbiDecode, AbiEncode};
use ethers::{
    abi,
    providers::Middleware,
    types::{Address, Block, BlockNumber, H256, U256},
    utils::keccak256,
};
use hashbrown::HashMap;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use revm::{
    db::{EmptyDB, EthersDB, InMemoryDB},
    primitives::{
        AccountInfo, Bytecode, CfgEnv, ExecutionResult, Output, ResultAndState, TransactTo, TxEnv, KECCAK_EMPTY,
    },
    ContextWithHandlerCfg, Database, DatabaseRef, Evm,
};

use contracts::balancer_flash_loan_recipient::OneSwapInfo;
use contracts::erc20_token::{BalanceOfCall, BalanceOfReturn};
use contracts::simulator::SIMULATORABI_DEPLOYED_BYTECODE;
use vidger::utilities::block_on;

use crate::amm::AmmPoolKind;
use crate::types::CryptoToken;

/*
use ethers::{
    abi::{self, AbiDecode, AbiEncode},
    providers::Middleware,
    types::{Address, H256, U256, U64},
    utils::keccak256,
};
*/
#[derive(Debug, Clone)]
pub struct TxResult {
    pub output: revm::primitives::Bytes,
    pub logs: Option<Vec<revm::primitives::Log>>,
    pub gas_used: u64,
    pub gas_refunded: u64,
}

pub struct RevmSimulator<M> {
    provider: Arc<M>,
    account: revm::primitives::Address,
    ctx_with_handler: ContextWithHandlerCfg<(), InMemoryDB>,
}

impl<M> RevmSimulator<M>
where
    M: Middleware + 'static,
{
    fn deploy_token_and_spoof_balance(
        db: Arc<RwLock<InMemoryDB>>,
        ethers_db: Arc<RwLock<EthersDB<M>>>,
        token: &CryptoToken,
    ) {
        let hundred_grand_eth: revm::primitives::U256 = revm::primitives::U256::from(100_000)
            .checked_mul(revm::primitives::U256::from(10).pow(revm::primitives::U256::from(18)))
            .unwrap();

        // Deploy token
        let token_address: revm::primitives::Address = token.proxy_or_address().0.into();
        let token_acc_info: AccountInfo = ethers_db.read().unwrap().basic_ref(token_address).unwrap().unwrap();

        // Spoof balance
        let input_balance_slot: [u8; 32] = keccak256(abi::encode(&[
            abi::Token::Address(ethers::types::Address::from(token_address.0 .0)),
            abi::Token::Uint(ethers::types::U256::from(token.balance_contract_slot())),
        ]));

        // Commit changes
        let mut write_guard = db.write().unwrap();
        write_guard.insert_account_info(token_address, token_acc_info);
        write_guard
            .insert_account_storage(
                token_address,
                revm::primitives::U256::from_be_bytes(input_balance_slot),
                hundred_grand_eth,
            )
            .expect("failed to insert token balance in DB");
    }

    pub(super) fn new(provider: Arc<M>, tokens: &[CryptoToken]) -> Result<Self> {
        // https://github.com/bluealloy/revm/issues/1062
        let hundred_grand_eth: revm::primitives::U256 = revm::primitives::U256::from(100_000)
            .checked_mul(revm::primitives::U256::from(10).pow(revm::primitives::U256::from(18)))
            .unwrap();
        let account = revm::primitives::Address::from_str("0x9cf277A22EB4c551c6E18F7a6C0ee1893bcB034f").unwrap();
        let cur_block: Block<H256> =
            block_on(provider.get_block(BlockNumber::Latest))?.ok_or(anyhow!("failed to retrieve block"))?;

        // Prepare in-memory DB
        let mut db = InMemoryDB::new(EmptyDB::default());

        // Give the user enough ETH to pay for gas
        let user_acc_info = AccountInfo::new(hundred_grand_eth, 0, KECCAK_EMPTY, Bytecode::default());
        db.insert_account_info(account, user_acc_info); // TODO: Remove .into()

        // Deploy Simulator contract
        let simulator_address =
            revm::primitives::Address::from_str("0xF2d01Ee818509a9540d8324a5bA52329af27D19E").unwrap();
        let simulator_bytes = Bytecode::new_raw((*SIMULATORABI_DEPLOYED_BYTECODE.0).into());
        let simulator_acc_info = AccountInfo::new(
            revm::primitives::U256::ZERO,
            0,
            simulator_bytes.hash_slow(),
            simulator_bytes,
        );
        db.insert_account_info(simulator_address, simulator_acc_info);

        // Spoof tokens balance for the user
        let db: Arc<RwLock<InMemoryDB>> = Arc::new(RwLock::new(db));
        let ethers_db: Arc<RwLock<EthersDB<M>>> = Arc::new(RwLock::new(
            EthersDB::new(provider.clone(), Some(cur_block.number.unwrap().into())).unwrap(),
        ));
        tokens.par_iter().for_each(|token: &CryptoToken| {
            Self::deploy_token_and_spoof_balance(Arc::clone(&db), Arc::clone(&ethers_db), token)
        });

        let db: InMemoryDB = Arc::try_unwrap(db).unwrap().into_inner().unwrap();

        // Create EVM
        let mut evm: Evm<'static, (), InMemoryDB> = Evm::builder().with_db(db).build();

        // overriding some default env values to make it more efficient for testing
        let evm_cfg: &mut CfgEnv = evm.cfg_mut();
        evm_cfg.limit_contract_code_size = None;
        evm_cfg.disable_block_gas_limit = true;
        evm_cfg.disable_base_fee = true;

        // Create context
        let ctx_with_handler: ContextWithHandlerCfg<(), InMemoryDB> = evm.into_context_with_handler_cfg();

        Ok(Self {
            provider,
            account,
            ctx_with_handler,
        })
    }
}

impl<M> RevmSimulator<M> {
    #[inline]
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
        Evm::builder()
            .with_context_with_handler_cfg(self.ctx_with_handler.clone())
            .build()
    }

    #[inline]
    fn clone_evm(&self, context_with_handler_cfg: ContextWithHandlerCfg<(), InMemoryDB>) -> Evm<(), InMemoryDB> {
        Evm::builder()
            .with_context_with_handler_cfg(context_with_handler_cfg)
            .build()
    }

    #[inline]
    fn get_tx_result(result: ExecutionResult) -> Result<TxResult> {
        let output: TxResult = match result {
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
                return Err(anyhow!("EVM REVERT: {:?} / Gas used: {:?}", output, gas_used))
            }
            ExecutionResult::Halt { reason, .. } => return Err(anyhow!("EVM HALT: {:?}", reason)),
        };

        Ok(output)
    }

    pub fn on_new_block(&mut self) {
        // Update all AccountInfo in the db
    }

    pub fn get_token_balance(&self, token: Address) -> Result<U256> {
        let calldata: Vec<u8> = abi::AbiEncode::encode(BalanceOfCall {
            who: self.account.0 .0.into(),
        });

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

    pub fn get_tokens_balance_slot(&self, tokens: &[Address]) -> Result<HashMap<Address, Result<Option<i32>>>> {
        let mut evm: Evm<(), InMemoryDB> = self.get_evm();

        // Get token account info from ethers middleware and insert it into EVM
        let cur_block: Block<H256> =
            block_on(self.provider.get_block(BlockNumber::Latest))?.ok_or(anyhow!("failed to retrieve block"))?;
        let ethers_db: EthersDB<M> =
            EthersDB::new(Arc::clone(&self.provider), Some(cur_block.number.unwrap().into())).unwrap();

        let tokens_accounts: Vec<(AccountInfo, revm::primitives::Address)> = tokens
            .par_iter()
            .map(|token: &Address| {
                let token: revm::primitives::Address = token.0.into();
                let token_acc_info: AccountInfo = ethers_db.basic_ref(token).unwrap().unwrap();

                (token_acc_info, token)
            })
            .collect();

        for (token_acc_info, token) in tokens_accounts {
            evm.db_mut().insert_account_info(token, token_acc_info);
        }

        // Call balanceOf
        let calldata: revm::primitives::Bytes = AbiEncode::encode(BalanceOfCall {
            who: self.account.0 .0.into(),
        })
        .into();

        let handler_cfg: &ContextWithHandlerCfg<(), InMemoryDB> = &evm.into_context_with_handler_cfg();
        let ret: HashMap<Address, Result<Option<i32>>> = tokens
            .par_iter()
            .map(|token: &Address| {
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
                        abi::Token::Address(self.account.0 .0.into()),
                        abi::Token::Uint(U256::from(i).into()),
                    ]));

                    let slot = revm::primitives::U256::from_be_bytes(slot);
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

    pub fn multi_swaps(&self, swaps: Vec<OneSwapInfo>, chain_swaps: bool) -> Result<U256> {
        todo!()
    }
}
