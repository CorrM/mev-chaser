use std::sync::Arc;

use anyhow::{anyhow, Result};
use ethers::addressbook::Address;

use vidger::types::NetworkKind;

use crate::amm::{AmmPoolKind, AmmProtocolKind, UniswapV2Pool, UniswapV2Protocol};
use crate::database::{Database, DbAmm, DbAmmPool, DbToken, DbTokenNetwork};
use crate::managers::TokenManager;
use crate::types::CryptoToken;

pub struct AmmManager {
    amms: Vec<Arc<AmmProtocolKind>>,
}

impl AmmManager {
    fn get_amms(
        db: &Database,
        network: &NetworkKind,
        token_manager: &TokenManager,
    ) -> Result<Vec<Arc<AmmProtocolKind>>> {
        let mut ret: Vec<Arc<AmmProtocolKind>> = Vec::new();

        let db_dexes: Vec<DbAmm> = db.get_amms_by_network(network)?;
        for db_dex in db_dexes {
            let Some(db_dex_protocol) = db.get_amm_protocol_by_id(db_dex.amm_protocol_id)? else {
                continue;
            };

            let Some(db_dex_network) = db.get_amm_network(db_dex.id, network)? else {
                continue;
            };

            let db_dex_pools: Vec<DbAmmPool> = db.get_amm_pools_by_amm_id(db_dex.id, network, true)?;
            match db_dex_protocol.name.as_str() {
                "UniswapV2" => {
                    //let dex_options: serde_json::Value = serde_json::from_str(&db_dex.options)?;
                    let network_options: serde_json::Value = serde_json::from_str(&db_dex_network.options)?;

                    let uniswap_v2 = Arc::new(AmmProtocolKind::UniswapV2(UniswapV2Protocol::new(
                        db_dex.name,
                        network_options["factory"].as_str().unwrap(),
                        network_options["router"].as_str().unwrap(),
                    )?));

                    let mut pools: Vec<AmmPoolKind> = Vec::with_capacity(db_dex_pools.len());
                    for db_dex_pool in db_dex_pools {
                        let pool_address: Address = db_dex_pool.address.parse::<Address>()?;
                        if pool_address.is_zero() {
                            continue;
                        }

                        let token0: Option<DbToken> = db.get_token_by_id(db_dex_pool.token0_id)?;
                        let token1: Option<DbToken> = db.get_token_by_id(db_dex_pool.token1_id)?;
                        if token0.is_none() || token1.is_none() {
                            return Err(anyhow!("Token not found"));
                        }

                        let db_token0_network: DbTokenNetwork =
                            db.get_token_network_by_token(token0.unwrap().id, network)?.unwrap();
                        let db_token1_network: DbTokenNetwork =
                            db.get_token_network_by_token(token1.unwrap().id, network)?.unwrap();

                        let token0: &Arc<CryptoToken> =
                            token_manager.get_by_address_str(&db_token0_network.address).unwrap();
                        let token1: &Arc<CryptoToken> =
                            token_manager.get_by_address_str(&db_token1_network.address).unwrap();

                        let pool = AmmPoolKind::UniswapV2(UniswapV2Pool::new(
                            pool_address,
                            Arc::clone(&uniswap_v2),
                            Arc::clone(token0),
                            Arc::clone(token1),
                        )?);
                        pools.push(pool);
                    }

                    pools = pools[0..5].to_vec(); // TODO: REMOVE
                    pools.reverse(); // TODO: REMOVE
                    pools = pools[1..].to_vec(); // TODO: REMOVE

                    unsafe {
                        let uniswap_v2 = Arc::into_raw(uniswap_v2) as *mut AmmProtocolKind;

                        for pool in pools {
                            (*uniswap_v2).add_pool(pool);
                        }

                        ret.push(Arc::from_raw(uniswap_v2));
                    }
                }
                _ => panic!("Unsupported dex protocol"),
            }

            break; // TODO: REMOVE
        }

        Ok(ret)
    }

    pub fn new(amms: Vec<Arc<AmmProtocolKind>>) -> Self {
        Self { amms }
    }

    pub fn new_by_db(db: &Database, network: &NetworkKind, token_manager: &TokenManager) -> Result<Self> {
        Ok(Self::new(Self::get_amms(db, network, token_manager)?))
    }
}

impl AmmManager {
    #[inline]
    pub fn amms(&self) -> &Vec<Arc<AmmProtocolKind>> {
        &self.amms
    }
}
