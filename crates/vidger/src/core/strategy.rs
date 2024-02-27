use async_trait::async_trait;

/// Strategy trait, which defines the core logic for each opportunity.
#[async_trait]
pub trait Strategy<E, A>: Send + Sync {
    /// Sync the initial state of the strategy if needed, usually by fetching
    /// onchain data, Only called once on startup.
    async fn sync_state(&mut self) -> anyhow::Result<()>;

    /// Process an event, and return an action if needed.
    async fn process_event(&mut self, event: E) -> Vec<A>;
}
