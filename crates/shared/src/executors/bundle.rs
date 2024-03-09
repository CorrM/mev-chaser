use std::{sync::Arc, thread, time::Duration};

pub use bundle_provider::*;
pub use post_data::*;
pub use relay_type::*;

pub mod bundle_provider;
pub mod post_data;
pub mod relay_type;

pub fn fast_bundle_provider() -> Arc<BundleProvider> {
    let bundle_provider = Arc::new(BundleProvider::new());
    bundle_provider.ping();

    let provider_arc_ref: Arc<BundleProvider> = Arc::clone(&bundle_provider);

    // https://ryhl.io/blog/async-what-is-blocking/#spawn-a-dedicated-thread
    thread::spawn(move || {
        let duration = Duration::from_secs(60);
        loop {
            thread::sleep(duration);
            provider_arc_ref.ping();
        }
    });

    bundle_provider
}
