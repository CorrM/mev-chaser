use serde::{Deserialize, Serialize};

//pfl_addSearcherFastBid

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FastLaneFastBidPostData {
    jsonrpc: String,
    id: u32,
    method: String,
    params: Vec<String>,
}
impl FastLaneFastBidPostData {
    pub(super) fn new(tx: Vec<String>, id: u32) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            method: "pfl_addSearcherFastBid".to_string(),
            params: tx,
        }
    }
}