use rusqlite::{params, Connection, OptionalExtension, Result, Statement};
use std::path::Path;

use shared::amm::{AmmProtocol, AmmProtocolKind};
use shared::token::CryptoToken;
use shared::{amm::AmmPool, network::NetworkKind};

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
    pub network_id: i64,
    pub address: String,
}

impl DbTokenNetwork {
    fn from_row(row: &rusqlite::Row) -> Result<DbTokenNetwork> {
        Ok(DbTokenNetwork {
            id: row.get(0)?,
            network_id: row.get(1)?,
            address: row.get(2)?,
        })
    }
}

#[derive(Debug)]
pub struct DbToken {
    pub id: i64,
    pub name: String,
    pub symbol: String,
    pub decimals: i64,
    pub token_networks_ids: String,
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
    pub pools_ids: Option<String>,
    pub dex_networks_ids: String,
    pub options: String,
}

impl DbDex {
    fn from_row(row: &rusqlite::Row) -> Result<DbDex> {
        Ok(DbDex {
            id: row.get(0)?,
            name: row.get(1)?,
            dex_protocol_id: row.get(2)?,
            pools_ids: row.get(3)?,
            dex_networks_ids: row.get(4)?,
            options: row.get(5)?,
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

    pub fn add_token_network(&self, network: &NetworkKind, address: impl Into<String>) -> Result<DbTokenNetwork> {
        let db_network: Option<DbNetwork> = self.get_network(network)?;
        if db_network.is_none() {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        let mut stmt: Statement = self
            .db
            .prepare("INSERT INTO TokenNetworks (networkId, address) VALUES (?, ?) RETURNING *")?;

        stmt.query_row(
            params![db_network.unwrap().id, address.into()],
            DbTokenNetwork::from_row,
        )
    }

    pub fn get_token(
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

        stmt.query_row(params![format!(",{},", db_network.id)], |row| {
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

    pub fn add_token(&self, token: &CryptoToken) -> Result<(DbToken, DbTokenNetwork)> {
        let db_network: Option<DbNetwork> = self.get_network(token.network())?;
        if db_network.is_none() {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        let mut stmt: Statement = self
            .db
            .prepare("INSERT INTO Tokens (name, symbol, decimals) VALUES (?, ?, ?) RETURNING *")?;

        let db_token: DbToken = stmt.query_row(
            params![token.name(), token.symbol(), token.decimals()],
            DbToken::from_row,
        )?;

        let token_address: String = format!("{:?}", token.address());
        let db_token_network: DbTokenNetwork =
            self.add_token_network(token.network(), &token_address)?;
        
        let mut stmt: Statement = self
            .db
            .prepare("UPDATE Tokens SET tokenNetworksIds = ? WHERE id = ?")?;

        stmt.execute(params![
            format!("{}{},", db_token.token_networks_ids, db_token_network.id),
            db_token.id
        ])?;

        Ok(self.get_token(&token_address, token.network())?.unwrap())
    }

    pub fn get_dex_protocol(&self, protocol: &AmmProtocolKind) -> Result<Option<DbDexProtocol>> {
        let mut stmt: Statement = self.db.prepare("SELECT * FROM DexProtocols WHERE name = ? LIMIT 1")?;
        stmt.query_row(params![protocol.to_string()], DbDexProtocol::from_row)
            .optional()
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

    pub fn add_dex_network(&self, dex_id: i64, network: &NetworkKind, options: &str) -> Result<DbDexNetwork> {
        let db_network: Option<DbNetwork> = self.get_network(network)?;
        if db_network.is_none() {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        let mut stmt: Statement = self
            .db
            .prepare("INSERT INTO DexNetworks (dexId, networkId, options) VALUES (?, ?, ?) RETURNING *")?;

        stmt.query_row(params![dex_id, db_network.unwrap().id, options], DbDexNetwork::from_row)
    }

    pub fn get_dex_pool(&self, dex_id: i64, network: &NetworkKind, address: &str) -> Result<Option<DbDexPool>> {
        let db_network: Option<DbNetwork> = self.get_network(network)?;
        if db_network.is_none() {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        let mut stmt: Statement = self
            .db
            .prepare("SELECT * FROM DexPools WHERE dexId = ? AND networkId = ? AND address = ? LIMIT 1")?;

        stmt.query_row(params![dex_id, db_network.unwrap().id, address], DbDexPool::from_row)
            .optional()
    }

    pub fn add_dex_pool(&self, pool: &impl AmmPool) -> Result<DbDexPool> {
        let db_network: Option<DbNetwork> = self.get_network(pool.network())?;
        if db_network.is_none() {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        let db_dex: Option<DbDex> = self.get_dex_by_name(pool.dex().name())?;
        if db_dex.is_none() {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        let db_token0: Option<(DbToken, DbTokenNetwork)> =
            self.get_token(format!("{:?}", pool.token0().address()), pool.network())?;
        if db_token0.is_none() {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        let db_token1: Option<(DbToken, DbTokenNetwork)> =
            self.get_token(format!("{:?}", pool.token1().address()), pool.network())?;
        if db_token1.is_none() {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        let mut stmt: Statement = self.db.prepare(
            "INSERT INTO DexPools (dexId, networkId, address, token0Id, token1Id) VALUES (?, ?, ?, ?, ?) RETURNING *",
        )?;

        stmt.query_row(
            params![
                db_dex.unwrap().id,
                db_network.unwrap().id,
                format!("{:?}", pool.address()),
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

    pub fn add_dex(&self, dex: &impl AmmProtocol) -> Result<DbDex> {
        let dex_protocol_id: Option<DbDexProtocol> = self.get_dex_protocol(&dex.protocol())?;
        if dex_protocol_id.is_none() {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        let mut stmt: Statement = self
            .db
            .prepare("INSERT INTO Dexes (name, dexProtocolId, options) VALUES (?, ?, ?) RETURNING id")?;

        stmt.query_row(
            params![dex.name(), dex_protocol_id.unwrap().id, dex.options()],
            DbDex::from_row,
        )
    }
}
