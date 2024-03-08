use std::sync::Arc;
use std::{collections::HashMap, str::FromStr};

use alloy_primitives::Address;
use anyhow::Result;
use ethers::{
    abi::{Log, RawLog},
    types::CallLogFrame,
};

use contracts::uniswap_v2_pair::UNISWAPV2PAIRABI_ABI;

use crate::amm::{AmmPoolKind, AmmProtocol};

#[derive(Clone)]
pub struct UniswapV2Protocol {
    name: String,
    factory: Address,
    router: Address,
    pools: Vec<Arc<AmmPoolKind>>,
}

impl UniswapV2Protocol {
    pub fn new(name: impl Into<String>, factory: impl Into<String>, router: impl Into<String>) -> Result<Self> {
        Ok(Self {
            name: name.into(),
            factory: Address::from_str(&factory.into())?,
            router: Address::from_str(&router.into())?,
            pools: Vec::new(),
        })
    }

    #[inline]
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
            let log_result: Result<Log, ethers::abi::Error> = ev.parse_log(RawLog {
                topics: topics.clone(),
                data: trace_log.data.as_ref().unwrap().to_vec(),
            });
            if let Ok(log) = log_result {
                ret.insert(ev.name.clone(), (trace_log.address.unwrap().0.into(), log));
            }
        }

        Some(ret)
    }

    #[inline]
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
        let log_result: Result<Log, ethers::abi::Error> = ev.parse_log(RawLog {
            topics: topics.clone(),
            data: trace_log.data.as_ref().unwrap().to_vec(),
        });

        let Ok(log) = log_result else {
            return None;
        };

        Some((trace_log.address.unwrap().0.into(), log))
    }
}

impl UniswapV2Protocol {
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    pub fn factory(&self) -> &Address {
        &self.factory
    }

    #[inline]
    pub fn router(&self) -> &Address {
        &self.router
    }
}

impl AmmProtocol for UniswapV2Protocol {
    fn name(&self) -> &str {
        &self.name
    }

    fn pools(&self) -> &Vec<Arc<AmmPoolKind>> {
        &self.pools
    }

    fn add_pool(&mut self, pool: AmmPoolKind) {
        self.pools.push(Arc::new(pool));
    }
}
