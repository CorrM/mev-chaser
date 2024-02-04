use amm::{uniswap_v2_utils::batch_update_uniswap_v2_pools, AmmPool, AmmProtocol, AmmProtocolKind, UniswapV2Protocol};
use anyhow::Result;
use ethers_core::{
    abi::Log,
    types::{Address, GethTrace, Transaction, U256},
    utils::to_checksum,
};
use shared::{
    network_streams::{NetworkEvent, NetworkStreamManagerBuilder, NetworkStreamsManager},
    provider::{NodeProviderKind, NodeProviderManager, NormalNodeProvider},
    token::CryptoToken,
    trace::{get_trace_all_logs, TraceLogData},
};
use std::sync::RwLock;
use std::{collections::HashMap, ops::Deref, sync::Arc};
use tokio::sync::broadcast::Receiver;

use crate::pool::{generate_pool_paths, PoolPath, PoolPathsContainer};

pub struct BackRunnerStrategy {
    provider_manager: NodeProviderManager,
    dexes: Vec<Arc<dyn AmmProtocol>>,
    pools: HashMap<Address, Arc<RwLock<dyn AmmPool>>>,
    paths_container: PoolPathsContainer,
}

impl BackRunnerStrategy {
    pub async fn new(
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

        let mut paths_container = PoolPathsContainer::new();
        for start_token in start_tokens {
            let paths: Vec<PoolPath> = generate_pool_paths(&pools, &start_token, &start_token, max_hops);
            paths_container.add_multi_path(paths);
        }

        Self {
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

        // TODO: MAYBE add tokio::spawn for paths
        // Get paths
        let paths: &Vec<Arc<PoolPath>> = self.paths_container.get_paths_containing_pool(pool_address).unwrap();

        // Get spreads
        for path in paths {
            let one_token_in: U256 = U256::from(1);
            let simulated: Option<U256> = path.simulate_v2_path(one_token_in);

            match simulated {
                Some(price_quote) => {
                    let one_usdc_in = one_token_in * U256::from(6); // usdc_decimals
                    let _out = price_quote.as_u128() as i128;
                    let _in = one_usdc_in.as_u128() as i128;
                    let spread = _out - _in;

                    if spread > 0 {
                        println!("spread: {}", spread);
                        //spreads.insert(idx, spread);
                    }
                }
                None => {}
            }
        }
    }

    pub async fn run(&self) -> Result<()> {
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
            .watch_pending_transactions(Some(filters.clone()))
            .build();

        let mut event_receiver: Receiver<NetworkEvent> = ns.subscribe();
        while let Ok(event) = event_receiver.recv().await {
            let NetworkEvent::PendingTx(tx) = &event else { continue };
            let Some(to) = tx.to else { continue };

            let is_router_address: bool = router_addresses.iter().any(|&f| f == to);
            if !is_router_address {
                continue;
            }

            let frame: GethTrace = self
                .provider_manager
                .get_next_debug_trace_call()
                .debug_trace_call(tx, None)
                .await?;
            let trace_logs: Vec<TraceLogData> = get_trace_all_logs(frame);

            // TODO: Use to_address to determine which dex to `decode_pair_trace_logs`
            for trace_log in trace_logs {
                let logs: HashMap<String, (Address, Log)> = UniswapV2Protocol::decode_pair_trace_logs(&trace_log);
                self.on_new_pending_tx(tx, &logs).await;
            }
        }

        Ok(())
    }
}
