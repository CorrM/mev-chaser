use std::sync::Arc;

use anyhow::Result;
use ethers::providers::Middleware;
use ethers::types::{Address, Log, U256};
use hashbrown::HashMap;

use vidger::types::NewBlock;

use crate::amm::AmmPoolKind;
use crate::managers::AmmManager;
use crate::simulator::RevmSimulator;

pub enum EvmSimulator<M> {
    Revm(RevmSimulator<M>),
    //Ethers(EthersSimulator<M>),
}

impl<M> EvmSimulator<M>
where
    M: Middleware + 'static,
{
    #[inline]
    pub fn new_revm(provider: Arc<M>, amm_manager: &AmmManager) -> Result<Self> {
        Ok(Self::Revm(RevmSimulator::new(provider, amm_manager)?))
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
    #[inline]
    pub fn on_new_block(&mut self, new_block: &NewBlock, logs: &[Log]) {
        match self {
            Self::Revm(ref mut revm) => revm.on_new_block(new_block, logs),
        }
    }

    #[inline]
    pub fn get_tokens_balance_slot(&self, tokens: &[Address]) -> Result<HashMap<Address, Result<Option<i32>>>> {
        match self {
            Self::Revm(ref revm) => revm.get_tokens_balance_slot(tokens),
        }
    }

    #[inline]
    pub fn get_amounts_out(&self, pool: &AmmPoolKind, amount_in: U256) -> Result<U256> {
        match self {
            Self::Revm(ref revm) => revm.get_amounts_out(pool, amount_in),
        }
    }
}
