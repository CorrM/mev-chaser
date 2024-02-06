use amm::{
    uniswap_v2_utils::batch_update_uniswap_v2_pools, update_touched_pool_reserves, AmmPool, AmmProtocol,
    AmmProtocolKind, UniswapV2Protocol,
};
use anyhow::Result;
use contracts::OneSwapInfo;
use ethers_core::{
    abi::Log,
    types::{Address, Block, BlockNumber, CallFrame, CallLogFrame, Transaction, H256, U256},
    utils::to_checksum,
};
use ethers_providers::Middleware;
use shared::{
    network_streams::{NetworkEvent, NetworkStreamManagerBuilder, NetworkStreamsManager, NewBlock},
    provider::{DebugTraceCallNodeProvider, NodeProvider, NodeProviderKind, NodeProviderManager, NormalNodeProvider},
    solidity_bridge::SolidityBridge,
    token::{CryptoToken, TokenManager},
    utils::calculate_next_block_base_fee,
};
use std::sync::RwLock;
use std::{collections::HashMap, ops::Deref, sync::Arc};
use tokio::sync::broadcast::Receiver;

use crate::pool::{generate_pool_paths, PoolPath, PoolPathItem, PoolPathsContainer};

pub struct BackRunnerStrategy {
    solidity_bridge: SolidityBridge,
    token_manager: TokenManager,
    provider_manager: NodeProviderManager,
    dexes: Vec<Arc<dyn AmmProtocol>>,
    pools: HashMap<Address, Arc<RwLock<dyn AmmPool>>>,
    paths_container: PoolPathsContainer,
    next_block_base_fee: U256, // TODO: Should be a service that manages gas_price
}

impl BackRunnerStrategy {
    pub async fn new(
        solidity_bridge: SolidityBridge,
        token_manager: TokenManager,
        provider_manager: NodeProviderManager,
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
        batch_update_uniswap_v2_pools(provider_manager.get_next(), &pools).await;

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
        for start_token in start_tokens {
            let paths: Vec<PoolPath> = generate_pool_paths(&pools, &start_token, &start_token, max_hops);
            paths_container.add_multi_path(paths);
        }

        // No path test
        //let pool_address: Address = Address::from_str("0xDF9549017071B88a5B9a7252f875632bfdbb7fc7").unwrap();
        //paths_container.get_paths_containing_pool(&pool_address).unwrap();

        Self {
            solidity_bridge,
            token_manager,
            provider_manager,
            dexes,
            pools: pools
                .into_iter()
                .map(|p: Arc<RwLock<dyn AmmPool>>| {
                    let address: Address = *p.read().unwrap().address();
                    (address, p)
                })
                .collect::<HashMap<_, _>>(),
            paths_container,
            next_block_base_fee: U256::zero(),
        }
    }

