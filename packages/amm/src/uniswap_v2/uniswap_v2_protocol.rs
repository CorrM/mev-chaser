use anyhow::Result;
use contracts::UNISWAPV2PAIRABI_ABI;
use ethers_core::{
    abi::{Log, RawLog},
    types::Address,
};
use shared::trace::TraceLogData;
use std::{collections::HashMap, str::FromStr, sync::Arc};

use crate::{AmmPool, AmmProtocol, AmmProtocolKind};

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

    pub fn decode_pair_trace_logs(trace_log: &TraceLogData) -> HashMap<String, (Address, Log)> {
        let mut ret: HashMap<String, (Address, Log)> = HashMap::new();

        for ev in UNISWAPV2PAIRABI_ABI.events() {
            let raw_log: &RawLog = trace_log.raw_log();
            if ev.signature() != raw_log.topics[0] {
                continue;
            }

            let log_result: Result<Log, ethers_core::abi::Error> = ev.parse_log(raw_log.clone());

            if let Ok(log) = log_result {
                ret.insert(ev.name.clone(), (trace_log.address(), log));
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
    fn kind(&self) -> AmmProtocolKind {
        AmmProtocolKind::UniswapV2
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn pools(&self) -> Vec<Arc<dyn AmmPool>> {
        self.pools.iter().map(|p| Arc::clone(p) as Arc<dyn AmmPool>).collect()
    }
}
