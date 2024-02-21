use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use ethers::types::U64;
use reqwest::header::CONTENT_TYPE;
use reqwest::header::{HeaderValue, CONNECTION};
use reqwest::Client;
use tokio::sync::Mutex;

use crate::post_data::PostData;
use crate::relay_type::RelayType;

const FLASH_RELAY: &str = "https://beta-rpc.fastlane-labs.xyz";
const MARLIN_RELAY: &str = "https://bor.txrelay.marlin.org";

pub struct BundleProvider {
    client: Arc<Client>,
    id: Mutex<u32>,
}

impl BundleProvider {
    pub(super) async fn new() -> Self {
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

    async fn get_id(&self) -> u32 {
        let mut id_guard = self.id.lock().await;
        let id = *id_guard;
        *id_guard = id.wrapping_add(1);
        id
    }

    async fn send_bundle(&self, post_data: PostData) -> reqwest::Result<String> {
        let relay = match post_data {
            PostData::FastLaneFastBid(_) => FLASH_RELAY,
            PostData::FastLaneFlashBid(_) => FLASH_RELAY,
            PostData::Marlin(_) => MARLIN_RELAY,
        };

        println!("post_data: {}", post_data.to_json());

        self.client
            .post(relay)
            .header(CONTENT_TYPE, "application/json")
            .body(post_data.to_json())
            .send()
            .await?
            .text()
            .await
    }

    pub(super) async fn ping(&self) {
        let start_time = Instant::now();
        let response = self.client.get(&format!("{}/ping", FLASH_RELAY)).send().await.unwrap();

        let status = response.status();
        let response_time = start_time.elapsed().as_millis();
        println!("PONG : {:?}, responseTime: {}", status, response_time);
    }

    pub async fn send_flashbid_bundle(&self, bundle: Vec<String>) -> reqwest::Result<String> {
        let post_data_fast_lane = PostData::new(
            bundle,
            self.get_id().await,
            RelayType::FastLaneFlashBid,
            U64::zero(),
        );

        self.send_bundle(post_data_fast_lane).await
    }

    pub async fn send_fastbid(&self, tx: String) -> reqwest::Result<String> {
        let post_data_fast_lane = PostData::new(vec![tx], self.get_id().await, RelayType::FastLaneFastBid, U64::zero());

        self.send_bundle(post_data_fast_lane).await
    }

    pub async fn send_marlin_bundle(&self, bundle: Vec<String>, block_number: U64) -> String {
        let post_data_fast_lane = PostData::new(bundle, self.get_id().await, RelayType::Marlin, block_number);

        // let json_post = post_data_fast_lane.to_json();
        // write_to_json("test.json", &json_post);

        match self.send_bundle(post_data_fast_lane).await {
            Ok(r) => r,
            Err(e) => e.to_string(),
        }
    }
}
