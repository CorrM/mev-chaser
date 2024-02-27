use std::sync::Arc;

use anyhow::Result;
use ethers::prelude::PubsubClient;
use ethers::providers::Middleware;

use shared::executors::FastLineExecutor;
use shared::types::{MevActions, MevEvents};
use vidger::collectors::BlockCollector;
use vidger::core::{CollectorMapper, ExecutorMapper, Notifier};
use vidger::executors::MempoolExecutor;
use vidger::notifiers::TelegramNotifier;
use vidger::types::Notification;
use vidger::VidgerEngine;

use crate::utilities::env::Env;

pub struct RunCommand;

impl RunCommand {
    pub async fn process<M>(env: &Env, provider: Arc<M>) -> Result<()>
    where
        M: Middleware + 'static,
        M::Provider: PubsubClient,
    {
        let mut engine: VidgerEngine<MevEvents, MevActions> = VidgerEngine::new();

        // Set up block collector.
        let block_collector = Box::new(BlockCollector::new(Arc::clone(&provider)));
        let block_collector = CollectorMapper::new(block_collector, MevEvents::NewBlock);
        engine.add_collector(Box::new(block_collector));

        /*
        // Set up pre-strategy
        let pre_strategy = PreStrategy::new(Arc::clone(&provider));
        engine.set_pre_strategy(Box::new(pre_strategy));

        // Set up strategy.
        let configs = BackRunningStrategyConfig {
            sando_address: config.sando_address,
            sando_inception_block: config.sando_inception_block,
            searcher_signer,
        };
        let strategy = BackRunningStrategy::new(provider.clone(), configs);
        engine.add_strategy(Box::new(strategy));
        */

        let executor = Box::new(MempoolExecutor::new(Arc::clone(&provider)));
        let executor = ExecutorMapper::new(executor, |action| match action {
            MevActions::SubmitTxToMempool(tx) => Some(tx),
            _ => None,
        });
        engine.add_executor(Box::new(executor));

        let executor = Box::new(FastLineExecutor::new(Arc::clone(&provider)));
        let executor = ExecutorMapper::new(executor, |action| match action {
            MevActions::SubmitTxToFastLine(tx) => Some(tx),
            _ => None,
        });
        engine.add_executor(Box::new(executor));

        // Set up notifier.
        let notifier = TelegramNotifier::new(env.telegram_token_id.clone(), env.telegram_channel_id.clone());
        notifier
            .notify(Notification {
                message: "Hello, World!".to_owned(),
            })
            .await?;
        engine.add_notifier(Box::new(notifier));

        // Start engine.
        if let Ok(mut set) = engine.run().await {
            while let Some(res) = set.join_next().await {
                println!("res: {:?}", res);
            }
        }

        Ok(())
    }
}
