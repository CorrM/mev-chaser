use anyhow::Result;
use tokio::sync::broadcast::{self, Sender};
use tokio::task::JoinSet;
use tokio_stream::StreamExt;

use crate::core::{Collector, Executor, Notifier, Strategy};

/// The main engine of Vidger. This struct is responsible for orchestrating the
/// data flow between collectors, strategies, and executors.
pub struct VidgerEngine<E, A> {
    /// The set of collectors that the engine will use to collect events.
    collectors: Vec<Box<dyn Collector<E>>>,

    /// The set of strategies that the engine will use to process events.
    strategies: Vec<Box<dyn Strategy<E, A>>>,

    /// The set of executors that the engine will use to execute actions.
    executors: Vec<Box<dyn Executor<A>>>,

    /// The notifier responsible for alerting after the executors have finished.
    notifier: Vec<Box<dyn Notifier<A>>>,

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

impl<E, A> Default for VidgerEngine<E, A> {
    fn default() -> Self {
        Self::new()
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
    pub fn add_notifier(&mut self, notifier: Box<dyn Notifier<A>>) {
        self.notifier.push(notifier);
    }

    /// The core run loop of the engine. This function will spawn a thread for
    /// each collector, strategy, and executor. It will then orchestrate the
    /// data flow between them.
    pub async fn run(self) -> Result<JoinSet<()>> {
        let (event_sender, _): (Sender<E>, _) = broadcast::channel(self.event_channel_capacity);
        let (action_sender, _): (Sender<A>, _) = broadcast::channel(self.action_channel_capacity);
        let (notify_sender, _): (Sender<A>, _) = broadcast::channel(self.action_channel_capacity);

        let mut set = JoinSet::new();

        // Spawn notifiers in separate threads.
        for notifier in self.notifier {
            let mut receiver = notify_sender.subscribe();
            set.spawn(async move {
                loop {
                    match receiver.recv().await {
                        Ok(action) => match notifier.notify(action).await {
                            Ok(_) => {}
                            Err(e) => println!("error notifying: {}", e),
                        },
                        Err(e) => println!("error receiving notification: {}", e),
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
                        Ok(mut action) => match executor.execute(&mut action).await {
                            Ok(_) => match notify.send(action) {
                                Ok(_) => {}
                                Err(e) => println!("error sending notification: {:?}", e),
                            },
                            Err(e) => println!("error executing action: {:?}", e),
                        },
                        Err(e) => println!("error receiving action: {}", e),
                    }
                }
            });
        }

        // Spawn strategies in separate threads.
        for mut strategy in self.strategies {
            let mut event_receiver = event_sender.subscribe();
            let action_sender = action_sender.clone();
            strategy.sync_state().await?;

            set.spawn(async move {
                loop {
                    match event_receiver.recv().await {
                        Ok(event) => {
                            for action in strategy.process_event(event).await {
                                match action_sender.send(action) {
                                    Ok(_) => {}
                                    Err(e) => println!("error sending action: {}", e),
                                }
                            }
                        }
                        Err(e) => println!("error receiving event: {}", e),
                    }
                }
            });
        }

        // Spawn collectors in separate threads.
        for collector in self.collectors {
            let event_sender = event_sender.clone();
            set.spawn(async move {
                let mut event_stream = collector.get_event_stream().await.unwrap();
                while let Some(event) = event_stream.next().await {
                    match event_sender.send(event) {
                        Ok(_) => {}
                        Err(e) => println!("error sending event: {}", e),
                    }
                }
            });
        }

        Ok(set)
    }
}
