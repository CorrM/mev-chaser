use anyhow::Result;
use log::error;
use tokio::sync::broadcast::{self, Receiver, Sender};
use tokio::task::JoinSet;
use tokio_stream::StreamExt;

use crate::core::{Collector, Executor, Notifier, Strategy};
use crate::types::Notification;

/// The main engine of Vidger. This struct is responsible for orchestrating the
/// data flow between collectors, strategies, and executors.
pub struct VidgerEngine<E, A> {
    /// The set of collectors that the engine will use to collect events.
    collectors: Vec<Box<dyn Collector<E>>>,

    /// The set of strategies that the engine will use to process events.
    strategies: Vec<Box<dyn Strategy<E, A>>>,

    /// The set of executors that the engine will use to execute actions.
    executors: Vec<Box<dyn Executor<A>>>,

    /// The set of executors that the engine will for alerting after the executors process actions.
    notifier: Vec<Box<dyn Notifier>>,

    /// The pre strategy will use to react to events before the strategies process them.
    /// Good to handle events that are not related to the strategies like update last block state.
    /// result(Actions) of the pre strategy is ignored.
    pre_strategy: Option<Box<dyn Strategy<E, A>>>,

    /// The capacity of the event channel.
    event_channel_capacity: usize,

    /// The capacity of the action channel.
    action_channel_capacity: usize,
}

impl<E, A> VidgerEngine<E, A> {
    pub fn new() -> Self {
        Self {
            collectors: vec![],
            strategies: vec![],
            executors: vec![],
            notifier: vec![],
            pre_strategy: None,
            event_channel_capacity: 512,
            action_channel_capacity: 512,
        }
    }

    pub fn with_event_channel_capacity(mut self, capacity: usize) -> Self {
        self.event_channel_capacity = capacity;
        self
    }

    pub fn with_action_channel_capacity(mut self, capacity: usize) -> Self {
        self.action_channel_capacity = capacity;
        self
    }
}

impl<E, A> VidgerEngine<E, A>
where
    E: Send + Clone + std::fmt::Debug + 'static,
    A: Send + Clone + std::fmt::Debug + 'static,
{
    /// Adds a collector to be used by the engine.
    pub fn add_collector(&mut self, collector: Box<dyn Collector<E>>) {
        self.collectors.push(collector);
    }

    /// Adds a strategy to be used by the engine.
    pub fn add_strategy(&mut self, strategy: Box<dyn Strategy<E, A>>) {
        self.strategies.push(strategy);
    }

    /// Adds an executor to be used by the engine.
    pub fn add_executor(&mut self, executor: Box<dyn Executor<A>>) {
        self.executors.push(executor);
    }

    /// Adds a notifier to be used by the engine.
    pub fn add_notifier(&mut self, notifier: Box<dyn Notifier>) {
        self.notifier.push(notifier);
    }

    /// Adds a strategy to be used by the engine.
    pub fn set_pre_strategy(&mut self, strategy: Box<dyn Strategy<E, A>>) {
        self.pre_strategy = Some(strategy);
    }

    /// The core run loop of the engine. This function will spawn a thread for
    /// each collector, strategy, and executor. It will then orchestrate the
    /// data flow between them.
    pub async fn run(self) -> Result<JoinSet<()>> {
        let (pre_event_sender, _): (Sender<E>, _) = broadcast::channel(self.event_channel_capacity);
        let (post_event_sender, _): (Sender<E>, _) = broadcast::channel(self.event_channel_capacity);
        let (action_sender, _): (Sender<A>, _) = broadcast::channel(self.action_channel_capacity);
        let (notify_sender, _): (Sender<Notification>, _) = broadcast::channel(self.action_channel_capacity);

        let mut set = JoinSet::new();

        // Spawn notifiers in separate threads.
        for notifier in self.notifier {
            let mut receiver = notify_sender.subscribe();
            set.spawn(async move {
                loop {
                    match receiver.recv().await {
                        Ok(notification) => match notifier.notify(notification).await {
                            Ok(_) => {}
                            Err(e) => error!("error notifying: {}", e),
                        },
                        Err(e) => error!("error receiving notification: {}", e),
                    }
                }
            });
        }

        // Spawn executors in separate threads.
        for executor in self.executors {
            let mut receiver = action_sender.subscribe();
            let notify = notify_sender.clone();
            set.spawn(async move {
                loop {
                    match receiver.recv().await {
                        Ok(action) => match executor.execute(action.clone()).await {
                            Ok(Some(notification)) => match notify.send(notification) {
                                Ok(_) => {}
                                Err(e) => error!("error sending notification: {:?}", e),
                            },
                            Ok(None) => {}
                            Err(e) => error!("error executing action: {:?}", e),
                        },
                        Err(e) => error!("error receiving action: {}", e),
                    }
                }
            });
        }

        // Spawn strategies in separate threads.
        for mut strategy in self.strategies {
            let mut event_receiver: Receiver<E> = if self.pre_strategy.is_some() {
                post_event_sender.subscribe()
            } else {
                pre_event_sender.subscribe()
            };

            let action_sender: Sender<A> = action_sender.clone();
            strategy.sync_state().await?;

            set.spawn(async move {
                loop {
                    match event_receiver.recv().await {
                        Ok(mut event) => {
                            for action in strategy.process_event(&mut event).await {
                                match action_sender.send(action) {
                                    Ok(_) => {}
                                    Err(e) => error!("error sending action: {}", e),
                                }
                            }
                        }
                        Err(e) => error!("error receiving event: {}", e),
                    }
                }
            });
        }

        // Spawn pre_strategy in separate thread.
        if let Some(mut pre_strategy) = self.pre_strategy {
            pre_strategy.sync_state().await?;
            let mut event_receiver: Receiver<E> = pre_event_sender.subscribe();

            set.spawn(async move {
                loop {
                    match event_receiver.recv().await {
                        Ok(mut event) => {
                            // result are ignored
                            pre_strategy.process_event(&mut event).await;

                            match post_event_sender.send(event) {
                                Ok(_) => {}
                                Err(e) => error!("error post_event_sender event: {}", e),
                            }
                        }
                        Err(e) => error!("error receiving event: {}", e),
                    }
                }
            });
        };

        // Spawn collectors in separate threads.
        for collector in self.collectors {
            let event_sender: Sender<E> = pre_event_sender.clone();

            set.spawn(async move {
                let mut event_stream = collector.get_event_stream().await.unwrap();
                while let Some(event) = event_stream.next().await {
                    match event_sender.send(event) {
                        Ok(_) => {}
                        Err(e) => error!("error sending event: {}", e),
                    }
                }
            });
        }

        Ok(set)
    }
}

impl<E, A> Default for VidgerEngine<E, A> {
    fn default() -> Self {
        Self::new()
    }
}
