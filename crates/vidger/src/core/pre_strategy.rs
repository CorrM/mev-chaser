use anyhow::Result;

/// Strategy trait, which defines the core logic for each opportunity.
pub trait PreStrategy<E>: Send + Sync {
    /// Sync the initial state of the strategy if needed, usually by fetching
    /// onchain data, Only called once on startup.
    fn sync_state(&mut self) -> Result<()>;

    /// Handle incoming events.
    fn on_event(&mut self, event: &mut E);
}
