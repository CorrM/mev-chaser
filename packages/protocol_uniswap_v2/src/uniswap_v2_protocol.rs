use anyhow::Result;
use std::str::FromStr;
use ethers_core::abi::{Abi, Log, RawLog};
use std::sync::Arc;
use ethers_core::types::H256;

use shared::{
    amm::{AmmPool, AmmProtocol},
    trace::TraceLogData,
};

use crate::uniswap_v2_pool::UniswapV2Pool;

pub struct UniswapV2Protocol {
    name: String,
    fees: u32,
    pools: Vec<Arc<UniswapV2Pool>>,
    factory: H256,
    router: H256,
}

impl UniswapV2Protocol {
    pub fn new(
        name: impl Into<String>,
        fees: u32,
        factory: impl Into<String>,
        router: impl Into<String>,
    ) -> Result<Self> {
        Ok(Self {
            name: name.into(),
            fees,
            pools: Vec::new(),
            factory: H256::from_str(&factory.into())?,
            router: H256::from_str(&router.into())?,
        })
    }

    pub fn decode_pair_trace_logs(pair_abi: &Abi, trace_log: TraceLogData) -> Vec<(String, Log)> {
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

    fn pools(&self) -> Vec<Arc<dyn AmmPool>> {
        self.pools.iter().map(|x| x.clone() as Arc<dyn AmmPool>).collect()
    }
}
