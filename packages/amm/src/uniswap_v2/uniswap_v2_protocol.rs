use anyhow::Result;
use contracts::UNISWAPV2PAIRABI_ABI;
use ethers_core::{
    abi::{Log, RawLog},
    types::{Address, CallLogFrame},
};
use std::{
    collections::HashMap,
    str::FromStr,
    sync::{Arc, RwLock},
};

use crate::{AmmPool, AmmProtocol, AmmProtocolKind};

use crate::uniswap_v2_pool::UniswapV2Pool;

#[derive(Clone)]
pub struct UniswapV2Protocol {
    name: String,
    fees: u32,
    pools: Vec<Arc<RwLock<UniswapV2Pool>>>,
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

    pub fn decode_pair_trace_logs(trace_log: &CallLogFrame) -> Option<HashMap<String, (Address, Log)>> {
        let mut ret: HashMap<String, (Address, Log)> = HashMap::new();

        let Some(ref topics) = trace_log.topics else {
            return None;
        };

        if topics.is_empty() {
            return None;
        }

        for ev in UNISWAPV2PAIRABI_ABI.events() {
            if ev.signature() != topics[0] {
                continue;
            }

            // TODO: Need to change this
            let log_result: Result<Log, ethers_core::abi::Error> = ev.parse_log(RawLog {
                topics: topics.clone(),
                data: trace_log.data.as_ref().unwrap().to_vec(),
            });
            if let Ok(log) = log_result {
                ret.insert(ev.name.clone(), (trace_log.address.unwrap(), log));
            }
        }

        Some(ret)
    }

    pub fn decode_pair_trace_log(event_name: &str, trace_log: &CallLogFrame) -> Option<(Address, Log)> {
        let Some(ref topics) = trace_log.topics else {
            return None;
        };

        if topics.is_empty() {
            return None;
        }

        let Ok(ev) = UNISWAPV2PAIRABI_ABI.event(event_name) else {
            panic!("Event not found: {}", event_name);
        };

        if ev.signature() != topics[0] {
            return None;
        }

        // TODO: Need to change this (a lot of clones)
        let log_result: Result<Log, ethers_core::abi::Error> = ev.parse_log(RawLog {
            topics: topics.clone(),
            data: trace_log.data.as_ref().unwrap().to_vec(),
        });

        let Ok(log) = log_result else {
            return None;
        };

        Some((trace_log.address.unwrap(), log))
    }

    pub fn factory(&self) -> &Address {
        &self.factory
    }

    pub fn router(&self) -> &Address {
        &self.router
    }

    pub fn add_pool(&mut self, pool: UniswapV2Pool) {
        self.pools.push(Arc::new(RwLock::new(pool)));
    }
}

impl AmmProtocol for UniswapV2Protocol {
    fn kind(&self) -> AmmProtocolKind {
        AmmProtocolKind::UniswapV2
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn pools(&self) -> Vec<Arc<RwLock<dyn AmmPool>>> {
        self.pools
            .iter()
            .map(|p| Arc::clone(p) as Arc<RwLock<dyn AmmPool>>)
            .collect()
    }
}
