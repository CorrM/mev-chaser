use amm::{AmmPool, AmmProtocol, AmmProtocolKind, UniswapV2Protocol};
use anyhow::Result;
use ethers_core::{
    abi::Log,
    types::{Address, Transaction},
    utils::to_checksum,
};
use shared::{
    network_streams::{NetworkEvent, NetworkStreamManagerBuilder, NetworkStreamsManager},
    provider::{NodeProviderKind, NodeProviderManager, NormalNodeProvider},
    token::CryptoToken,
    trace::{get_trace_all_logs, TraceLogData},
};
use std::any::Any;
use std::{collections::HashMap, ops::Deref, sync::Arc};
use tokio::sync::{broadcast::Receiver, Mutex};

use crate::pool::{generate_pool_paths, PoolPath, PoolPathsContainer};

pub struct BackRunnerStragegy {
    provider_manager: NodeProviderManager,
    dexes: Vec<Arc<dyn AmmProtocol>>,
    pools: HashMap<Address, Arc<dyn AmmPool>>,
    paths_container: PoolPathsContainer,
}

impl BackRunnerStragegy {
    pub fn new(
        provider_manager: NodeProviderManager,
        dexes: Vec<Arc<dyn AmmProtocol>>,
        max_hops: i32,
        start_tokens: Vec<Arc<CryptoToken>>,
    ) -> Self {
        /*
        let mut pools: Vec<Arc<Mutex<dyn AmmPool>>> = Vec::new();

        for dex in &dexes {
            pools.extend(dex.pools().iter().map(|p| Mutex::new(*p)).collect());
        }
        */

        let mut pools: Vec<Arc<dyn AmmPool>> = Vec::new();

        for dex in &dexes {
            pools.extend(dex.pools())
        }

        let mut paths_container = PoolPathsContainer::new();
        for start_token in start_tokens {
            let paths: Vec<PoolPath> = generate_pool_paths(&pools, &start_token, &start_token, max_hops);
            paths_container.add_multi_path(paths);
        }

        Self {
            provider_manager,
            dexes,
            pools: pools.iter().map(|&p| (*p.address(), p)).collect::<HashMap<_, _>>(),
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
        println!("address: {}", to_checksum(pool_address, None));
        println!("sync_log: {:#?}", log);

        // Update pool
        let ethers_core::abi::Token::Uint(reserve0) = log.params[0].value else {
            panic!("reserve0 is not uint")
        };
        let ethers_core::abi::Token::Uint(reserve1) = log.params[1].value else {
            panic!("reserve1 is not uint")
        };

        self.pools.get(pool_address).unwrap().update_reserve(reserve0, reserve1);

        // Get paths
        let paths: &Vec<Arc<PoolPath>> = self.paths_container.get_paths_containing_pool(pool_address).unwrap();
        println!("paths: {:#?}", paths);
    }

    pub async fn run(&self) -> Result<()> {
        let router_addresses: Vec<Address> = self
            .dexes
            .iter()
            .map(|d| match d.kind() {
                AmmProtocolKind::UniswapV2 => *((d as &dyn Any).downcast_ref::<UniswapV2Protocol>().unwrap().router()),
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

            let to_address: Option<&Address> = router_addresses.iter().find(|&&f| f == to);
            if to_address.is_none() {
                continue;
            }

            let trace_logs: Vec<TraceLogData> = get_trace_all_logs(
                self.provider_manager
                    .get_next_debug_trace_call()
                    .debug_trace_call(tx, None)
                    .await?,
            );

            // TODO: Use to_address to determine which dex to `decode_pair_trace_logs`
            for trace_log in trace_logs {
                self.on_new_pending_tx(tx, &UniswapV2Protocol::decode_pair_trace_logs(&trace_log)).await;
            }
        }

        Ok(())
    }
}
