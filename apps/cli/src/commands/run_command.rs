use std::str::FromStr;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use ethers::prelude::{PubsubClient, U64};
use ethers::providers::Middleware;
use ethers::signers::LocalWallet;
use ethers::types::{Address, BlockNumber};
use tokio::time::Instant;

use contracts::balancer_flash_loan_recipient::OneSwapInfo;
use shared::amm::{AmmPoolKind, AmmProtocolKind, UniswapV2Pool, UniswapV2Protocol};
use shared::database::{Database, DbDex, DbDexPool, DbToken, DbTokenNetwork};
use shared::managers::{AmmManager, BlockManager, PoolManager, TokenManager};
use shared::{
    executors::FastLineExecutor,
    simulator::EvmSimulator,
    types::{MevActions, MevEvents},
};
use vidger::types::{CryptoToken, NetworkKind};
use vidger::{
    collectors::BlockCollector,
    core::{CollectorMapper, ExecutorMapper},
    executors::MempoolExecutor,
    notifiers::TelegramNotifier,
    VidgerEngine,
};

use crate::strategy::back_running_strategy::{BackRunningStrategy, BackRunningStrategyConfig};
use crate::strategy::main_pre_strategy::MainPreStrategy;
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
    }

    fn get_tokens(db: &Database, network: &NetworkKind) -> Result<Vec<CryptoToken>> {
        let db_tokens: Vec<(DbToken, DbTokenNetwork)> = db.get_tokens(network)?;
        let mut tokens: Vec<CryptoToken> = Vec::new();

        for (db_token, db_token_network) in db_tokens {
            tokens.push(CryptoToken::new(
                db_token_network.address,
                None, // TODO
                db_token.name,
                db_token.symbol,
                db_token.decimals as u8,
                0, // TODO
            )?);
        }

        Ok(tokens)
    }

    fn get_dexes(
        db: &Database,
        network: &NetworkKind,
        token_manager: &TokenManager,
    ) -> Result<Vec<Arc<AmmProtocolKind>>> {
        let mut ret: Vec<Arc<AmmProtocolKind>> = Vec::new();

        let db_dexes: Vec<DbDex> = db.get_dexes_by_network(network)?;
        for db_dex in db_dexes {
            let Some(db_dex_protocol) = db.get_dex_protocol_by_id(db_dex.dex_protocol_id)? else {
                continue;
            };

            let Some(db_dex_network) = db.get_dex_network(db_dex.id, network)? else {
                continue;
            };

            let db_dex_pools: Vec<DbDexPool> = db.get_dex_pools_by_dex_id(db_dex.id, network, true)?;
            match db_dex_protocol.name.as_str() {
                "UniswapV2" => {
                    //let dex_options: serde_json::Value = serde_json::from_str(&db_dex.options)?;
                    let network_options: serde_json::Value = serde_json::from_str(&db_dex_network.options)?;

                    let uniswap_v2 = Arc::new(AmmProtocolKind::UniswapV2(UniswapV2Protocol::new(
                        db_dex.name,
                        network_options["factory"].as_str().unwrap(),
                        network_options["router"].as_str().unwrap(),
                    )?));

                    let mut pools: Vec<AmmPoolKind> = Vec::with_capacity(db_dex_pools.len());
                    for db_dex_pool in db_dex_pools {
                        let pool_address: Address = db_dex_pool.address.parse::<Address>()?;
                        if pool_address.is_zero() {
                            continue;
                        }

                        let token0: Option<DbToken> = db.get_token_by_id(db_dex_pool.token0_id)?;
                        let token1: Option<DbToken> = db.get_token_by_id(db_dex_pool.token1_id)?;
                        if token0.is_none() || token1.is_none() {
                            return Err(anyhow!("Token not found"));
                        }

                        let db_token0_network: DbTokenNetwork =
                            db.get_token_network_by_token(token0.unwrap().id, network)?.unwrap();
                        let db_token1_network: DbTokenNetwork =
                            db.get_token_network_by_token(token1.unwrap().id, network)?.unwrap();

                        let token0: Arc<CryptoToken> =
                            token_manager.get_by_address_str(&db_token0_network.address).unwrap();
                        let token1: Arc<CryptoToken> =
                            token_manager.get_by_address_str(&db_token1_network.address).unwrap();

                        let pool = AmmPoolKind::UniswapV2(UniswapV2Pool::new(
                            pool_address,
                            Arc::clone(&uniswap_v2),
                            token0,
                            token1,
                        )?);
                        pools.push(pool);
                    }

                    unsafe {
                        let uniswap_v2 = Arc::into_raw(uniswap_v2) as *mut AmmProtocolKind;

                        for pool in pools {
                            (*uniswap_v2).add_pool(pool);
                        }

                        ret.push(Arc::from_raw(uniswap_v2));
                    }
                }
                _ => panic!("Unsupported dex protocol"),
            }
        }

        Ok(ret)
    }

    pub async fn process<M>(env: &Env, db: Database, network: NetworkKind, provider: Arc<M>) -> Result<()>
    where
        M: Middleware + 'static,
        M::Provider: PubsubClient,
    {
        //Self::test(Arc::clone(&provider)).await;
        //return Ok(());

        let searcher_signer = env
            .private_key
            .parse::<LocalWallet>()
            .map_err(|_| anyhow!("Failed to parse \"PRIVATE_KEY\""))?;

        let tokens: Vec<CryptoToken> = Self::get_tokens(&db, &network).expect("Failed to get tokens");
        let simulator = Arc::new(tokio::sync::RwLock::new(
            EvmSimulator::new_revm(Arc::clone(&provider), &tokens).await,
        ));

        // Set up managers.
        let token_manager = TokenManager::new(tokens, &network);
        let amm_manager = AmmManager::new(Self::get_dexes(&db, &network, &token_manager)?);
        let pool_manager = PoolManager::new(Arc::clone(&provider), Arc::clone(&simulator), &amm_manager);
        let block_manager = BlockManager::new();

        let pool_manager = Arc::new(tokio::sync::RwLock::new(pool_manager));
        let block_manager = Arc::new(tokio::sync::RwLock::new(block_manager));

        // Set up engine.
        let mut engine: VidgerEngine<MevEvents, MevActions> = VidgerEngine::new();

        // Set up block collector.
        let block_collector = Box::new(BlockCollector::new(Arc::clone(&provider)));
        let block_collector = CollectorMapper::new(block_collector, MevEvents::NewBlock);
        engine.add_collector(Box::new(block_collector));

        //let mempool_collector = Box::new(MempoolCollector::new(Arc::clone(&provider), true));
        //let mempool_collector = CollectorMapper::new(mempool_collector, MevEvents::NewTransaction);
        //engine.add_collector(Box::new(mempool_collector));

        // Set up pre-strategy
        let pre_strategy = MainPreStrategy::new(
            Arc::clone(&provider),
            Arc::clone(&pool_manager),
            Arc::clone(&block_manager),
            Arc::clone(&simulator),
        );
        engine.set_pre_strategy(Box::new(pre_strategy));

        // Set up strategy.
        let config = BackRunningStrategyConfig { searcher_signer };
        let strategy = BackRunningStrategy::new(
            Arc::clone(&provider),
            config,
            Arc::clone(&pool_manager),
            Arc::clone(&block_manager),
        );
        engine.add_strategy(Box::new(strategy));

        // Set up executor.
        let executor = Box::new(FastLineExecutor::new(Arc::clone(&provider)));
        let executor = ExecutorMapper::new(executor, |action| match action {
            MevActions::SubmitTxToFastLine(tx) => Some(tx),
            _ => None,
        });
        engine.add_executor(Box::new(executor));

        let executor = Box::new(MempoolExecutor::new(Arc::clone(&provider)));
        let executor = ExecutorMapper::new(executor, |action| match action {
            MevActions::SubmitTxToMempool(tx) => Some(tx),
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
