use super::new_block_stream::NewBlock;
use ethers::types::{Log, Transaction};

#[derive(Debug, Clone)]
pub enum NetworkEvent {
    Block(NewBlock),
    PendingTx(Transaction),
    Log(Log),
}
