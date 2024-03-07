use anyhow::Result;

use crate::types::Notification;

/// Notifier trait, responsible for alerting when an action has been executed.
pub trait Notifier: Send + Sync {
    /// Notify an action has been executed.
    fn notify(&self, notification: Notification) -> Result<()>;
}
