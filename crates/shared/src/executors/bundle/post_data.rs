use ethers::types::U64;
pub use fast_lane_fast_bid_post_data::*;
pub use fast_lane_flash_bid_post_data::*;
pub use marlin_post_data::*;
use serde::{Deserialize, Serialize};

use crate::relay_type::RelayType;

pub mod fast_lane_fast_bid_post_data;
pub mod fast_lane_flash_bid_post_data;
pub mod marlin_post_data;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PostData {
    FastLaneFlashBid(FastLaneFlashBidPostData),
    FastLaneFastBid(FastLaneFastBidPostData),
    Marlin(MarlinPostData),
}

impl PostData {
    pub fn new(param: Vec<String>, id: u32, relay_type: RelayType, block_number: U64) -> Self {
        match relay_type {
            RelayType::FastLaneFlashBid => PostData::FastLaneFlashBid(FastLaneFlashBidPostData::new(param, id)),
            RelayType::FastLaneFastBid => PostData::FastLaneFastBid(FastLaneFastBidPostData::new(param, id)),
            RelayType::Marlin => PostData::Marlin(MarlinPostData::new(param, id, block_number)),
        }
    }

    pub fn to_json(&self) -> String {
        match self {
            PostData::FastLaneFlashBid(data) => serde_json::to_string(data).unwrap(),
            PostData::FastLaneFastBid(data) => serde_json::to_string(data).unwrap(),
            PostData::Marlin(data) => serde_json::to_string(data).unwrap(),
        }
    }
}
