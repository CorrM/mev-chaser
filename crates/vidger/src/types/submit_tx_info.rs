use ethers::types::transaction::eip2718::TypedTransaction;

use crate::executors::GasBidInfo;

#[derive(Debug, Clone)]
pub struct SubmitTxInfo {
    pub tx: TypedTransaction,
    pub gas_bid_info: Option<GasBidInfo>,
}
