pub use bundle_provider::*;
pub use post_data::*;
pub use relay_type::*;
use std::{sync::Arc, time::Duration};
use tokio::time::interval;

pub mod bundle_provider;
pub mod post_data;
pub mod relay_type;

pub async fn fast_bundle_provider() -> Arc<BundleProvider> {
    let bundle_provider = Arc::new(BundleProvider::new().await);
    bundle_provider.ping().await;

    let mut interval = interval(Duration::from_secs(60));
    let provider_arc_ref = Arc::clone(&bundle_provider);

    tokio::spawn(async move {
        loop {
            interval.tick().await;
            provider_arc_ref.ping().await;
        }
    });

    bundle_provider
}
