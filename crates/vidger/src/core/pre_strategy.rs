use anyhow::Result;
use async_trait::async_trait;

/// Strategy trait, which defines the core logic for each opportunity.
#[async_trait]
pub trait PreStrategy<E>: Send + Sync {
    /// Sync the initial state of the strategy if needed, usually by fetching
    /// onchain data, Only called once on startup.
    async fn sync_state(&mut self) -> Result<()>;

    /// Handle incoming events.
    async fn on_event(&mut self, event: &mut E);
}
