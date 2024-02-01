use std::{collections::HashMap, ops::Deref, sync::Arc};

use amm_protocol::{AmmProtocolContainer, UniswapV2Protocol};
use anyhow::Result;
use ethers_core::{
    abi::Log,
    types::{Address, Transaction},
};
use shared::{
    abi::ABI,
    amm::{AmmPool, AmmProtocol},
    network_streams::{NetworkEvent, NetworkStreamManagerBuilder, NetworkStreamsManager},
    pool::{PoolPathFinder, PoolPathItem},
    provider::{NodeProviderKind, NodeProviderManager, NormalNodeProvider},
    token::CryptoToken,
    trace::{get_trace_all_logs, TraceLogData},
};
use tokio::sync::broadcast::Receiver;

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
    abi: ABI,
    provider_manager: NodeProviderManager,
    dexes: Vec<AmmProtocolContainer>,
}

impl BackRunnerStragegy {
    pub fn new(
        abi: ABI,
        provider_manager: NodeProviderManager,
        dexes: Vec<AmmProtocolContainer>,
        max_hops: i32,
        start_tokens: Vec<&CryptoToken>,
    ) -> Self {
        let mut pools: Vec<Arc<dyn AmmPool>> = Vec::new();

        for dex in &dexes {
            match dex {
                AmmProtocolContainer::UniswapV2(v2) => pools.extend(v2.pools()),
                _ => panic!("Unsupported protocol"),
            }
        }

        let mut map: HashMap<CryptoToken, Vec<Vec<PoolPathItem>>> = HashMap::new();
        let path_finder = PoolPathFinder::new(pools);
        for ele in start_tokens {
            let input_token = Arc::new(ele.clone());
            map.insert(
                ele.clone(),
                path_finder.generate_paths(input_token.clone(), input_token, max_hops),
            );
        }

        Self {
            abi,
            provider_manager,
            dexes,
        }
    }

    pub async fn run(&self) -> Result<()> {
        let router_addresses: Vec<Address> = self
            .dexes
            .iter()
            .map(|d| match d {
                AmmProtocolContainer::UniswapV2(v2) => *v2.router(),
                _ => panic!("Unsupported protocol"),
            })
            .collect();
        let filters: Vec<String> = router_addresses.iter().map(|s: &Address| format!("{:?}", s)).collect();

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
                            UniswapV2Protocol::decode_pair_trace_logs(&self.abi.uniswap_v2_pair, trace_log),
                        );
                    }
                }
            }
        }

        Ok(())
    }
}
