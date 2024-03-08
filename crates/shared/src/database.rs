use std::path::Path;

use ethers::{types::Address, utils::to_checksum};
use rusqlite::{params, Connection, OptionalExtension, Result, Statement};

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
    fn from_row(row: &rusqlite::Row) -> Result<DbNetwork> {
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
}

impl DbTokenNetwork {
    fn from_row(row: &rusqlite::Row) -> Result<DbTokenNetwork> {
        Ok(DbTokenNetwork {
            id: row.get(0)?,
            token_id: row.get(1)?,
            network_id: row.get(2)?,
            address: row.get(3)?,
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
    fn from_row(row: &rusqlite::Row) -> Result<DbToken> {
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
pub struct DbDexNetwork {
    pub id: i64,
    pub dex_id: i64,
    pub network_id: i64,
    pub options: String,
}

impl DbDexNetwork {
    fn from_row(row: &rusqlite::Row) -> Result<DbDexNetwork> {
        Ok(DbDexNetwork {
            id: row.get(0)?,
            dex_id: row.get(1)?,
            network_id: row.get(2)?,
            options: row.get(3)?,
        })
    }
}

#[derive(Debug)]
pub struct DbDexProtocol {
    pub id: i64,
    pub name: String,
}

impl DbDexProtocol {
    fn from_row(row: &rusqlite::Row) -> Result<DbDexProtocol> {
        Ok(DbDexProtocol {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    }
}

#[derive(Debug)]
pub struct DbDexPool {
    pub id: i64,
    pub dex_id: i64,
    pub network_id: i64,
    pub address: String,
    pub token0_id: i64,
    pub token1_id: i64,
}

impl DbDexPool {
    fn from_row(row: &rusqlite::Row) -> Result<DbDexPool> {
        Ok(DbDexPool {
            id: row.get(0)?,
            dex_id: row.get(1)?,
            network_id: row.get(2)?,
            address: row.get(3)?,
            token0_id: row.get(4)?,
            token1_id: row.get(5)?,
        })
    }
}

#[derive(Debug)]
pub struct DbDex {
    pub id: i64,
    pub name: String,
    pub dex_protocol_id: i64,
    pub dex_networks_ids: String,
    pub options: String,
}

impl DbDex {
    fn from_row(row: &rusqlite::Row) -> Result<DbDex> {
        Ok(DbDex {
            id: row.get(0)?,
            name: row.get(1)?,
            dex_protocol_id: row.get(2)?,
            dex_networks_ids: row.get(3)?,
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
        let db_tokens_networks = stmt.query_map(params![db_network.id], DbTokenNetwork::from_row)?;

        let mut ret: Vec<(DbToken, DbTokenNetwork)> = Vec::new();

        for db_token_network in db_tokens_networks.map(|t| t.unwrap()) {
            let mut stmt: Statement = self
                .db
                .prepare("SELECT * FROM Tokens WHERE instr(tokenNetworksIds, ?) > 0")?;

            let tokens = stmt.query_map(params![format!(",{},", db_token_network.id)], DbToken::from_row)?;
            for token in tokens.map(|t| t.unwrap()) {
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
            .get_token_network(token.address().to_checksum(None), network)?
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

        let token_address: String = token.address().to_checksum(None);
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

    pub fn get_dex_protocol(&self, protocol: &AmmProtocolKind) -> Result<Option<DbDexProtocol>> {
        let mut stmt: Statement = self.db.prepare("SELECT * FROM DexProtocols WHERE name = ? LIMIT 1")?;
        stmt.query_row(params![protocol.to_string()], DbDexProtocol::from_row)
            .optional()
    }

    pub fn get_dex_protocol_by_id(&self, id: i64) -> Result<Option<DbDexProtocol>> {
        let mut stmt: Statement = self.db.prepare("SELECT * FROM DexProtocols WHERE id = ? LIMIT 1")?;
        stmt.query_row(params![id], DbDexProtocol::from_row).optional()
    }

    pub fn add_dex_protocol(&self, protocol: &AmmProtocolKind) -> Result<DbDexProtocol> {
        let mut stmt: Statement = self
            .db
            .prepare("INSERT INTO DexProtocols (name) VALUES (?) RETURNING *")?;

        stmt.query_row(params![protocol.to_string()], DbDexProtocol::from_row)
    }

    pub fn get_dex_network(&self, dex_id: i64, network: &NetworkKind) -> Result<Option<DbDexNetwork>> {
        let db_network: Option<DbNetwork> = self.get_network(network)?;
        if db_network.is_none() {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        let mut stmt: Statement = self
            .db
            .prepare("SELECT * FROM DexNetworks WHERE dexId = ? AND networkId = ? LIMIT 1")?;
        stmt.query_row(params![dex_id, db_network.unwrap().id], DbDexNetwork::from_row)
            .optional()
    }

    pub fn get_dex_networks_by_network(&self, network: &NetworkKind) -> Result<Vec<DbDexNetwork>> {
        let db_network: Option<DbNetwork> = self.get_network(network)?;
        if db_network.is_none() {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        let mut stmt: Statement = self.db.prepare("SELECT * FROM DexNetworks WHERE networkId = ?")?;
        let ret = stmt
            .query_map(params![db_network.unwrap().id], DbDexNetwork::from_row)?
            .collect();
        ret
    }

    pub fn add_dex_network(
        &self,
        dex_id: i64,
        network: &NetworkKind,
        options: impl Into<String>,
    ) -> Result<DbDexNetwork> {
        let db_network: Option<DbNetwork> = self.get_network(network)?;
        if db_network.is_none() {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        let mut stmt: Statement = self
            .db
            .prepare("INSERT INTO DexNetworks (dexId, networkId, options) VALUES (?, ?, ?) RETURNING *")?;

        stmt.query_row(
            params![dex_id, db_network.unwrap().id, options.into()],
            DbDexNetwork::from_row,
        )
    }

    pub fn get_dex_pool(&self, network: &NetworkKind, address: impl Into<String>) -> Result<Option<DbDexPool>> {
        let db_network: Option<DbNetwork> = self.get_network(network)?;
        if db_network.is_none() {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        let mut stmt: Statement = self
            .db
            .prepare("SELECT * FROM DexPools WHERE networkId = ? AND address = ? LIMIT 1")?;

        stmt.query_row(params![db_network.unwrap().id, address.into()], DbDexPool::from_row)
            .optional()
    }

    pub fn get_dex_pool_by_tokens(
        &self,
        dex_id: i64,
        network: &NetworkKind,
        token_a: impl Into<String>,
        token_b: impl Into<String>,
    ) -> Result<Option<DbDexPool>> {
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
            "SELECT * FROM DexPools WHERE dexId = ?1 AND networkId = ?2 AND ((token0Id = ?3 AND token1Id = ?4) OR (token1Id = ?3 AND token0Id = ?4)) LIMIT 1",
        )?;

        stmt.query_row(
            params![dex_id, db_network.id, token_a.id, token_b.id],
            DbDexPool::from_row,
        )
        .optional()
    }

    pub fn get_dex_pools(&self, network: &NetworkKind, valid_pools_only: bool) -> Result<Vec<DbDexPool>> {
        let db_network: Option<DbNetwork> = self.get_network(network)?;
        if db_network.is_none() {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        let mut stmt: Statement = if valid_pools_only {
            self.db.prepare("SELECT * FROM DexPools WHERE networkId = ?")?
        } else {
            self.db.prepare(&format!(
                "SELECT * FROM DexPools WHERE networkId = ? AND address != {}",
                to_checksum(&Address::zero(), None)
            ))?
        };

        let ret = stmt
            .query_map(params![db_network.unwrap().id], DbDexPool::from_row)?
            .collect();
        ret
    }

    pub fn get_dex_pools_by_dex_id(
        &self,
        dex_id: i64,
        network: &NetworkKind,
        valid_pools_only: bool,
    ) -> Result<Vec<DbDexPool>> {
        let db_network: Option<DbNetwork> = self.get_network(network)?;
        if db_network.is_none() {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        let mut stmt: Statement = if valid_pools_only {
            self.db
                .prepare("SELECT * FROM DexPools WHERE dexId = ? AND networkId = ?")?
        } else {
            self.db.prepare(&format!(
                "SELECT * FROM DexPools WHERE dexId = ? AND networkId = ? AND address != {}",
                to_checksum(&Address::zero(), None)
            ))?
        };

        let ret = stmt
            .query_map(params![dex_id, db_network.unwrap().id], DbDexPool::from_row)?
            .collect();
        ret
    }

    pub fn add_dex_pool(
        &self,
        address: &Address,
        network: &NetworkKind,
        dex_id: i64,
        token0_id: i64,
        token1_id: i64,
    ) -> Result<DbDexPool> {
        let db_network: Option<DbNetwork> = self.get_network(network)?;
        if db_network.is_none() {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        let db_dex: Option<DbDex> = self.get_dex_by_id(dex_id)?;
        if db_dex.is_none() {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        let db_dex: DbDex = db_dex.unwrap();
        let pool_address: String = to_checksum(address, None);
        if self.get_dex_pool(network, &pool_address)?.is_some() {
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
            "INSERT INTO DexPools (dexId, networkId, address, token0Id, token1Id) VALUES (?, ?, ?, ?, ?) RETURNING *",
        )?;

        stmt.query_row(
            params![db_dex.id, db_network.unwrap().id, pool_address, token0_id, token1_id],
            DbDexPool::from_row,
        )
    }

    pub fn add_dex_pool_empty(
        &self,
        dex_id: i64,
        network: &NetworkKind,
        token_a: &str,
        token_b: &str,
    ) -> Result<DbDexPool> {
        let db_network: Option<DbNetwork> = self.get_network(network)?;
        if db_network.is_none() {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        let db_dex: Option<DbDex> = self.get_dex_by_id(dex_id)?;
        if db_dex.is_none() {
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

        let db_dex: DbDex = db_dex.unwrap();
        if self
            .get_dex_pool_by_tokens(db_dex.id, network, token_a, token_b)?
            .is_some()
        {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        let mut stmt: Statement = self.db.prepare(
            "INSERT INTO DexPools (dexId, networkId, address, token0Id, token1Id) VALUES (?, ?, ?, ?, ?) RETURNING *",
        )?;

        let pool_address: String = to_checksum(&Address::zero(), None);
        stmt.query_row(
            params![
                db_dex.id,
                db_network.unwrap().id,
                pool_address,
                db_token0.unwrap().0.id,
                db_token1.unwrap().0.id
            ],
            DbDexPool::from_row,
        )
    }

    pub fn get_dex_by_name(&self, name: &str) -> Result<Option<DbDex>> {
        let mut stmt: Statement = self.db.prepare("SELECT * FROM Dexes WHERE name = ? LIMIT 1")?;
        stmt.query_row(params![name], DbDex::from_row).optional()
    }

    pub fn get_dex_by_id(&self, dex_id: i64) -> Result<Option<DbDex>> {
        let mut stmt: Statement = self.db.prepare("SELECT * FROM Dexes WHERE id = ? LIMIT 1")?;
        stmt.query_row(params![dex_id], DbDex::from_row).optional()
    }

    pub fn get_dexes(&self) -> Result<Vec<DbDex>> {
        let mut stmt: Statement = self.db.prepare("SELECT * FROM Dexes")?;
        let ret = stmt.query_map([], DbDex::from_row)?.collect();
        ret
    }

    pub fn get_dexes_by_network(&self, network: &NetworkKind) -> Result<Vec<DbDex>> {
        let db_network: Option<DbNetwork> = self.get_network(network)?;
        if db_network.is_none() {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        let db_dex_networks: Vec<DbDexNetwork> = self.get_dex_networks_by_network(network)?;
        let mut stmt: Statement = self
            .db
            .prepare("SELECT * FROM Dexes WHERE instr(dexNetworksIds, ?) > 0")?;

        let mut ret: Vec<DbDex> = Vec::new();
        for db_dex_network in db_dex_networks {
            for ele in stmt.query_map([format!(",{},", db_dex_network.id)], DbDex::from_row)? {
                ret.push(ele?);
            }
        }

        Ok(ret)
    }

    pub fn add_dex(&self, dex: &AmmProtocolKind, options: impl Into<String>) -> Result<DbDex> {
        let dex_protocol_id: Option<DbDexProtocol> = self.get_dex_protocol(dex)?;
        if dex_protocol_id.is_none() {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        let mut stmt: Statement = self
            .db
            .prepare("INSERT INTO Dexes (name, dexProtocolId, options) VALUES (?, ?, ?) RETURNING id")?;

        stmt.query_row(
            params![dex.name(), dex_protocol_id.unwrap().id, options.into()],
            DbDex::from_row,
        )
    }
}
