use anyhow::{anyhow, Result};
use contracts::erc20_token::{BalanceOfCall, BalanceOfReturn};
use ethers::{
    abi::{AbiDecode, AbiEncode},
    types::{Address, U256},
};
use revm::{
    db::{CacheDB, EmptyDB, InMemoryDB},
    primitives::{CfgEnv, ExecutionResult, Output, TransactTo},
    Evm,
};

use crate::TxResult;

pub struct EvmSimulator<'a> {
    evm: Evm<'a, (), InMemoryDB>,
}

impl<'a> Default for EvmSimulator<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> EvmSimulator<'a> {
    pub fn new() -> Self {
        let db: InMemoryDB = CacheDB::new(EmptyDB::default());
        let mut evm: Evm<'a, (), InMemoryDB> = Evm::builder().with_db(db).build();

        // overriding some default env values to make it more efficient for testing
        let evm_cfg: &mut CfgEnv = evm.cfg_mut();
        evm_cfg.limit_contract_code_size = Some(0x100000);
        evm_cfg.disable_block_gas_limit = true;
        evm_cfg.disable_base_fee = true;

        Self { evm }
    }

    pub fn get_token_balance(&mut self, token: Address, account: Address) -> Result<U256> {
        let calldata: Vec<u8> = BalanceOfCall { who: account }.encode();

        self.evm.context.evm.env.tx.caller = account.0.into();
        self.evm.context.evm.env.tx.transact_to = TransactTo::Call(token.0.into());
        self.evm.context.evm.env.tx.data = calldata.into();

        // This will fail, because the token contract has not been deployed yet
        let result_and_state = match self.evm.transact_preverified() {
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
                return Err(anyhow!("EVM REVERT: {:?} / Gas used: {:?}", output, gas_used))
            }
            ExecutionResult::Halt { reason, gas_used } => {
                return Err(anyhow!("EVM HALT: {:?} / Gas used: {:?}", reason, gas_used))
            }
        };

        let Ok(decoded_output) = BalanceOfReturn::decode(&tx_result.output) else {
            return Err(anyhow!("Failed to decode output"));
        };

        Ok(decoded_output.0)
    }
}
