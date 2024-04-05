use std::sync::{Arc, RwLock};

use anyhow::{anyhow, Result};
use ethers::{providers::Middleware, providers::PubsubClient, signers::LocalWallet};

use shared::{
    database::Database,
    executors::FastLineExecutor,
    managers::{AmmManager, BlockManager, PoolManager, TokenManager},
    simulator::EvmSimulator,
    types::{MevActions, MevEvents},
};
use vidger::{
    collectors::BlockCollector,
    core::{CollectorMapper, ExecutorMapper},
    executors::MempoolExecutor,
    logger::info,
    notifiers::TelegramNotifier,
    types::NetworkKind,
    utilities::block_on,
    VidgerEngine,
};

use crate::strategy::back_running_strategy::{BackRunningStrategy, BackRunningStrategyConfig};
use crate::strategy::main_pre_strategy::MainPreStrategy;
use crate::utilities::env::Env;

pub struct RunCommand;

impl RunCommand {
    /*fn test<M>(provider: Arc<M>)
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
        simulator
            .as_ethers()
            .unwrap()
            .multicall_multi_swap(block_number, swaps, true)
            .await
            .unwrap();
        //let result: Result<HashMap<Address, Result<Option<i32>>>> = simulator.get_tokens_balance_slot(
        //    &[Address::from_str("0x7ceB23fD6bC0adD59E62ac25578270cFf1b9f619").unwrap()],
        //    block_number,
        //);
        println!("duration: {}ms", start.elapsed().as_millis());
        //info!("result: {:?}", result);
    }*/

    pub fn process<M>(env: &Env, db: Database, network: NetworkKind, provider: &Arc<M>) -> Result<()>
    where
        M: Middleware + 'static,
        M::Provider: PubsubClient,
    {
        //Self::test(Arc::clone(&provider)).await;
        //return Ok(());

        // Set up signer.
        info!("Setting up signer");
        let searcher_signer = env
            .private_key
            .parse::<LocalWallet>()
            .map_err(|_| anyhow!("Failed to parse \"PRIVATE_KEY\""))?;

        // Set up managers.
        info!("Setting up managers:");

        info!("  - BlockManager .. ⏳");
        let block_manager = Arc::new(RwLock::new(BlockManager::new()));
        info!("  - BlockManager .. ✅");

        info!("  - TokenManager .. ⏳");
        let token_manager = TokenManager::new_by_db(&db, &network)?;
        info!("  - TokenManager .. ✅");

        info!("  - AmmManager .. ⏳");
        let amm_manager = AmmManager::new_by_db(&db, &network, &token_manager)?;
        info!("  - AmmManager .. ✅");

        info!("  - PoolManager .. ⏳");
        let simulator = Arc::new(RwLock::new(EvmSimulator::new(Arc::clone(provider), &amm_manager)?));
        let pool_manager = Arc::new(RwLock::new(PoolManager::new(
            Arc::clone(provider),
            Arc::clone(&simulator),
            &amm_manager,
        )));
        info!("  - PoolManager .. ✅");

        // Set up engine.
        info!("Setting up engine");
        let mut engine: VidgerEngine<MevEvents, MevActions> = VidgerEngine::new();

        // Set up block collector.
        let block_collector = Box::new(BlockCollector::new(Arc::clone(provider)));
        let block_collector = CollectorMapper::new(block_collector, MevEvents::NewBlock);
        engine.add_collector(Box::new(block_collector));

        //let mempool_collector = Box::new(MempoolCollector::new(Arc::clone(&provider), true));
        //let mempool_collector = CollectorMapper::new(mempool_collector, MevEvents::NewTransaction);
        //engine.add_collector(Box::new(mempool_collector));

        // Set up pre-strategy
        let pre_strategy = MainPreStrategy::new(
            Arc::clone(provider),
            Arc::clone(&pool_manager),
            Arc::clone(&block_manager),
            Arc::clone(&simulator),
        );
        engine.set_pre_strategy(Box::new(pre_strategy));

        // Set up strategy.
        let config = BackRunningStrategyConfig { searcher_signer };
        let strategy = BackRunningStrategy::new(
            Arc::clone(provider),
            config,
            Arc::clone(&pool_manager),
            Arc::clone(&block_manager),
        );
        engine.add_strategy(Box::new(strategy));

        // Set up executor.
        let executor = Box::new(FastLineExecutor::new(Arc::clone(provider)));
        let executor = ExecutorMapper::new(executor, |action| match action {
            MevActions::SubmitTxToFastLine(tx) => Some(tx),
            _ => None,
        });
        engine.add_executor(Box::new(executor));

        let executor = Box::new(MempoolExecutor::new(Arc::clone(provider)));
        let executor = ExecutorMapper::new(executor, |action| match action {
            MevActions::SubmitTxToMempool(tx) => Some(tx),
            _ => None,
        });
        engine.add_executor(Box::new(executor));

        // Set up notifier.
        let notifier = TelegramNotifier::new(env.telegram_token_id.clone(), env.telegram_channel_id.clone());
        engine.add_notifier(Box::new(notifier));

        // Start engine.
        info!("Starting engine");
        if let Ok(mut set) = block_on(engine.run()) {
            while let Some(res) = block_on(set.join_next()) {
                println!("res: {:?}", res);
            }
        }

        Ok(())
    }
}
