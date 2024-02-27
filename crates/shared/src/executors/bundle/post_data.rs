use ethers::types::U64;
use serde::{Deserialize, Serialize};

use crate::executors::bundle::post_data::{
    fast_lane_fast_bid_post_data::FastLaneFastBidPostData, fast_lane_flash_bid_post_data::FastLaneFlashBidPostData,
    marlin_post_data::MarlinPostData,
};
use crate::executors::bundle::RelayType;

mod fast_lane_fast_bid_post_data;
mod fast_lane_flash_bid_post_data;
mod marlin_post_data;

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
