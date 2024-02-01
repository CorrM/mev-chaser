use anyhow::Result;
use ethers_core::{
    abi::{Abi, Log, RawLog},
    types::Address,
};
use shared::trace::TraceLogData;
use std::{str::FromStr, sync::Arc};

use crate::AmmProtocol;

use crate::uniswap_v2_pool::UniswapV2Pool;

#[derive(Clone)]
pub struct UniswapV2Protocol {
    name: String,
    fees: u32,
    pools: Vec<Arc<UniswapV2Pool>>,
    factory: Address,
    router: Address,
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
            factory: Address::from_str(&factory.into())?,
            router: Address::from_str(&router.into())?,
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

    pub fn factory(&self) -> &Address {
        &self.factory
    }

    pub fn router(&self) -> &Address {
        &self.router
    }

    pub fn add_pool(&mut self, pool: UniswapV2Pool) {
        self.pools.push(Arc::new(pool));
    }
}

impl AmmProtocol for UniswapV2Protocol {
    type Pool = UniswapV2Pool;
    
    fn name(&self) -> &str {
        &self.name
    }

    fn pools(&self) -> Vec<Arc<UniswapV2Pool>> {
        self.pools.clone()
    }

    fn options(&self) -> String {
        panic!("Unimplemented");
    }

    fn set_options(&mut self, options: String) {
        panic!("Unimplemented");
    }
}
