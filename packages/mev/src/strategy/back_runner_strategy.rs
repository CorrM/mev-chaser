use std::sync::{Mutex, RwLock};
use std::time::Instant;
use std::{collections::HashMap, sync::Arc};

use anyhow::Result;
use ethers_core::types::U64;
use ethers_core::{
    abi::Log,
    types::{Address, CallFrame, CallLogFrame, Transaction, U256},
    utils::to_checksum,
};
use ethers_providers::{Middleware, PubsubClient};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use tokio::sync::broadcast::Receiver;

use amm::{
    uniswap_v2_utils::batch_update_uniswap_v2_pools, update_touched_pool_reserves, AmmPool, AmmProtocol,
    AmmProtocolKind, UniswapV2Protocol,
};
use contracts::OneSwapInfo;
use shared::{
    network_streams::{NetworkEvent, NetworkStreamManagerBuilder, NetworkStreamsManager, NewBlock},
    solidity_bridge::SolidityBridge,
    token::{CryptoToken, TokenManager},
};

use crate::pool::{generate_pool_paths, PoolPath, PoolPathsContainer};
use crate::{PriceManager, ProviderHelper};

fn submit_slippage(amount: U256) -> U256 {
    // 0.5% slippage (995/1000)
    // 5.0% slippage (95/100)
    let final_amount: U256 = (amount * 995) / 1000;
    final_amount
}

pub struct BackRunnerStrategy<M>
where
    M: Middleware,
{
    solidity_bridge: SolidityBridge<M>,
    token_manager: TokenManager,
    dexes: Vec<Arc<dyn AmmProtocol>>,
    pools: HashMap<Address, Arc<RwLock<dyn AmmPool>>>,
    price_manager: Arc<PriceManager>,
    paths_container: PoolPathsContainer,
    next_block_base_fee: U256, // TODO: Should be a service that manages gas_price
}

