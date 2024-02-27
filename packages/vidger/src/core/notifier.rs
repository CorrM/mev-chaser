use async_trait::async_trait;

/// Notifier trait, responsible for alerting when an action has been executed.
#[async_trait]
pub trait Notifier<A>: Send + Sync {
    /// Notify an action has been executed.
    async fn notify(&self, action: A) -> anyhow::Result<()>;
}
