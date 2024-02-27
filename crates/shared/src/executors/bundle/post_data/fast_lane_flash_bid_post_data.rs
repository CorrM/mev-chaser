use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FastLaneFlashBidPostData {
    #[serde(rename = "jsonrpc")]
    json_rpc: String,
    id: u32,
    method: String,
    params: Vec<Vec<String>>,
}

impl FastLaneFlashBidPostData {
    pub(super) fn new(bundle: Vec<String>, id: u32) -> Self {
        Self {
            json_rpc: "2.0".to_string(),
            id,
            method: "pfl_addSearcherBundle".to_string(),
            params: vec![bundle],
        }
    }
}
