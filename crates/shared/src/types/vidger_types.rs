use ethers::types::Transaction;

use vidger::executors::mempool_executor::SubmitTxToMempool;
use vidger::types::NewBlock;

/// Convenience enum containing all the events that can be emitted by collectors.
#[derive(Clone, Debug)]
pub enum Events {
    NewBlock(NewBlock),
    NewTransaction(Transaction),
}

/// Convenience enum containing all the actions that can be executed by executors.
#[derive(Clone, Debug)]
pub enum Actions {
    SubmitTxToMempool(SubmitTxToMempool),
}
