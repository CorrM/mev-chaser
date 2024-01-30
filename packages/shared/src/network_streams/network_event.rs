use ethers::types::{Log, Transaction};

use super::new_block_stream::NewBlock;

#[derive(Debug, Clone)]
pub enum NetworkEvent {
    Block(NewBlock),
    PendingTx(Transaction),
    Log(Log),
}
