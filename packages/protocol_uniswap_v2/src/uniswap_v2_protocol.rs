use crate::uniswap_v2_pool::UniswapV2Pool;
use ethers_core::abi::{Abi, Event, EventExt, Log, RawLog};
use shared::{
    amm::{AmmPool, AmmProtocol},
    trace::TraceLogData,
};

pub struct UniswapV2Protocol {
    name: String,
    fees: u32,
    pools: Vec<UniswapV2Pool>,
}

impl UniswapV2Protocol {
    pub fn decode_trace(pair_abi: &Abi, trace: TraceLogData) {
        let ev: Result<&Event, ethers_core::abi::Error> = pair_abi.event("Sync");
        if let Ok(ev) = ev {
            if ev.signature() != trace.topics()[0] {
                return;
            }
            
            let log: Log = ev
                .parse_log(RawLog {
                    topics: trace.topics(),
                    data: trace.data().to_vec(),
                })
                .unwrap();

            // Logs works YAAAYY
            println!("Sig: {}", ev.abi_signature());
            println!("Log: {:#?}", log);
        }
    }

    pub fn new(name: impl Into<String>, fees: u32) -> Self {
        Self {
            name: name.into(),
            fees,
            pools: Vec::new(),
        }
    }
}

impl AmmProtocol for UniswapV2Protocol {
    fn name(&self) -> &str {
        &self.name
    }

    fn pools(&self) -> &Vec<impl AmmPool> {
        &self.pools
    }
}
