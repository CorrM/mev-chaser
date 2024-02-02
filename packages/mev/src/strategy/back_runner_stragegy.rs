use std::{collections::HashMap, ops::Deref, sync::Arc};

use amm::{AmmProtocol, UniswapV2Protocol};
use amm::{AmmPoolKind, AmmProtocolKind};
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

use crate::pool::{PoolPathFinder, PoolPathItem};

fn on_new_pending_tx(tx: &Transaction, decoded_log: Vec<(String, Log)>) {
    let sync_log: Option<&(String, Log)> = decoded_log.iter().find(|(name, _)| name == "Sync");
    if sync_log.is_none() {
        return;
    }

    let tx_hash: String = format!("{:?}", tx.hash);
    println!("tx_hash: {}", tx_hash);

    let (_, log): &(String, Log) = sync_log.unwrap();
    println!("sync_log: {:#?}", log);
}

pub struct BackRunnerStragegy {
    provider_manager: NodeProviderManager,
    dexes: Vec<AmmProtocolKind>,
}

impl BackRunnerStragegy {
    pub fn new(
        provider_manager: NodeProviderManager,
        dexes: Vec<AmmProtocolKind>,
        max_hops: i32,
        start_tokens: Vec<&CryptoToken>,
    ) -> Self {
        let mut pools: Vec<AmmPoolKind> = Vec::new();

        for dex in &dexes {
            match dex {
                AmmProtocolKind::UniswapV2(v2) => {
                    pools.extend(v2.pools().iter().map(|p| AmmPoolKind::UniswapV2(p.deref().clone())))
                }
            }
        }

        let mut map: HashMap<CryptoToken, Vec<Vec<PoolPathItem>>> = HashMap::new();
        for ele in start_tokens {
            let input_token = Arc::new(ele.clone());
            map.insert(
                ele.clone(),
                PoolPathFinder::generate_paths(&pools, input_token.clone(), input_token, max_hops),
            );
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
            .map(|d| match d {
                AmmProtocolKind::UniswapV2(v2) => *v2.router(),
            })
            .collect();
        let filters: Vec<String> = router_addresses.iter().map(|s: &Address| to_checksum(s, None)).collect();

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
                        on_new_pending_tx(
                            tx,
                            UniswapV2Protocol::decode_pair_trace_logs(trace_log),
                        );
                    }
                }
            }
        }

        Ok(())
    }
}
