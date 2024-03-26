use anyhow::anyhow;
use ethers::types::{Block, H256, U256, U64};

use vidger::types::NewBlock;

/// Calculate the next block base fee
// based on math provided here: https://ethereum.stackexchange.com/questions/107173/how-is-the-base-fee-per-gas-computed-for-a-new-block
fn calculate_next_block_base_fee(block: &BlockInfo) -> U256 {
    // Get the block base fee per gas
    let current_base_fee_per_gas = block.base_fee_per_gas;

    let current_gas_used: U256 = block
        .gas_used
        .expect("can't calculate base fee from un-mined block \"next_block\"");

    let current_gas_target: U256 = block
        .gas_limit
        .expect("can't calculate base fee from un-mined block \"next_block\"")
        / 2;

    let calculate_next_block_base_fee: U256 = if current_gas_used == current_gas_target {
        current_base_fee_per_gas
    } else if current_gas_used > current_gas_target {
        let gas_used_delta: U256 = current_gas_used - current_gas_target;
        let base_fee_per_gas_delta: U256 = current_base_fee_per_gas * gas_used_delta / current_gas_target / 8;

        current_base_fee_per_gas + base_fee_per_gas_delta
    } else {
        let gas_used_delta: U256 = current_gas_target - current_gas_used;
        let base_fee_per_gas_delta: U256 = current_base_fee_per_gas * gas_used_delta / current_gas_target / 8;

        current_base_fee_per_gas - base_fee_per_gas_delta
    };

    calculate_next_block_base_fee
}

/// Hold block information
#[derive(Default, Clone, Copy)]
pub struct BlockInfo {
    pub number: U64,
    pub base_fee_per_gas: U256,
    pub timestamp: U256,
    // These are optional because we don't know these values for `next_block`
    pub gas_used: Option<U256>,
    pub gas_limit: Option<U256>,
}

impl BlockInfo {
    /// Returns block info for next block
    pub fn get_next_block(&self) -> BlockInfo {
        BlockInfo {
            number: self.number + 1,
            base_fee_per_gas: calculate_next_block_base_fee(self),
            timestamp: self.timestamp + 2, // TODO: Polygon block is 2 seconds, Ethereum block is 15 seconds
            gas_used: None,
            gas_limit: None,
        }
    }
}

impl TryFrom<Block<H256>> for BlockInfo {
    type Error = anyhow::Error;

    fn try_from(value: Block<H256>) -> Result<Self, Self::Error> {
        Ok(BlockInfo {
            number: value
                .number
                .ok_or(anyhow!("could not parse block.number when setting up `block_manager`"))?,
            gas_used: Some(value.gas_used),
            gas_limit: Some(value.gas_limit),
            base_fee_per_gas: value
                .base_fee_per_gas
                .ok_or(anyhow!("could not parse base fee when setting up `block_manager`"))?,
            timestamp: value.timestamp,
        })
    }
}

impl From<NewBlock> for BlockInfo {
    fn from(value: NewBlock) -> Self {
        Self {
            number: value.number,
            base_fee_per_gas: value.base_fee_per_gas,
            timestamp: value.timestamp,
            gas_used: Some(value.gas_used),
            gas_limit: Some(value.gas_limit),
        }
    }
}
