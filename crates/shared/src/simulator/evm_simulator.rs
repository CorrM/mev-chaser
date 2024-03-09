use std::str::FromStr;
use std::sync::Arc;

use anyhow::Result;
use ethers::prelude::H256;
use ethers::providers::Middleware;
use ethers::types::{Address, U256, U64};
use ethers::types::{BigEndianHash, TxHash};
use hashbrown::HashMap;
use tokio::task::JoinError;

use crate::amm::AmmPoolKind;
use crate::simulator::RevmSimulator;
use crate::types::CryptoToken;

pub enum EvmSimulator<M> {
    Revm(RevmSimulator<M>),
    //Ethers(EthersSimulator<M>),
}

impl<M> EvmSimulator<M>
where
    M: Middleware + 'static,
{
    #[inline]
    pub fn new_revm(provider: Arc<M>, tokens_to_override_balance: &[CryptoToken]) -> Result<Self> {
        Ok(Self::Revm(RevmSimulator::new(provider, tokens_to_override_balance)?))
    }

    #[inline]
    pub fn is_revm(&self) -> bool {
        matches!(self, Self::Revm(_))
    }

    #[inline]
    pub fn as_revm(&self) -> Option<&RevmSimulator<M>> {
        match self {
            Self::Revm(revm) => Some(revm),
            _ => None,
        }
    }
}

impl<M> EvmSimulator<M> {
    #[inline]
    pub fn provider(&self) -> &Arc<M> {
        match self {
            Self::Revm(revm) => revm.provider(),
        }
    }
}

impl<M> EvmSimulator<M>
where
    M: Middleware + 'static,
{
    pub fn update_block(&mut self) {
        match self {
            Self::Revm(ref mut revm) => revm.on_new_block(),
        }
    }

    pub fn get_proxy_implementation(&self, token: Address, block_number: U64) -> Result<Option<Address>> {
        // adapted from: https://github.com/gnosis/evm-proxy-detection/blob/main/src/index.ts
        let eip_1967_logic_slot: U256 =
            U256::from_str("0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc")?;
        let eip_1967_beacon_slot: U256 =
            U256::from_str("0xa3f0ad74e5423aebfd80d3ef4346578335a9a72aeaee59ff6cb3582b35133d50")?;
        let open_zeppelin_implementation_slot: U256 =
            U256::from_str("0x7050c9e0f4ca769c69bd3a8ef740bc37934f8e2c036e5a723fd8ee048ed3f8c3")?;
        let eip_1822_logic_slot: U256 =
            U256::from_str("0xc5f16f0fcc639fa48a6947836d9850f504798523bf8c9a3a87d5876cf622bcf7")?;

        let implementation_slots: Vec<U256> = vec![
            eip_1967_logic_slot,
            eip_1967_beacon_slot,
            open_zeppelin_implementation_slot,
            eip_1822_logic_slot,
        ];

        let slots: Vec<Result<Result<H256, <M as Middleware>::Error>, JoinError>> =
            async_scoped::TokioScope::scope_and_block(|s| {
                for slot in &implementation_slots {
                    s.spawn(async {
                        self.provider()
                            .get_storage_at(token, TxHash::from_uint(slot), Some(block_number.into()))
                            .await
                    });
                }
            })
            .1;

        for slot in slots {
            let out: TxHash = slot??;
            let implementation = Address::from(out);
            if implementation != Address::zero() {
                return Ok(Some(implementation));
            }
        }

        Ok(None)
    }

    pub fn get_tokens_balance_slot(&self, tokens: &[Address]) -> Result<HashMap<Address, Result<Option<i32>>>> {
        match self {
            Self::Revm(ref revm) => revm.get_tokens_balance_slot(&tokens),
        }
    }

    pub fn get_amounts_out(&self, pool: &AmmPoolKind, amount_in: U256) -> Result<U256> {
        match self {
            Self::Revm(ref revm) => revm.get_amounts_out(pool, amount_in),
        }
    }
}
