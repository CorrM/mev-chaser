use std::str::FromStr;
use std::sync::Arc;

use anyhow::Result;
use ethers::{
    addressbook::Address,
    middleware::Middleware,
    types::BigEndianHash,
    types::{TxHash, H256, U256, U64},
};
use tokio::task::JoinError;

pub use pool_path_finder::*;

pub mod pool_path_finder;

pub fn get_proxy_implementation<M: Middleware + 'static>(
    provider: &Arc<M>,
    token: Address,
    block_number: Option<U64>,
) -> Result<Option<Address>> {
    // adapted from: https://github.com/gnosis/evm-proxy-detection/blob/main/src/index.ts
    let eip_1967_logic_slot: U256 =
        U256::from_str("0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc")?;
    let eip_1967_beacon_slot: U256 =
        U256::from_str("0xa3f0ad74e5423aebfd80d3ef4346578335a9a72aeaee59ff6cb3582b35133d50")?;
    let open_zeppelin_implementation_slot: U256 =
        U256::from_str("0x7050c9e0f4ca769c69bd3a8ef740bc37934f8e2c036e5a723fd8ee048ed3f8c3")?;
    let eip_1822_logic_slot: U256 =
        U256::from_str("0xc5f16f0fcc639fa48a6947836d9850f504798523bf8c9a3a87d5876cf622bcf7")?;

    let implementation_slots: Vec<U256> = vec![
        eip_1967_logic_slot,
        eip_1967_beacon_slot,
        open_zeppelin_implementation_slot,
        eip_1822_logic_slot,
    ];

    let slots: Vec<Result<Result<H256, <M as Middleware>::Error>, JoinError>> =
        async_scoped::TokioScope::scope_and_block(|s| {
            for slot in &implementation_slots {
                s.spawn(async {
                    provider
                        .get_storage_at(token, TxHash::from_uint(slot), block_number.map(|bn| bn.into()))
                        .await
                });
            }
        })
        .1;

    for slot in slots {
        let out: TxHash = slot??;
        let implementation = Address::from(out);
        if implementation != Address::zero() {
            return Ok(Some(implementation));
        }
    }

    Ok(None)
}
