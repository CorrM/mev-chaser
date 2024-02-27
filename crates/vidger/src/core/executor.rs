use anyhow::Result;
use async_trait::async_trait;

use crate::types::Notification;

/// Executor trait, responsible for executing actions returned by strategies.
#[async_trait]
pub trait Executor<A>: Send + Sync {
    /// Execute an action.
    async fn execute(&self, action: A) -> Result<Option<Notification>>;
}

/// ExecutorMap is a wrapper around an [Executor](Executor) that maps incoming
/// actions to a different type.
pub struct ExecutorMapper<A, F> {
    executor: Box<dyn Executor<A>>,
    f: F,
}

impl<A, F> ExecutorMapper<A, F> {
    pub fn new(executor: Box<dyn Executor<A>>, f: F) -> Self {
        Self { executor, f }
    }
}

#[async_trait]
impl<A1, A2, F> Executor<A1> for ExecutorMapper<A2, F>
where
    A1: Send + Sync + 'static,
    A2: Send + Sync + 'static,
    F: Fn(A1) -> Option<A2> + Send + Sync + Clone + 'static,
{
    async fn execute(&self, action: A1) -> Result<Option<Notification>> {
        let action: Option<A2> = (self.f)(action);
        match action {
            Some(action) => self.executor.execute(action).await,
            None => Ok(None),
        }
    }
}
