#![allow(non_snake_case)]

use std::path::Path;

use ethers::{types::Address, utils::to_checksum};
use rusqlite::{params, Connection, OptionalExtension, Result, Row, Statement};

use vidger::types::NetworkKind;

use crate::amm::AmmProtocolKind;
use crate::types::CryptoToken;

#[derive(Debug)]
pub struct DbNetwork {
    pub id: i64,
    pub name: String,
    //pub native_token_id: i64, // TODO
}

impl DbNetwork {
    fn from_row(row: &Row) -> Result<DbNetwork> {
        Ok(DbNetwork {
            id: row.get(0)?,
            name: row.get(1)?,
            //native_token_id: row.get(2)?,
        })
    }
}

#[derive(Debug)]
pub struct DbProviderNetwork {
    pub id: i64,
    pub provider_id: i64,
    pub network_id: i64,
    pub subdomain: String,
    pub http_api_key: String,
    pub ws_api_key: String,
}

#[derive(Debug)]
pub struct DbProvider {
    pub id: i64,
    pub name: String,
    pub http: String,
    pub websocket: String,
    pub providers_networks_ids: String,
}

#[derive(Debug, Clone)]
pub struct DbTokenNetwork {
    pub id: i64,
    pub token_id: i64,
    pub network_id: i64,
    pub address: String,
    pub proxy: Option<String>,
    pub balance_contract_slot: i32,
    pub code: bytes::Bytes,
}

impl DbTokenNetwork {
    fn from_row(row: &Row) -> Result<DbTokenNetwork> {
        Ok(DbTokenNetwork {
            id: row.get(0)?,
            token_id: row.get(1)?,
            network_id: row.get(2)?,
            address: row.get(3)?,
            proxy: row.get(4).unwrap_or_default(),
            balance_contract_slot: row.get(5).unwrap_or_default(),
            code: bytes::Bytes::from(row.get::<usize, Vec<u8>>(6).unwrap_or_default()),
        })
    }
}

#[derive(Debug)]
pub struct DbToken {
    pub id: i64,
    pub name: String,
    pub symbol: String,
    pub decimals: i64,
    pub token_networks_ids: Option<String>,
}

impl DbToken {
    fn from_row(row: &Row) -> Result<DbToken> {
        Ok(DbToken {
            id: row.get(0)?,
            name: row.get(1)?,
            symbol: row.get(2)?,
            decimals: row.get(3)?,
            token_networks_ids: row.get(4)?,
        })
    }
}

#[derive(Debug)]
pub struct DbAmmNetwork {
    pub id: i64,
    pub amm_id: i64,
    pub network_id: i64,
    pub options: String,
}

impl DbAmmNetwork {
    fn from_row(row: &Row) -> Result<DbAmmNetwork> {
        Ok(DbAmmNetwork {
            id: row.get(0)?,
            amm_id: row.get(1)?,
            network_id: row.get(2)?,
            options: row.get(3)?,
        })
    }
}

#[derive(Debug)]
pub struct DbAmmProtocol {
    pub id: i64,
    pub name: String,
}

