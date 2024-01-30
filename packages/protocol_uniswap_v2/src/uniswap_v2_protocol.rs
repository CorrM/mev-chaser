use ethers_core::abi::{Abi, Log, RawLog};

use shared::{
    amm::{AmmPool, AmmProtocol},
    trace::TraceLogData,
};

use crate::uniswap_v2_pool::UniswapV2Pool;

pub struct UniswapV2Protocol {
    name: String,
    fees: u32,
    pools: Vec<UniswapV2Pool>,
}

impl UniswapV2Protocol {
    pub fn new(name: impl Into<String>, fees: u32) -> Self {
        Self {
            name: name.into(),
            fees,
            pools: Vec::new(),
        }
    }

    pub fn decode_trace_pair_logs(pair_abi: &Abi, trace_log: TraceLogData) -> Vec<(String, Log)> {
        let mut ret: Vec<(String, Log)> = Vec::new();

        for ev in pair_abi.events() {
            if ev.signature() != trace_log.topics()[0] {
                continue;
            }

            let log_result: Result<Log, ethers_core::abi::Error> = ev.parse_log(RawLog {
                topics: trace_log.topics(),
                data: trace_log.data().to_vec(),
            });

            if let Ok(log) = log_result {
                ret.push((ev.name.clone(), log));
            }
        }

        ret
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