impl<M> BackRunnerStrategy<M>
where
    M: Middleware + Clone + 'static,
    <M as Middleware>::Provider: PubsubClient,
{
    pub async fn new(
        solidity_bridge: SolidityBridge<M>,
        provider: &Arc<M>,
        token_manager: TokenManager,
        dexes: Vec<Arc<dyn AmmProtocol>>,
        max_hops: i32,
        start_tokens: Vec<Arc<CryptoToken>>,
    ) -> Self {
        // Collect pools
        let mut pools: Vec<Arc<RwLock<dyn AmmPool>>> = Vec::new();
        for dex in &dexes {
            pools.extend(dex.pools())
        }

        // Update pools
        batch_update_uniswap_v2_pools(Arc::clone(provider), &pools).await;

        // Remove empty pools
        let mut pools_to_remove: Vec<usize> = Vec::new();
        for (idx, pool) in pools.iter().enumerate() {
            let pool_read_lock = pool.read().unwrap();

            if pool_read_lock.reserve0().is_zero() || pool_read_lock.reserve1().is_zero() {
                pools_to_remove.push(idx);
            }
        }

        for idx in pools_to_remove {
            pools.remove(idx);
        }

        for pool in &pools {
            let pool_read_lock = pool.read().unwrap();

            if pool_read_lock.reserve0().is_zero() || pool_read_lock.reserve1().is_zero() {
                panic!("Pool {} is empty", to_checksum(pool_read_lock.address(), None));
            }
        }

        // Generate paths
        let mut paths_container = PoolPathsContainer::new();
        for start_token in &start_tokens {
            let paths: Vec<PoolPath> = generate_pool_paths(&pools, start_token, start_token, max_hops);
            paths_container.add_multi_path(paths);
        }

        // => Test
        //let pool_address: Address = Address::from_str("0x2cF7252e74036d1Da831d11089D326296e64a728").unwrap();
        //let gffsdg = &paths_container.get_paths_containing_pool(&pool_address).unwrap()[0];
        //let swaps = gffsdg
        //    .make_swaps(
        //        parse_units("1", "ether").unwrap().into(),
        //        parse_units("0.5", "ether").unwrap().into(),
        //    )
        //    .unwrap();
        //let gg = solidity_bridge
        //    .estimate_get_loan_then_swap_chain(
        //        swaps.0,
        //        swaps.1,
        //        false,
        //    )
        //    .await;
        //let err = gg.unwrap_err();
        //println!("Error 0: {:?}", err);
        //println!("Error 1: {:?}", err.decode_revert::<String>());
        // => Test

        // Update pools again before starts
        batch_update_uniswap_v2_pools(Arc::clone(provider), &pools).await;

        let native_token: &Arc<CryptoToken> = token_manager.native_token();
        let price_manager = PriceManager::new(
            *native_token.address(),
            start_tokens.iter().map(|t| *t.address()).collect(),
            &pools,
        );

        Self {
            solidity_bridge,
            token_manager,
            dexes,
            pools: pools
                .into_iter()
                .map(|p: Arc<RwLock<dyn AmmPool>>| {
                    let address: Address = *p.read().unwrap().address();
                    (address, p)
                })
                .collect::<HashMap<_, _>>(),
            price_manager,
            paths_container,
            next_block_base_fee: U256::zero(),
        }
    }

    async fn on_new_pending_tx_with_sync_event(&self, tx: &Transaction, sync_log: &(Address, Log)) {
        let tx_hash: String = format!("{:?}", tx.hash);
        let (pool_address, log): &(Address, Log) = sync_log;
        println!(
            "tx_hash: {}\npool_address: {}",
            tx_hash,
            to_checksum(pool_address, None)
        );

        let Some(local_pool) = self.pools.get(pool_address) else {
            return;
        };

        // Update pool
        let ethers_core::abi::Token::Uint(reserve0) = log.params[0].value else {
            panic!("reserve0 is not uint")
        };
        let ethers_core::abi::Token::Uint(reserve1) = log.params[1].value else {
            panic!("reserve1 is not uint")
        };
        local_pool.write().unwrap().update_reserve(&reserve0, &reserve1);

        // Get paths
        let touched_paths: Option<&Vec<Arc<PoolPath>>> = self.paths_container.get_paths_containing_pool(pool_address);
        let Some(touched_paths) = touched_paths else {
            // No path with this pool
            return;
        };

        println!("touched_paths: {}", touched_paths.len());

        let native_token: &Arc<CryptoToken> = self.token_manager.native_token();
        let mut best_profit_in_native: std::sync::Mutex<i128> = std::sync::Mutex::new(0_i128);
        let mut best_path: std::sync::Mutex<Option<(&Arc<PoolPath>, U256, U256)>> = std::sync::Mutex::new(None);
        touched_paths.par_iter().for_each(|path: &Arc<PoolPath>| {
            let input_token: Arc<CryptoToken> = path.get_input_token();

            let amount_in: U256 = input_token.convert_to_amount(1_f64);
            let Some(amount_out) = path.get_amount_out_v2(amount_in) else {
                // TODO: Fix fees in this function
                return;
            };

            let _in: i128 = amount_in.as_u128() as i128;
            let _out: i128 = amount_out.as_u128() as i128;
            let spread: i128 = _out - _in;

            if spread <= 0 {
                return;
            }

            let (optimized_in, _amount_min_out, profit) = path.find_optimal_input(1000, 10);
            if optimized_in.is_zero() {
                return;
            }

            // Convert profit to native so we can get the most profitable path
            let native_token_price: f64 = self
                .price_manager
                .get_native_token_price(input_token.address())
                .unwrap();
            let profit_in_native: i128 = native_token
                .convert_to_amount(input_token.convert_to_decimal(profit) / native_token_price)
                .as_u128() as i128;

            // Lock from here to the end of the socpe so that we can check without other threads messing with it
            let mut best_profit_lock = best_profit_in_native.lock().unwrap();
            if *best_profit_lock > profit_in_native {
                return;
            }

            // amount_min_out are just input + cost of the transaction, then AMM will give use max output
            *best_path.lock().unwrap() = Some((path, optimized_in, optimized_in + profit_in_native));
            *best_profit_lock = profit_in_native;
        });

        let best_profit: i128 = *best_profit_in_native.get_mut().unwrap();
        if best_profit == 0 {
            return;
        }

        // Get gas cost
        let legacy_tx: bool = tx.transaction_type.unwrap_or(U64::from(0)) == U64::from(0);
        let gas_price: (U256, U256) = if legacy_tx {
            (tx.gas_price.unwrap(), U256::from(0))
        } else {
            (tx.max_fee_per_gas.unwrap(), tx.max_priority_fee_per_gas.unwrap())
        };
        let estimated_gas_usage: U256 = U256::from(550_000);
        let gas_cost_in_wei_native: U256 = (gas_price.0 + gas_price.1) * estimated_gas_usage;

        // get net profit
        let net_profit: i128 = best_profit - (gas_cost_in_wei_native.as_u128() as i128);
        if net_profit <= 0 {
            return;
        }

        let best_path: &Option<(&Arc<PoolPath>, U256, U256)> = best_path.get_mut().unwrap();
        let Some(best_path) = best_path else {
            return;
        };

        // Execute swap
        let swap_path: &Arc<PoolPath> = best_path.0;
        let swap_input_amount: U256 = best_path.1;
        let swap_output_amount: U256 = best_path.2;
        let input_token = swap_path.get_input_token();

        // TODO: That's only valid for stable coins
        if input_token.convert_to_amount(0.5_f64).as_u128() as i128 > net_profit {
            println!("Min profit not reached: {net_profit}");
            return;
        }

        let start = Instant::now();
        let swaps: Result<(Vec<OneSwapInfo>, bool)> = swap_path.make_swaps(swap_input_amount, swap_output_amount);
        let Ok(swaps) = swaps else {
            println!("Failed to make swap information: {:?}", swaps.unwrap_err());
            return;
        };
        println!("make_swaps_took: {}ms", start.elapsed().as_millis());

        let swaps_to_execute: Vec<OneSwapInfo> = swaps.0;
        let swaps_are_chained: bool = swaps.1;

        let start = Instant::now();

        let tx_hash = if legacy_tx {
            self.solidity_bridge
                .get_loan_then_swap_chain(swaps_to_execute, swaps_are_chained, false, tx.gas_price, None, None)
                .await
        } else {
            self.solidity_bridge
                .get_loan_then_swap_chain(
                    swaps_to_execute,
                    swaps_are_chained,
                    false,
                    None,
                    tx.max_fee_per_gas,
                    tx.max_priority_fee_per_gas,
                )
                .await
        };

        println!("input_token: {}", swap_path.get_input_token().symbol());
        println!("best_net_profit: {}", net_profit);
        println!(
            "path: {:?}\nback_running_tx_hash: ({:?}) {:?}\nmake_tx_took: {}ms",
            swap_path,
            tx.hash,
            tx_hash,
            start.elapsed().as_millis()
        );
    }

    async fn on_new_block(&mut self, provider: &Arc<M>, block: &NewBlock) {
        update_touched_pool_reserves(provider, block.block_number, &mut self.pools)
            .await
            .unwrap_or_else(|e| {
                println!("[?] Error from get_touched_pool_reserves: {:?}", e);
            });
    }

    pub async fn run<MD>(&mut self, provider: Arc<M>, debug_provider: Arc<MD>) -> Result<()>
    where
        MD: Middleware + 'static,
    {
        let router_addresses: Vec<Address> = self
            .dexes
            .iter()
            .map(|d| match d.kind() {
                AmmProtocolKind::UniswapV2 => {
                    // https://users.rust-lang.org/t/can-you-get-the-raw-pointer-of-a-pinned-arc/28276/3
                    let d: *mut UniswapV2Protocol = &**d as *const _ as *mut UniswapV2Protocol;
                    unsafe { *(*d).router() }
                }
            })
            .collect();
        let filters: Vec<String> = router_addresses
            .iter()
            .map(|s: &Address| to_checksum(s, None))
            .collect();

        let ns: NetworkStreamsManager = NetworkStreamManagerBuilder::<M>::new((*provider).clone())
            .watch_new_blocks()
            .watch_pending_transactions(Some(filters.clone()))
            .build();

        /*
        let block: Block<H256> = provider
            .get_block(BlockNumber::Latest)
            .await
            .unwrap()
            .unwrap();
        let mut new_block = NewBlock {
            block_number: block.number.unwrap(),
            base_fee: block.base_fee_per_gas.unwrap(),
            next_base_fee: calculate_next_block_base_fee(
                block.gas_used,
                block.gas_limit,
                block.base_fee_per_gas.unwrap(),
            ),
        };
        */

        let mut event_receiver: Receiver<NetworkEvent> = ns.subscribe();
        while let Ok(event) = event_receiver.recv().await {
            match event {
                NetworkEvent::Block(block) => {
                    // new_block = block;
                    self.next_block_base_fee = block.next_base_fee;

                    self.on_new_block(&provider, &block).await;
                }
                NetworkEvent::PendingTx(ref tx) => {
                    let Some(to) = tx.to else {
                        continue;
                    };

                    let is_router_address: bool = router_addresses.contains(&to);
                    if !is_router_address {
                        continue;
                    }

                    let start = Instant::now();

                    let frame: Result<Option<CallFrame>> = ProviderHelper::debug_trace_call(Arc::clone(&debug_provider), tx, None).await;
                    if frame.is_err() {
                        println!("[?] Error from debug_trace_call: {:?}", frame.unwrap_err());
                        continue;
                    }

                    let Some(frame) = frame.unwrap() else {
                        continue;
                    };

                    if frame.error.is_some() {
                        println!(
                            "[?] Error from transaction when calling debug_trace_call: {:?}",
                            frame.error
                        );
                        continue;
                    }

                    println!("debug_trace_call took: {}ms", start.elapsed().as_millis());

                    let start = Instant::now();

                    let mut trace_logs: Vec<CallLogFrame> = Vec::new();
                    ProviderHelper::extract_trace_logs(&frame, &mut trace_logs);

                    println!("extract_trace_logs took: {}ms", start.elapsed().as_millis());

                    let start = Instant::now();

                    // TODO: Use to_address to determine which dex to `decode_pair_trace_logs`
                    let mut all_sync_logs: Mutex<Vec<(Address, Log)>> = Mutex::new(Vec::new());
                    trace_logs.par_iter().for_each(|trace_log| {
                        let logs: Option<(Address, Log)> = UniswapV2Protocol::decode_pair_trace_log("Sync", trace_log);
                        let Some(logs) = logs else {
                            return;
                        };

                        all_sync_logs.lock().unwrap().push(logs);
                    });
                    let all_sync_logs: &mut Vec<(Address, Log)> = all_sync_logs.get_mut().unwrap();

                    async_scoped::TokioScope::scope_and_block(|s| {
                        for logs in all_sync_logs {
                            s.spawn(self.on_new_pending_tx_with_sync_event(tx, logs));
                        }
                    });

                    println!(
                        "Back running took: {}ms, trace_logs: {}",
                        start.elapsed().as_millis(),
                        trace_logs.len()
                    );
                    println!("============");
                }
                NetworkEvent::Log(_) => {}
            }
        }

        Ok(())
    }
}
