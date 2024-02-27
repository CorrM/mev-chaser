use anyhow::Result;
use async_trait::async_trait;

use crate::types::Notification;

/// Notifier trait, responsible for alerting when an action has been executed.
#[async_trait]
pub trait Notifier: Send + Sync {
    /// Notify an action has been executed.
    async fn notify(&self, notification: Notification) -> Result<()>;
}
