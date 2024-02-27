use ethers::types::Transaction;

use vidger::{types::NewBlock, types::SubmitTxInfo};

/// Convenience enum containing all the events that can be emitted by collectors.
#[derive(Clone, Debug)]
pub enum MevEvents {
    NewBlock(NewBlock),
    NewTransaction(Transaction),
}

/// Convenience enum containing all the actions that can be executed by executors.
#[derive(Clone, Debug)]
pub enum MevActions {
    SubmitTxToMempool(SubmitTxInfo),
    SubmitTxToFastLine(SubmitTxInfo),
}
