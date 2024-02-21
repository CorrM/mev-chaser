use std::str::FromStr;
use std::sync::RwLock;
use std::time::Instant;
use std::{collections::HashMap, sync::Arc};

use anyhow::Result;
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
    token::{CryptoToken, TokenManager},
};

use crate::pool::{generate_pool_paths, PoolPath, PoolPathsContainer};
use crate::{PriceManager, ProviderHelper, SolidityBridge};

//fn submit_slippage(amount: U256) -> U256 {
//    // 0.5% slippage (995/1000)
//    // 5.0% slippage (95/100)
//    let final_amount: U256 = (amount * 10) / 100;
//    final_amount
//}

fn submit_slippage(amount: i128) -> i128 {
    // 0.5% slippage (995/1000)
    // 5.0% slippage (95/100)
    (amount * 10) / 100
}

struct BackrunningSwapInfo {
    swaps_to_execute: Vec<OneSwapInfo>,
    swaps_are_chained: bool,
    swaps_are_legacy: bool,
    profit_in_native: i128,
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

            // TODO: Maybe check if the reservers are worth 1000USDT

            if pool_read_lock.reserve0().is_zero() || pool_read_lock.reserve1().is_zero() {
                pools_to_remove.push(idx);
            }
        }

        pools_to_remove.reverse();

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

    fn get_backrunning_swap(&self, tx: &Transaction, pool_address: &Address) -> Option<BackrunningSwapInfo> {
        // Get paths
        let touched_paths: Option<&Vec<Arc<PoolPath>>> = self.paths_container.get_paths_containing_pool(pool_address);
        let Some(touched_paths) = touched_paths else {
            // No path with this pool
            return None;
        };

        let start = Instant::now();
        let touched_path_time = Instant::now();

        let native_token: &Arc<CryptoToken> = self.token_manager.native_token();
        let best_swap: Option<(&Arc<PoolPath>, U256, i128)> = touched_paths
            .par_iter()
            .filter_map(|path: &Arc<PoolPath>| {
                let input_token: &CryptoToken = path.get_input_token();

                let amount_in: U256 = input_token.one_token_amount();
                let Some(amount_out) = path.get_amount_out_v2(amount_in) else {
                    // TODO: Fix fees in this function
                    return None;
                };

                // TODO: Do benchmark to check if I128 is faster than U256
                let _in: i128 = amount_in.as_u128() as i128;
                let _out: i128 = amount_out.as_u128() as i128;
                let spread: i128 = _out - _in;
                if spread <= 0 {
                    return None;
                }

                let (optimized_in, profit) = path.find_optimal_input(1000, 10);
                if optimized_in.is_zero() {
                    return None;
                }

                // Convert profit to native so we can get the most profitable path
                let native_token_price: f64 = self
                    .price_manager
                    .get_native_token_price(input_token.address())
                    .unwrap();
                let profit_in_native: i128 = native_token
                    .convert_to_amount(input_token.convert_to_decimal(profit) / native_token_price)
                    .as_u128() as i128;

                Some((path, optimized_in, profit_in_native))
            })
            .max_by_key(|(_, _, profit)| *profit);

        println!("touched_paths_time: {}ms", touched_path_time.elapsed().as_millis());

        let Some(best_swap) = best_swap else {
            println!("touched_paths: {}", touched_paths.len());
            return None;
        };

        // Get gas cost
        let estimated_gas_usage: U256 = U256::from(550_000);
        let legacy_tx: bool = tx.transaction_type.is_none();
        let gas_cost_in_wei_native: U256 = match tx.transaction_type.map(|t| t.as_u64()) {
            None | Some(1) => tx.gas_price.unwrap() * estimated_gas_usage,
            Some(2) => (tx.max_fee_per_gas.unwrap() + tx.max_priority_fee_per_gas.unwrap()) * estimated_gas_usage,
            _ => return None,
        };
        let gas_cost_in_wei_native: i128 = gas_cost_in_wei_native.as_u128() as i128;

        // get net profit
        let net_profit_in_native: i128 = best_swap.2 - gas_cost_in_wei_native;
        if net_profit_in_native <= 0 {
            return None;
        }

        // Execute swap
        let swap_path: &Arc<PoolPath> = best_swap.0;
        let swap_input_amount: U256 = best_swap.1;
        let swap_output_amount: U256 = swap_input_amount + gas_cost_in_wei_native; // amount_min_out AMM will give use max output
        let swap_input_token: &CryptoToken = swap_path.get_input_token();

        // TODO: That's only valid for stable coins
        if swap_input_token.convert_to_amount(0.5_f64).as_u128() as i128 > net_profit_in_native {
            println!("Min profit not reached: {}", net_profit_in_native);
            return None;
        }

        let swaps: Result<(Vec<OneSwapInfo>, bool)> = swap_path.make_swaps(swap_input_amount, swap_output_amount);
        let Ok(swaps) = swaps else {
            println!("Failed to make swap information: {:?}", swaps.unwrap_err());
            return None;
        };

        println!("processing: {}ms", start.elapsed().as_millis());
        println!("touched_paths: {}", touched_paths.len());
        Some(BackrunningSwapInfo {
            swaps_to_execute: swaps.0,
            swaps_are_chained: swaps.1,
            swaps_are_legacy: legacy_tx,
            profit_in_native: net_profit_in_native,
        })
    }

    async fn on_new_block(&mut self, provider: &Arc<M>, block: &NewBlock) {
        update_touched_pool_reserves(provider, block.block_number, &mut self.pools)
            .await
            .unwrap_or_else(|e| {
                println!("[?] Error from get_touched_pool_reserves: {:?}", e);
            });
    }

    async fn on_pending_tx(&mut self, tx: &Transaction, debug_provider: &Arc<M>) {
        let start = Instant::now();

        {
            let swaps: Vec<OneSwapInfo> = vec![
                PoolPath::make_uniswap_v2_protocol_swap_info(
                    Address::from_str("0xa5E0829CaCEd8fFDD4De3c43696c57F7D7A678ff").unwrap(),
                    vec![
                        Address::from_str("0xc2132D05D31c914a87C6611C10748AEb04B58e8F").unwrap(),
                        Address::from_str("0x346404079b3792a6c548B072B9C4DDdFb92948d5").unwrap(),
                    ],
                    10000000,
                    0,
                )
                .unwrap(),
                PoolPath::make_uniswap_v2_protocol_swap_info(
                    Address::from_str("0xa5E0829CaCEd8fFDD4De3c43696c57F7D7A678ff").unwrap(),
                    vec![
                        Address::from_str("0x346404079b3792a6c548B072B9C4DDdFb92948d5").unwrap(),
                        Address::from_str("0xc2132D05D31c914a87C6611C10748AEb04B58e8F").unwrap(),
                    ],
                    0,
                    1000000,
                )
                .unwrap(),
            ];

            let result = self
                .solidity_bridge
                .get_loan_then_swap_chain_bundle(tx, U256::from(100_000), swaps, false, false, tx.gas_price, None, None)
                .await;

            println!("get_loan_then_swap_chain_bundle: {:?}", result);
        }

        // TODO: No need for this as NetworkStreamsManager filters pending transactions
        //let Some(to) = tx.to else {
        //    continue;
        //};
        //let is_router_address: bool = router_addresses.contains(&to);
        //if !is_router_address {
        //    continue;
        //}

        let frame: Result<Option<CallFrame>> =
            ProviderHelper::debug_trace_call(Arc::clone(debug_provider), tx, None).await;
        if frame.is_err() {
            println!("[?] Error from debug_trace_call: {:?}", frame.unwrap_err());
            return;
        }

        let Some(frame) = frame.unwrap() else {
            return;
        };

        if frame.error.is_some() {
            println!(
                "[?] Error from transaction when calling debug_trace_call: {:?}",
                frame.error
            );
            return;
        }

        let mut trace_logs: Vec<CallLogFrame> = Vec::new();
        ProviderHelper::extract_trace_logs(&frame, &mut trace_logs);

        // Update and get touched pools
        let thouched_pools: Vec<Address> = trace_logs
            .par_iter()
            .filter_map(|trace_log| {
                let sync_log: Option<(Address, Log)> = UniswapV2Protocol::decode_pair_trace_log("Sync", trace_log);
                let Some(sync_log) = sync_log else {
                    return None;
                };

                let (pool_address, log): &(Address, Log) = &sync_log;
                let Some(local_pool) = self.pools.get(pool_address) else {
                    return None;
                };

                // Update pool
                let ethers_core::abi::Token::Uint(reserve0) = log.params[0].value else {
                    panic!("reserve0 is not uint")
                };
                let ethers_core::abi::Token::Uint(reserve1) = log.params[1].value else {
                    panic!("reserve1 is not uint")
                };
                local_pool.write().unwrap().update_reserve(&reserve0, &reserve1);

                Some(*pool_address)
            })
            .collect();

        // Get most profitable swap
        let most_proftable_swap: Option<BackrunningSwapInfo> = thouched_pools
            .par_iter()
            .filter_map(|touched_pool| self.get_backrunning_swap(tx, touched_pool))
            .max_by_key(|s| s.profit_in_native);

        let Some(most_proftable_swap) = most_proftable_swap else {
            println!(
                "⏳ Back running took: {}ms, trace_logs: {}",
                start.elapsed().as_millis(),
                trace_logs.len()
            );
            return;
        };

        // Execute swap
        let bid_amount: U256 =
            U256::from(most_proftable_swap.profit_in_native - submit_slippage(most_proftable_swap.profit_in_native));
        let tx_hash = if most_proftable_swap.swaps_are_legacy {
            self.solidity_bridge
                .get_loan_then_swap_chain_bundle(
                    tx,
                    bid_amount,
                    most_proftable_swap.swaps_to_execute,
                    most_proftable_swap.swaps_are_chained,
                    false,
                    tx.gas_price,
                    None,
                    None,
                )
                .await
        } else {
            self.solidity_bridge
                .get_loan_then_swap_chain_bundle(
                    tx,
                    bid_amount,
                    most_proftable_swap.swaps_to_execute,
                    most_proftable_swap.swaps_are_chained,
                    false,
                    None,
                    tx.max_fee_per_gas,
                    tx.max_priority_fee_per_gas,
                )
                .await
        };

        println!(
            "⌚ Back running took: {}ms, trace_logs: {}",
            start.elapsed().as_millis(),
            trace_logs.len()
        );

        println!("back_running_tx_hash: ({:?}) {:?}", tx.hash, tx_hash);
        println!("============");
    }

    pub async fn run(&mut self, provider: Arc<M>, debug_provider: Arc<M>) -> Result<()> {
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
                    self.on_pending_tx(tx, &debug_provider).await;
                }
                NetworkEvent::Log(_) => {}
            }
        }

        Ok(())
    }
}
