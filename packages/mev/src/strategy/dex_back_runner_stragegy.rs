use std::{ops::Deref, sync::Arc};

use amm_protocol::{AmmProtocolContainer, UniswapV2Protocol};
use anyhow::Result;
use ethers_core::{
    abi::Log,
    types::{Address, Transaction},
};
use shared::{
    abi::ABI,
    network_streams::{NetworkEvent, NetworkStreamManagerBuilder, NetworkStreamsManager},
    provider::{NodeProviderKind, NodeProviderManager, NormalNodeProvider},
    trace::{get_trace_all_logs, TraceLogData},
};
use tokio::sync::broadcast::Receiver;

fn on_new_pending_tx(tx: &Transaction, decoded_log: Vec<(String, Log)>) {
    let tx_hash: String = format!("{:?}", tx.hash);
    println!("tx_hash: {}", tx_hash);
    println!("decoded_log: {:#?}", decoded_log);

    let sync_log: Option<&(String, Log)> = decoded_log.iter().find(|(name, _)| name == "Sync");
    if sync_log.is_none() {
        return;
    }

    let (name, log): &(String, Log) = sync_log.unwrap();
    println!("sync_log: {:#?}", log);
}

pub struct DexBackRunnerStragegy {
    abi: ABI,
    provider_manager: NodeProviderManager,
    dexes: Vec<AmmProtocolContainer>,
}

impl DexBackRunnerStragegy {
    pub fn new(abi: ABI, provider_manager: NodeProviderManager, dexes: Vec<AmmProtocolContainer>) -> Self {
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
                    if !router_addresses.iter().any(|&f| f == to) {
                        continue;
                    }

                    let trace_logs: Vec<TraceLogData> = get_trace_all_logs(
                        self.provider_manager
                            .get_next_debug_trace_call()
                            .debug_trace_call(tx, None)
                            .await?,
                    );

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
