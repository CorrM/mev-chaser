use std::sync::{Mutex, MutexGuard};
use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use anyhow::{Context, Result};
use ethers::types::U64;
use reqwest::{
    header::CONTENT_TYPE,
    header::{HeaderValue, CONNECTION},
    Client, Response, StatusCode,
};

use vidger::utilities::block_on;

use crate::executors::bundle::{PostData, RelayType};

const FLASH_RELAY: &str = "https://beta-rpc.fastlane-labs.xyz";
const MARLIN_RELAY: &str = "https://bor.txrelay.marlin.org";

pub struct BundleProvider {
    client: Arc<Client>,
    id: Mutex<u32>,
}

impl BundleProvider {
    pub(super) fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(60))
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));
                headers
            })
            .pool_max_idle_per_host(5)
            .pool_idle_timeout(None)
            .build()
            .unwrap();

        Self {
            client: Arc::new(client),
            id: Mutex::new(1),
        }
    }

    fn get_id(&self) -> u32 {
        let mut id_guard: MutexGuard<u32> = self.id.lock().unwrap();
        let id: u32 = *id_guard;
        *id_guard = id.wrapping_add(1);
        id
    }

    fn send_bundle(&self, post_data: PostData) -> Result<String> {
        let relay: &str = match post_data {
            PostData::FastLaneFastBid(_) => FLASH_RELAY,
            PostData::FastLaneFlashBid(_) => FLASH_RELAY,
            PostData::Marlin(_) => MARLIN_RELAY,
        };

        println!("post_data: {}", post_data.to_json());

        block_on(
            block_on(
                self.client
                    .post(relay)
                    .header(CONTENT_TYPE, "application/json")
                    .body(post_data.to_json())
                    .send(),
            )?
            .text(),
        )
        .context("Failed to send bundle: {}")
    }

    pub(super) fn ping(&self) {
        let start_time = Instant::now();
        let response: Response = block_on(self.client.get(&format!("{}/ping", FLASH_RELAY)).send()).unwrap();

        let status: StatusCode = response.status();
        println!(
            "PONG : {:?}, responseTime: {}",
            status,
            start_time.elapsed().as_millis()
        );
    }

    pub fn send_flashbid_bundle(&self, bundle: Vec<String>) -> Result<String> {
        let post_data_fast_lane = PostData::new(bundle, self.get_id(), RelayType::FastLaneFlashBid, U64::zero());
        self.send_bundle(post_data_fast_lane)
    }

    pub fn send_fastbid(&self, tx: String) -> Result<String> {
        let post_data_fast_lane = PostData::new(vec![tx], self.get_id(), RelayType::FastLaneFastBid, U64::zero());

        self.send_bundle(post_data_fast_lane)
    }

    pub fn send_marlin_bundle(&self, bundle: Vec<String>, block_number: U64) -> String {
        let post_data_fast_lane = PostData::new(bundle, self.get_id(), RelayType::Marlin, block_number);

        // let json_post = post_data_fast_lane.to_json();
        // write_to_json("test.json", &json_post);

        self.send_bundle(post_data_fast_lane).unwrap_or_else(|e| e.to_string())
    }
}
