use amm_protocol::AmmProtocolContainer;
use shared::{abi::ABI, network_streams::{NetworkEvent, NetworkStreamManagerBuilder, NetworkStreamsManager}, provider::NodeProviderManager};

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

    pub async fn run(&self) {
        let ns: NetworkStreamsManager = NetworkStreamManagerBuilder::new(provider_kind.clone())
            .watch_pending_transactions(Some(router_addresses.clone()))
            .build();

        let mut event_receiver: Receiver<NetworkEvent> = ns.subscribe();

        while let Ok(event) = event_receiver.recv().await {
            if let NetworkEvent::PendingTx(tx) = &event {
                if let Some(to) = tx.to {
                    if !filters.iter().any(|&f| f == to) {
                        continue;
                    }

                    let tx_hash: String = tx.hash.encode_hex();
                    let trace_logs: Vec<TraceLogData> =
                        get_trace_all_logs(self.provider_manager.get_next_debug_trace_call().debug_trace_call(tx.clone(), None).await?);

                    for trace_log in trace_logs {
                        new_pending_tx(
                            tx_hash.clone(),
                            UniswapV2Protocol::decode_pair_trace_logs(&abi.uniswap_v2_pair, trace_log),
                        );
                    }

                    println!("=============");
                }
            }
        }
    }
}
