use anyhow::Result;

/// Strategy trait, which defines the core logic for each opportunity.
pub trait Strategy<E, A>: Send + Sync {
    /// Sync the initial state of the strategy if needed, usually by fetching
    /// onchain data, Only called once on startup.
    fn sync_state(&mut self) -> Result<()>;

    /// Process an event, and return an action if needed.
    fn process_event(&mut self, event: &mut E) -> Option<A>;
}
