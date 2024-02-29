use std::str::FromStr;
use std::sync::Arc;

use anyhow::Result;
use ethers::prelude::{PubsubClient, U64};
use ethers::providers::Middleware;
use ethers::types::{Address, BlockNumber};
use hashbrown::HashMap;
use tokio::time::Instant;

use contracts::balancer_flash_loan_recipient::OneSwapInfo;
use shared::{
    executors::FastLineExecutor,
    simulator::EvmSimulator,
    types::{MevActions, MevEvents},
};
use vidger::logger::info;
use vidger::{
    collectors::BlockCollector,
    core::{CollectorMapper, ExecutorMapper},
    executors::MempoolExecutor,
    notifiers::TelegramNotifier,
    VidgerEngine,
};

use crate::utilities::env::Env;

pub struct RunCommand;

impl RunCommand {
    async fn test<M>(provider: Arc<M>)
    where
        M: Middleware + 'static,
        M::Provider: PubsubClient,
    {
        let swaps: Vec<OneSwapInfo> = vec![
            shared::types::pool_path::make_uniswap_v2_protocol_swap_info(
                Address::from_str("0xa5E0829CaCEd8fFDD4De3c43696c57F7D7A678ff").unwrap(),
                vec![
                    Address::from_str("0xc2132D05D31c914a87C6611C10748AEb04B58e8F").unwrap(),
                    Address::from_str("0x346404079b3792a6c548B072B9C4DDdFb92948d5").unwrap(),
                ],
                10000000,
                0,
            )
            .unwrap(),
            shared::types::pool_path::make_uniswap_v2_protocol_swap_info(
                Address::from_str("0xa5E0829CaCEd8fFDD4De3c43696c57F7D7A678ff").unwrap(),
                vec![
                    Address::from_str("0x346404079b3792a6c548B072B9C4DDdFb92948d5").unwrap(),
                    Address::from_str("0xc2132D05D31c914a87C6611C10748AEb04B58e8F").unwrap(),
                ],
                0,
                1000,
            )
            .unwrap(),
        ];

        let block_number: U64 = provider
            .get_block(BlockNumber::Latest)
            .await
            .unwrap()
            .unwrap()
            .number
            .unwrap();

        let simulator = EvmSimulator::new_ethers(provider, &[]).await;

        let start = Instant::now();
        //simulator.multicall_multi_swap(block_number, swaps, true).await.unwrap();
        let result: Result<HashMap<Address, Result<Option<i32>>>> = simulator.get_tokens_balance_slot(
            &[Address::from_str("0x7ceB23fD6bC0adD59E62ac25578270cFf1b9f619").unwrap()],
            block_number,
        );
        println!("duration: {}ms", start.elapsed().as_millis());
        info!("result: {:?}", result);
    }

    pub async fn process<M>(env: &Env, provider: Arc<M>) -> Result<()>
    where
        M: Middleware + 'static,
        M::Provider: PubsubClient,
    {
        Self::test(Arc::clone(&provider)).await;
        return Ok(());

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