impl DbAmmProtocol {
    fn from_row(row: &Row) -> Result<DbAmmProtocol> {
        Ok(DbAmmProtocol {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    }
}

#[derive(Debug)]
pub struct DbAmmPool {
    pub id: i64,
    pub amm_id: i64,
    pub network_id: i64,
    pub address: String,
    pub token0_id: i64,
    pub token1_id: i64,
}

impl DbAmmPool {
    fn from_row(row: &Row) -> Result<DbAmmPool> {
        Ok(DbAmmPool {
            id: row.get(0)?,
            amm_id: row.get(1)?,
            network_id: row.get(2)?,
            address: row.get(3)?,
            token0_id: row.get(4)?,
            token1_id: row.get(5)?,
        })
    }
}

#[derive(Debug)]
pub struct DbAmm {
    pub id: i64,
    pub name: String,
    pub amm_protocol_id: i64,
    pub amm_networks_ids: String,
    pub options: String,
}

impl DbAmm {
    fn from_row(row: &Row) -> Result<DbAmm> {
        Ok(DbAmm {
            id: row.get(0)?,
            name: row.get(1)?,
            amm_protocol_id: row.get(2)?,
            amm_networks_ids: row.get(3)?,
            options: row.get(4)?,
        })
    }
}

pub struct Database {
    db: Connection,
}

impl Database {
    pub fn new(db_path: &Path) -> Result<Self> {
        let db: Connection = Connection::open(db_path)?;
        Ok(Database { db })
    }

    pub fn close(self) -> Result<(), (Connection, rusqlite::Error)> {
        self.db.close()
    }

    pub fn get_network(&self, network: &NetworkKind) -> Result<Option<DbNetwork>> {
        let db_network_id: i64 = match network {
            NetworkKind::Ethereum => 1,
            NetworkKind::Polygon => 2,
        };

        let mut stmt: Statement = self.db.prepare("SELECT * FROM Networks WHERE id = ? LIMIT 1")?;
        stmt.query_row(params![db_network_id], DbNetwork::from_row).optional()
    }

    pub fn get_token_network_by_token(&self, token_id: i64, network: &NetworkKind) -> Result<Option<DbTokenNetwork>> {
        let db_network: Option<DbNetwork> = self.get_network(network)?;
        if db_network.is_none() {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        let mut stmt: Statement = self
            .db
            .prepare("SELECT * FROM TokenNetworks WHERE tokenId = ? AND networkId = ? LIMIT 1")?;
        stmt.query_row(params![token_id, db_network.unwrap().id], DbTokenNetwork::from_row)
            .optional()
    }

    pub fn get_token_network(
        &self,
        address: impl Into<String>,
        network: &NetworkKind,
    ) -> Result<Option<DbTokenNetwork>> {
        let db_network: Option<DbNetwork> = self.get_network(network)?;
        if db_network.is_none() {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        let mut stmt: Statement = self
            .db
            .prepare("SELECT * FROM TokenNetworks WHERE networkId = ? AND address = ? LIMIT 1")?;
        stmt.query_row(
            params![db_network.unwrap().id, address.into()],
            DbTokenNetwork::from_row,
        )
        .optional()
    }

    pub fn add_token_network(
        &self,
        token_id: i64,
        network: &NetworkKind,
        address: impl Into<String>,
    ) -> Result<DbTokenNetwork> {
        let db_network: Option<DbNetwork> = self.get_network(network)?;
        if db_network.is_none() {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        let gg = self.get_token_network_by_token(token_id, network);
        if gg.is_ok_and(|x| x.is_some()) {
            return Err(rusqlite::Error::ExecuteReturnedResults);
        }

        let mut stmt: Statement = self
            .db
            .prepare("INSERT INTO TokenNetworks (tokenId, networkId, address) VALUES (?, ?, ?) RETURNING *")?;

        stmt.query_row(
            params![token_id, db_network.unwrap().id, address.into()],
            DbTokenNetwork::from_row,
        )
    }

    pub fn get_token_by_id(&self, id: i64) -> Result<Option<DbToken>> {
        let mut stmt: Statement = self.db.prepare("SELECT * FROM Tokens WHERE id = ? LIMIT 1")?;
        stmt.query_row(params![id], DbToken::from_row).optional()
    }

    pub fn get_token_by_address(&self, address: impl Into<String>, network: &NetworkKind) -> Result<Option<DbToken>> {
        let db_network: Option<DbNetwork> = self.get_network(network)?;
        if db_network.is_none() {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }
        let db_network: DbNetwork = db_network.unwrap();

        let mut stmt: Statement = self
            .db
            .prepare("SELECT * FROM TokenNetworks WHERE networkId = ? AND address = ? LIMIT 1")?;

        let db_token_network: Option<DbTokenNetwork> = stmt
            .query_row(params![db_network.id, address.into()], DbTokenNetwork::from_row)
            .optional()?;
        if db_token_network.is_none() {
            return Ok(None);
        }

        let db_token_network: DbTokenNetwork = db_token_network.unwrap();
        let mut stmt: Statement = self.db.prepare("SELECT * FROM Tokens WHERE id = ? LIMIT 1")?;
        stmt.query_row(params![db_token_network.token_id], DbToken::from_row)
            .optional()
    }

    pub fn get_token_and_network(
        &self,
        address: impl Into<String>,
        network: &NetworkKind,
    ) -> Result<Option<(DbToken, DbTokenNetwork)>> {
        let db_network: Option<DbNetwork> = self.get_network(network)?;
        if db_network.is_none() {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }
        let db_network: DbNetwork = db_network.unwrap();

        let mut stmt: Statement = self
            .db
            .prepare("SELECT * FROM TokenNetworks WHERE networkId = ? AND address = ? LIMIT 1")?;

        let db_token_network: DbTokenNetwork =
            stmt.query_row(params![db_network.id, address.into()], DbTokenNetwork::from_row)?;

        let mut stmt: Statement = self
            .db
            .prepare("SELECT * FROM Tokens WHERE instr(tokenNetworksIds, ?) > 0")?;

        stmt.query_row(params![format!(",{},", db_token_network.id)], |row| {
            Ok((DbToken::from_row(row)?, db_token_network))
        })
        .optional()
    }

    pub fn get_tokens(&self, network: &NetworkKind) -> Result<Vec<(DbToken, DbTokenNetwork)>> {
        let db_network: Option<DbNetwork> = self.get_network(network)?;
        if db_network.is_none() {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        let db_network: DbNetwork = db_network.unwrap();
        let mut stmt: Statement = self.db.prepare("SELECT * FROM TokenNetworks WHERE networkId = ?")?;
        let db_tokens_networks_map = stmt.query_map(params![db_network.id], DbTokenNetwork::from_row)?;

        let mut ret: Vec<(DbToken, DbTokenNetwork)> = Vec::new();
        for db_token_network in db_tokens_networks_map.map(|t| t.unwrap()) {
            let mut stmt: Statement = self
                .db
                .prepare("SELECT * FROM Tokens WHERE instr(tokenNetworksIds, ?) > 0")?;

            let tokens = stmt.query_map(params![format!(",{},", db_token_network.id)], DbToken::from_row)?;
            for token in tokens.map(|t: Result<DbToken>| t.unwrap()) {
                ret.push((token, db_token_network.clone()))
            }
        }

        Ok(ret)
    }

    pub fn add_token(&self, network: &NetworkKind, token: &CryptoToken) -> Result<(DbToken, DbTokenNetwork)> {
        let db_network: Option<DbNetwork> = self.get_network(network)?;
        if db_network.is_none() {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        // check if token already exists
        if self
            .get_token_network(to_checksum(token.address(), None), network)?
            .is_some()
        {
            return Err(rusqlite::Error::ExecuteReturnedResults);
        }

        let mut stmt: Statement = self
            .db
            .prepare("SELECT * FROM Tokens WHERE name = ? AND symbol = ? AND decimals = ? LIMIT 1")?;

        let db_token_opt = stmt
            .query_row(
                params![token.name(), token.symbol(), token.decimals()],
                DbToken::from_row,
            )
            .optional()?;
        let db_token: DbToken = if db_token_opt.is_some() {
            db_token_opt.unwrap()
        } else {
            let mut stmt: Statement = self
                .db
                .prepare("INSERT INTO Tokens (name, symbol, decimals) VALUES (?, ?, ?) RETURNING *")?;

            stmt.query_row(
                params![token.name(), token.symbol(), token.decimals()],
                DbToken::from_row,
            )?
        };

        let token_address: String = to_checksum(token.address(), None);
        let db_token_network: DbTokenNetwork = self.add_token_network(db_token.id, network, &token_address)?;

        let mut stmt: Statement = self.db.prepare("UPDATE Tokens SET tokenNetworksIds = ? WHERE id = ?")?;
        stmt.execute(params![
            format!(
                "{}{},",
                db_token.token_networks_ids.unwrap_or(",".to_string()),
                db_token_network.id
            ),
            db_token.id
        ])?;

        Ok(self.get_token_and_network(&token_address, network)?.unwrap())
    }

    pub fn update_token(&self, network: &NetworkKind, token: &CryptoToken) -> Result<()> {
        let db_token: Option<DbToken> = self.get_token_by_address(to_checksum(token.address(), None), network)?;
        let Some(db_token) = db_token else {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        };

        let mut stmt: Statement = self
            .db
            .prepare("UPDATE Tokens SET name = ?, symbol = ?, decimals = ? WHERE id = ?")?;
        stmt.execute(params![token.name(), token.symbol(), token.decimals(), db_token.id])?;

        // Update token network
        let token_address: String = to_checksum(token.address(), None);
        let db_token_network: Option<DbTokenNetwork> = self.get_token_network(&token_address, network)?;
        let Some(db_token_network) = db_token_network else {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        };

        let mut stmt: Statement = self.db.prepare(
            "UPDATE TokenNetworks SET address = ?, proxy = ?, balanceContractSlot = ?, code = ? WHERE id = ?",
        )?;
        stmt.execute(params![
            token_address,
            token.proxy_address().map(|pa| to_checksum(&pa, None)),
            token.balance_contract_slot(),
            token.code().to_vec(),
            db_token_network.id
        ])?;

        Ok(())
    }

    pub fn delete_token(&self, network: &NetworkKind, token_address: &str) -> Result<()> {
        let db_token: Option<(DbToken, DbTokenNetwork)> = self.get_token_and_network(token_address, network)?;
        let Some(db_token) = db_token else {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        };

        let mut stmt: Statement = self.db.prepare("DELETE FROM Tokens WHERE id = ?")?;
        stmt.execute(params![db_token.0.id])?;

        let mut stmt: Statement = self.db.prepare("DELETE FROM TokensNetworks WHERE id = ?")?;
        stmt.execute(params![db_token.1.id])?;

        Ok(())
    }

    pub fn get_amm_protocol(&self, protocol: &AmmProtocolKind) -> Result<Option<DbAmmProtocol>> {
        let mut stmt: Statement = self.db.prepare("SELECT * FROM AmmProtocols WHERE name = ? LIMIT 1")?;
        stmt.query_row(params![protocol.to_string()], DbAmmProtocol::from_row)
            .optional()
    }

    pub fn get_amm_protocol_by_id(&self, id: i64) -> Result<Option<DbAmmProtocol>> {
        let mut stmt: Statement = self.db.prepare("SELECT * FROM AmmProtocols WHERE id = ? LIMIT 1")?;
        stmt.query_row(params![id], DbAmmProtocol::from_row).optional()
    }

    pub fn add_amm_protocol(&self, protocol: &AmmProtocolKind) -> Result<DbAmmProtocol> {
        let mut stmt: Statement = self
            .db
            .prepare("INSERT INTO AmmProtocols (name) VALUES (?) RETURNING *")?;

        stmt.query_row(params![protocol.to_string()], DbAmmProtocol::from_row)
    }

    pub fn get_amm_network(&self, amm_id: i64, network: &NetworkKind) -> Result<Option<DbAmmNetwork>> {
        let db_network: Option<DbNetwork> = self.get_network(network)?;
        if db_network.is_none() {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        let mut stmt: Statement = self
            .db
            .prepare("SELECT * FROM AmmNetworks WHERE ammId = ? AND networkId = ? LIMIT 1")?;
        stmt.query_row(params![amm_id, db_network.unwrap().id], DbAmmNetwork::from_row)
            .optional()
    }

    pub fn get_amm_networks_by_network(&self, network: &NetworkKind) -> Result<Vec<DbAmmNetwork>> {
        let db_network: Option<DbNetwork> = self.get_network(network)?;
        if db_network.is_none() {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        let mut stmt: Statement = self.db.prepare("SELECT * FROM AmmNetworks WHERE networkId = ?")?;
        let ret = stmt
            .query_map(params![db_network.unwrap().id], DbAmmNetwork::from_row)?
            .collect();
        ret
    }

    pub fn add_amm_network(
        &self,
        amm_id: i64,
        network: &NetworkKind,
        options: impl Into<String>,
    ) -> Result<DbAmmNetwork> {
        let db_network: Option<DbNetwork> = self.get_network(network)?;
        if db_network.is_none() {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        let mut stmt: Statement = self
            .db
            .prepare("INSERT INTO AmmNetworks (ammId, networkId, options) VALUES (?, ?, ?) RETURNING *")?;

        stmt.query_row(
            params![amm_id, db_network.unwrap().id, options.into()],
            DbAmmNetwork::from_row,
        )
    }

    pub fn get_amm_pool(&self, network: &NetworkKind, address: impl Into<String>) -> Result<Option<DbAmmPool>> {
        let db_network: Option<DbNetwork> = self.get_network(network)?;
        if db_network.is_none() {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        let mut stmt: Statement = self
            .db
            .prepare("SELECT * FROM AmmPools WHERE networkId = ? AND address = ? LIMIT 1")?;

        stmt.query_row(params![db_network.unwrap().id, address.into()], DbAmmPool::from_row)
            .optional()
    }

    pub fn get_amm_pool_by_tokens(
        &self,
        amm_id: i64,
        network: &NetworkKind,
        token_a: impl Into<String>,
        token_b: impl Into<String>,
    ) -> Result<Option<DbAmmPool>> {
        let db_network: Option<DbNetwork> = self.get_network(network)?;
        if db_network.is_none() {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }
        let db_network = db_network.unwrap();

        let db_token_a: Option<DbToken> = self.get_token_by_address(token_a.into(), network)?;
        let db_token_b: Option<DbToken> = self.get_token_by_address(token_b.into(), network)?;
        if db_token_a.is_none() || db_token_b.is_none() {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        let token_a: DbToken = db_token_a.unwrap();
        let token_b: DbToken = db_token_b.unwrap();

        let mut stmt: Statement = self.db.prepare(
            "SELECT * FROM AmmPools WHERE ammId = ?1 AND networkId = ?2 AND ((token0Id = ?3 AND token1Id = ?4) OR (token1Id = ?3 AND token0Id = ?4)) LIMIT 1",
        )?;

        stmt.query_row(
            params![amm_id, db_network.id, token_a.id, token_b.id],
            DbAmmPool::from_row,
        )
        .optional()
    }

    pub fn get_amm_pools(&self, network: &NetworkKind, valid_pools_only: bool) -> Result<Vec<DbAmmPool>> {
        let db_network: Option<DbNetwork> = self.get_network(network)?;
        if db_network.is_none() {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        let mut stmt: Statement = if valid_pools_only {
            self.db.prepare("SELECT * FROM AmmPools WHERE networkId = ?")?
        } else {
            self.db.prepare(&format!(
                "SELECT * FROM AmmPools WHERE networkId = ? AND address != {}",
                to_checksum(&Address::zero(), None)
            ))?
        };

        let ret = stmt
            .query_map(params![db_network.unwrap().id], DbAmmPool::from_row)?
            .collect();
        ret
    }

    pub fn get_amm_pools_by_amm_id(
        &self,
        amm_id: i64,
        network: &NetworkKind,
        valid_pools_only: bool,
    ) -> Result<Vec<DbAmmPool>> {
        let db_network: Option<DbNetwork> = self.get_network(network)?;
        if db_network.is_none() {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        let mut stmt: Statement = if valid_pools_only {
            self.db
                .prepare("SELECT * FROM AmmPools WHERE ammId = ? AND networkId = ?")?
        } else {
            self.db.prepare(&format!(
                "SELECT * FROM AmmPools WHERE ammId = ? AND networkId = ? AND address != {}",
                to_checksum(&Address::zero(), None)
            ))?
        };

        let ret = stmt
            .query_map(params![amm_id, db_network.unwrap().id], DbAmmPool::from_row)?
            .collect();
        ret
    }

    pub fn add_amm_pool(
        &self,
        address: &Address,
        network: &NetworkKind,
        amm_id: i64,
        token0_id: i64,
        token1_id: i64,
    ) -> Result<DbAmmPool> {
        let db_network: Option<DbNetwork> = self.get_network(network)?;
        if db_network.is_none() {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        let db_amm: Option<DbAmm> = self.get_amm_by_id(amm_id)?;
        if db_amm.is_none() {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        let db_amm: DbAmm = db_amm.unwrap();
        let pool_address: String = to_checksum(address, None);
        if self.get_amm_pool(network, &pool_address)?.is_some() {
            return Err(rusqlite::Error::ExecuteReturnedResults);
        };

        let db_token0: Option<DbToken> = self.get_token_by_id(token0_id)?;
        if db_token0.is_none() {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        let db_token1: Option<DbToken> = self.get_token_by_id(token1_id)?;
        if db_token1.is_none() {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        let mut stmt: Statement = self.db.prepare(
            "INSERT INTO AmmPools (ammId, networkId, address, token0Id, token1Id) VALUES (?, ?, ?, ?, ?) RETURNING *",
        )?;

        stmt.query_row(
            params![db_amm.id, db_network.unwrap().id, pool_address, token0_id, token1_id],
            DbAmmPool::from_row,
        )
    }

    pub fn add_amm_pool_empty(
        &self,
        amm_id: i64,
        network: &NetworkKind,
        token_a: &str,
        token_b: &str,
    ) -> Result<DbAmmPool> {
        let db_network: Option<DbNetwork> = self.get_network(network)?;
        if db_network.is_none() {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        let db_amm: Option<DbAmm> = self.get_amm_by_id(amm_id)?;
        if db_amm.is_none() {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        let db_token0: Option<(DbToken, DbTokenNetwork)> = self.get_token_and_network(token_a, network)?;
        if db_token0.is_none() {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        let db_token1: Option<(DbToken, DbTokenNetwork)> = self.get_token_and_network(token_b, network)?;
        if db_token1.is_none() {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        let db_amm: DbAmm = db_amm.unwrap();
        if self
            .get_amm_pool_by_tokens(db_amm.id, network, token_a, token_b)?
            .is_some()
        {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        let mut stmt: Statement = self.db.prepare(
            "INSERT INTO AmmPools (ammId, networkId, address, token0Id, token1Id) VALUES (?, ?, ?, ?, ?) RETURNING *",
        )?;

        let pool_address: String = to_checksum(&Address::zero(), None);
        stmt.query_row(
            params![
                db_amm.id,
                db_network.unwrap().id,
                pool_address,
                db_token0.unwrap().0.id,
                db_token1.unwrap().0.id
            ],
            DbAmmPool::from_row,
        )
    }

    pub fn get_amm_by_name(&self, name: &str) -> Result<Option<DbAmm>> {
        let mut stmt: Statement = self.db.prepare("SELECT * FROM Amms WHERE name = ? LIMIT 1")?;
        stmt.query_row(params![name], DbAmm::from_row).optional()
    }

    pub fn get_amm_by_id(&self, amm_id: i64) -> Result<Option<DbAmm>> {
        let mut stmt: Statement = self.db.prepare("SELECT * FROM Amms WHERE id = ? LIMIT 1")?;
        stmt.query_row(params![amm_id], DbAmm::from_row).optional()
    }

    pub fn get_amms(&self) -> Result<Vec<DbAmm>> {
        let mut stmt: Statement = self.db.prepare("SELECT * FROM Amms")?;
        let ret = stmt.query_map([], DbAmm::from_row)?.collect();
        ret
    }

    pub fn get_amms_by_network(&self, network: &NetworkKind) -> Result<Vec<DbAmm>> {
        let db_network: Option<DbNetwork> = self.get_network(network)?;
        if db_network.is_none() {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        let db_amm_networks: Vec<DbAmmNetwork> = self.get_amm_networks_by_network(network)?;
        let mut stmt: Statement = self
            .db
            .prepare("SELECT * FROM Amms WHERE instr(ammNetworksIds, ?) > 0")?;

        let mut ret: Vec<DbAmm> = Vec::new();
        for db_Amm_network in db_amm_networks {
            for ele in stmt.query_map([format!(",{},", db_Amm_network.id)], DbAmm::from_row)? {
                ret.push(ele?);
            }
        }

        Ok(ret)
    }

    pub fn add_amm(&self, amm: &AmmProtocolKind, options: impl Into<String>) -> Result<DbAmm> {
        let amm_protocol_id: Option<DbAmmProtocol> = self.get_amm_protocol(amm)?;
        if amm_protocol_id.is_none() {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        let mut stmt: Statement = self
            .db
            .prepare("INSERT INTO Amms (name, ammProtocolId, options) VALUES (?, ?, ?) RETURNING id")?;

        stmt.query_row(
            params![amm.name(), amm_protocol_id.unwrap().id, options.into()],
            DbAmm::from_row,
        )
    }
}
