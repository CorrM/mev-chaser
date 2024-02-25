use std::sync::Arc;

use anyhow::{anyhow, Result};
use ethers::{
    abi::{self, AbiDecode, AbiEncode},
    types::{Address, Block, BlockNumber, H256, U256},
    utils::keccak256,
};
use ethers_core::types::{BigEndianHash, spoof, TxHash, U64};
use ethers_providers::Middleware;
use revm::primitives::Storage;
use revm::{
    db::{CacheDB, EmptyDB, EthersDB, InMemoryDB},
    primitives::{AccountInfo, CfgEnv, ExecutionResult, Output, ResultAndState, TransactTo, TxEnv},
    ContextWithHandlerCfg, Database, Evm,
};
use tokio::task::JoinSet;

use contracts::erc20_token::{BalanceOfCall, BalanceOfReturn};

use crate::tx_result::TxResult;

pub struct EvmSimulator {
    ctx_with_handler: ContextWithHandlerCfg<(), InMemoryDB>,
}

impl EvmSimulator {
    pub fn new() -> Self {
        // https://github.com/bluealloy/revm/issues/1062
        let db: InMemoryDB = CacheDB::new(EmptyDB::default());
        let mut evm: Evm<'static, (), InMemoryDB> = Evm::builder().with_db(db).build();

        // overriding some default env values to make it more efficient for testing
        let evm_cfg: &mut CfgEnv = evm.cfg_mut();
        evm_cfg.limit_contract_code_size = None;
        evm_cfg.disable_block_gas_limit = true;
        evm_cfg.disable_base_fee = true;

        let ctx_with_handler: ContextWithHandlerCfg<(), InMemoryDB> = evm.into_context_with_handler_cfg();
        Self { ctx_with_handler }
    }

    fn get_evm(&self) -> Evm<'static, (), InMemoryDB> {
        let cfg = ContextWithHandlerCfg::new(self.ctx_with_handler.context.clone(), self.ctx_with_handler.cfg.clone());
        Evm::builder().with_context_with_handler_cfg(cfg).build()
    }

    pub fn get_token_balance(&self, token: Address, account: Address) -> Result<U256> {
        let calldata: Vec<u8> = BalanceOfCall { who: account }.encode();

        let mut evm: Evm<(), InMemoryDB> = self.get_evm();
        let tx: &mut TxEnv = evm.tx_mut();
        tx.caller = account.0.into();
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

    pub async fn revm_contract_deploy_and_tracing<M: Middleware + 'static>(
        &self,
        provider: Arc<M>,
        token: Address,
        account: Address,
    ) -> Result<i32> {
        let token: revm::primitives::Address = token.0.into();
        let mut evm: Evm<(), InMemoryDB> = self.get_evm();

        // Deploy contract to EVM then insert account using ethers middleware
        let block: Block<H256> = provider
            .get_block(BlockNumber::Latest)
            .await?
            .ok_or(anyhow!("failed to retrieve block"))?;

        // TEEEEES: Use eth_call this with `debug.traceCall` prestateTracer to make the state, then simulate swaps, EZ support for all AMMs
        let mut state = spoof::state();
        state.account(account).store(
            input_balance_slot.into(),
            H256::from_low_u64_be(ten_eth.as_u64()),
        );

        let mut ethersdb: EthersDB<M> =
            EthersDB::new(provider, Some(block.number.unwrap().into())).unwrap();
        let token_acc_info: AccountInfo = ethersdb.basic(token).unwrap().unwrap();
        evm.context.evm.db.insert_account_info(token, token_acc_info);

        // Call balanceOf
        let calldata: Vec<u8> = BalanceOfCall { who: account }.encode();

        let tx: &mut TxEnv = evm.tx_mut();
        tx.caller = account.0.into();
        tx.transact_to = TransactTo::Call(token);
        tx.data = calldata.into();

        let result_and_state: ResultAndState = match evm.transact_preverified() {
            Ok(result) => result,
            Err(e) => return Err(anyhow!("EVM call failed: {e:?}")),
        };

        // Get touched storage
        let token_acc: &revm::primitives::Account = result_and_state.state.get(&token).unwrap();
        let touched_storage: &Storage = &token_acc.storage;
        println!("Touched storage slots: {:?}", touched_storage);

        for i in 0..20 {
            let slot: [u8; 32] = keccak256(&abi::encode(&[
                abi::Token::Address(account),
                abi::Token::Uint(U256::from(i)),
            ]));

            let slot: revm::primitives::U256 = revm::primitives::U256::from_be_bytes(slot);
            if touched_storage.get(&slot).is_none() {
                continue;
            };

            println!("Balance storage slot: {:?} ({:?})", i, slot);
            return Ok(i);
        }

        Ok(0)
    }

    pub async fn get_proxy_implementation<M: Middleware + 'static>(
        &self,
        provider: Arc<M>,
        token: Address,
        block_number: U64,
    ) -> Result<Option<Address>> {
        // adapted from: https://github.com/gnosis/evm-proxy-detection/blob/main/src/index.ts
        let eip_1967_logic_slot: U256 =
            U256::from("0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc");
        let eip_1967_beacon_slot: U256 =
            U256::from("0xa3f0ad74e5423aebfd80d3ef4346578335a9a72aeaee59ff6cb3582b35133d50");
        let open_zeppelin_implementation_slot: U256 =
            U256::from("0x7050c9e0f4ca769c69bd3a8ef740bc37934f8e2c036e5a723fd8ee048ed3f8c3");
        let eip_1822_logic_slot: U256 =
            U256::from("0xc5f16f0fcc639fa48a6947836d9850f504798523bf8c9a3a87d5876cf622bcf7");

        let implementation_slots: Vec<U256> = vec![
            eip_1967_logic_slot,
            eip_1967_beacon_slot,
            open_zeppelin_implementation_slot,
            eip_1822_logic_slot,
        ];

        let mut set = JoinSet::new();
        for slot in implementation_slots {
            let _provider = Arc::clone(&provider);
            let fut = tokio::spawn(async move {
                _provider
                    .get_storage_at(token, TxHash::from_uint(&slot), Some(block_number.into()))
                    .await
            });
            set.spawn(fut);
        }

        while let Some(res) = set.join_next().await {
            let out: TxHash = res???;
            let implementation = Address::from(out);
            if implementation != Address::zero() {
                return Ok(Some(implementation));
            }
        }

        Ok(None)
    }
}

impl Default for EvmSimulator {
    fn default() -> Self {
        Self::new()
    }
}