    async fn on_new_pending_tx(&self, tx: &Transaction, decoded_log: &HashMap<String, (Address, Log)>) {
        let sync_log: Option<&(Address, Log)> = decoded_log.get("Sync");
        if sync_log.is_none() {
            return;
        }

        let tx_hash: String = format!("{:?}", tx.hash);
        println!("tx_hash: {}", tx_hash);

        let (pool_address, log): &(Address, Log) = sync_log.unwrap();
        println!("pool_address: {}", to_checksum(pool_address, None));

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
        local_pool.write().unwrap().update_reserve(reserve0, reserve1);

        // Get paths
        let touched_paths: Option<&Vec<Arc<PoolPath>>> = self.paths_container.get_paths_containing_pool(pool_address);
        let Some(touched_paths) = touched_paths else {
            // No path with this pool
            return;
        };

        // Get spreads
        let mut spreads: HashMap<usize, i128> = HashMap::new();
        for (idx, path) in touched_paths.iter().enumerate() {
            let amount_in: U256 = path.get_input_token().convert_to_amount(1_f64);
            let amount_out: Option<U256> = path.get_amount_out_v2(amount_in); // TODO: Fix fees in this function

            let Some(amount_out) = amount_out else { continue };
            let _in: i128 = amount_in.as_u128() as i128;
            let _out: i128 = amount_out.as_u128() as i128;
            let spread: i128 = _out - _in;

            if spread > 0 {
                spreads.insert(idx, spread);
            }
        }

        if spreads.is_empty() {
            return;
        }

        // Get gas cost
        let base_fee: U256 = self.next_block_base_fee;
        let estimated_gas_usage: U256 = U256::from(550000);
        let gas_cost_in_wei_native: U256 = base_fee * estimated_gas_usage;

        // Sort by spread
        let mut sorted_spreads: Vec<_> = spreads.iter().collect();
        sorted_spreads.sort_by_key(|x| x.1);
        sorted_spreads.reverse();

        // Get most profitable path
        let mut best_path: Option<(&Arc<PoolPath>, U256, U256)> = None;
        for spread in sorted_spreads {
            let path_idx: &usize = spread.0;
            let path: &Arc<PoolPath> = &touched_paths[*path_idx];
            let (optimized_in, amount_min_out, profit) = path.optimize_amount_in(1000, 10);

            println!("path: {:?}", path.path());
            println!("optimized_in: {optimized_in:?}");
            if optimized_in.is_zero() {
                continue;
            }

            let excess_profit: i128 = (profit.as_u128() as i128) - (gas_cost_in_wei_native.as_u128() as i128);
            println!("profit: {profit}");
            println!("cost: {gas_cost_in_wei_native}");
            println!("net_profit: {excess_profit}");

            if excess_profit <= 0 {
                continue;
            }

            // Check amount_min_out
            if best_path.is_some_and(|x| x.2 >= amount_min_out) {
                continue;
            }

            best_path = Some((path, optimized_in, amount_min_out));
        }

        let Some(best_path) = best_path else {
            return;
        };

        // Execute swap
        let swap_path: &Arc<PoolPath> = best_path.0;
        let swap_input_amount: U256 = best_path.1;
        let swap_output_amount: U256 = best_path.2;

        let swaps: Result<(Vec<OneSwapInfo>, bool)> = swap_path.make_swaps(swap_input_amount, swap_output_amount);
        let Ok(swaps) = swaps else {
            println!("Failed to make swap information: {:?}", swaps.unwrap_err());
            return;
        };

        let swaps_to_execute: Vec<OneSwapInfo> = swaps.0;
        let swaps_are_chained: bool = swaps.1;

        println!(
            "swaps_are_chained: {}, swaps: {:?}",
            swaps_are_chained, swaps_to_execute
        );

        //let tx_hash: Result<H256> = self
        //    .solidity_bridge
        //    .get_loan_then_swap_chain(swaps.0, swaps.1, tx.gas_price)
        //    .await;
    }

    async fn on_new_block(&mut self, block: &NewBlock) {
        let provider = self.provider_manager.get_next().raw_ws_provider();
        update_touched_pool_reserves(provider, block.block_number, &mut self.pools)
            .await
            .unwrap_or_else(|e| {
                println!("[?] Error from get_touched_pool_reserves: {:?}", e);
            });
    }

    pub async fn run(&mut self) -> Result<()> {
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

        let provider: &Arc<NormalNodeProvider> = self.provider_manager.get_next();
        let provider_kind: &Arc<NodeProviderKind> = &Arc::new(NodeProviderKind::Normal(provider.deref().clone()));

        let ns: NetworkStreamsManager = NetworkStreamManagerBuilder::new(provider_kind.clone())
            .watch_new_blocks()
            .watch_pending_transactions(Some(filters.clone()))
            .build();

        let block: Block<H256> = provider
            .raw_ws_provider()
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

        let mut event_receiver: Receiver<NetworkEvent> = ns.subscribe();
        while let Ok(event) = event_receiver.recv().await {
            if let NetworkEvent::PendingTx(tx) = &event {
                let Some(to) = tx.to else { continue };

                let is_router_address: bool = router_addresses.iter().any(|&f| f == to);
                if !is_router_address {
                    continue;
                }

                let debug_provider: &Arc<DebugTraceCallNodeProvider> =
                    self.provider_manager.get_next_debug_trace_call();
                let frame: Result<Option<CallFrame>> =
                    debug_provider.debug_trace_call(tx, new_block.block_number).await;
                if frame.is_err() {
                    println!("[?] Error from debug_trace_call: {:?}", frame.unwrap_err());
                    continue;
                }

                let Some(frame) = frame.unwrap() else {
                    continue;
                };

                let mut trace_logs: Vec<CallLogFrame> = Vec::new();
                DebugTraceCallNodeProvider::extract_trace_logs(&frame, &mut trace_logs);

                // TODO: Use to_address to determine which dex to `decode_pair_trace_logs`
                for trace_log in trace_logs {
                    let logs: Option<HashMap<String, (Address, Log)>> =
                        UniswapV2Protocol::decode_pair_trace_logs(&trace_log);
                    let Some(logs) = logs else { continue };

                    self.on_new_pending_tx(tx, &logs).await;
                }
            } else if let NetworkEvent::Block(block) = event {
                new_block = block;
                self.next_block_base_fee = new_block.next_base_fee;

                self.on_new_block(&new_block).await;
            }
        }

        Ok(())
    }
}
