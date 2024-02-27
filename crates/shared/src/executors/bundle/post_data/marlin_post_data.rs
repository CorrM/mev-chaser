use ethers::types::U64;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
struct MarlinParams {
    txs: Vec<String>, //bundle
    blockNumber: U64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MarlinPostData {
    jsonrpc: String,
    id: u32,
    method: String,
    params: Vec<MarlinParams>,
}

impl MarlinPostData {
    pub(super) fn new(bundle: Vec<String>, id: u32, block_number: U64) -> Self {
        let marlin_params = MarlinParams {
            txs: bundle,
            blockNumber: block_number,
        };

        Self {
            jsonrpc: "2.0".to_string(),
            id,
            method: "eth_sendBundle".to_string(),
            params: vec![marlin_params],
        }
    }
}
