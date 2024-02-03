use std::any::Any;
use std::{collections::HashMap, ops::Deref, sync::Arc};

use amm::{AmmPool, AmmPoolKind, AmmProtocolKind};
use amm::{AmmProtocol, UniswapV2Protocol};
use anyhow::Result;
use ethers_core::utils::to_checksum;
use ethers_core::{
    abi::Log,
    types::{Address, Transaction},
};
use shared::{
    network_streams::{NetworkEvent, NetworkStreamManagerBuilder, NetworkStreamsManager},
    provider::{NodeProviderKind, NodeProviderManager, NormalNodeProvider},
    token::CryptoToken,
    trace::{get_trace_all_logs, TraceLogData},
};
use tokio::sync::broadcast::Receiver;

use crate::pool::{generate_pool_paths, PoolPath, PoolPathItem};

fn on_new_pending_tx(tx: &Transaction, decoded_log: &HashMap<String, (Address, Log)>) {
    let sync_log: Option<&(Address, Log)> = decoded_log.get("Sync");
    if sync_log.is_none() {
        return;
    }

    let tx_hash: String = format!("{:?}", tx.hash);
    println!("tx_hash: {}", tx_hash);

    let (address, log): &(Address, Log) = sync_log.unwrap();
    println!("address: {}", to_checksum(address, None));
    println!("sync_log: {:#?}", log);
}

pub struct BackRunnerStragegy {
    provider_manager: NodeProviderManager,
    dexes: Vec<Arc<dyn AmmProtocol>>,
}

impl BackRunnerStragegy {
    pub fn new(
        provider_manager: NodeProviderManager,
        dexes: Vec<Arc<dyn AmmProtocol>>,
        max_hops: i32,
        start_tokens: Vec<Arc<CryptoToken>>,
    ) -> Self {
        let mut pools: Vec<Arc<dyn AmmPool>> = Vec::new();

        for dex in &dexes {
            pools.extend(dex.pools())
        }

        let mut map: HashMap<Arc<CryptoToken>, Vec<PoolPath>> = HashMap::new();
        for start_token in start_tokens {
            let paths: Vec<PoolPath> = generate_pool_paths(&pools, &start_token, &start_token, max_hops);
            map.insert(start_token, paths);
        }

        Self {
            provider_manager,
            dexes,
        }
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
            if let NetworkEvent::PendingTx(tx) = &event {
                if let Some(to) = tx.to {
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
                        on_new_pending_tx(tx, &UniswapV2Protocol::decode_pair_trace_logs(&trace_log));
                    }
                }
            }
        }

        Ok(())
    }
}
