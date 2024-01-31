/*use std::{
    io::{Error, ErrorKind},
    path::Path,
};

use anyhow::{anyhow, Result};
use rusqlite::{params, Connection, Statement};

use crate::{amm::AmmPool, network::NetworkKind, token::CryptoToken};

#[derive(Debug)]
pub struct DbNetwork {
    pub id: i64,
    pub name: String,
    pub native_token_id: i64,
}

#[derive(Debug)]
pub struct DbPool {
    pub id: i64,
    pub dex_id: i64,
    pub network_id: i64,
    pub address: String,
    pub token0_id: i64,
    pub token1_id: i64,
}

#[derive(Debug)]
pub struct DbDexNetwork {
    pub id: i64,
    pub dex_id: i64,
    pub network_id: i64,
    pub options: String,
}

#[derive(Debug)]
pub struct DbDexProtocol {
    pub id: i64,
    pub name: String,
}

#[derive(Debug)]
pub struct DbDex {
    pub id: i64,
    pub name: String,
    pub dex_protocol_id: i64,
    pub pools_ids: Option<String>,
    pub fee_percentage: f64,
    pub dex_networks_ids: String,
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

#[derive(Debug)]
pub struct DbCryptoToken {
    pub id: i64,
    pub name: String,
    pub symbol: String,
    pub decimals: i64,
    pub token_networks_ids: String,
}

#[derive(Debug)]
pub struct DbTokenNetwork {
    pub id: i64,
    pub network: i64,
    pub address: String,
}

pub struct Database {
    db: Connection,
}

impl Database {
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let db: Connection = Connection::open(db_path)?;
        Ok(Database { db })
    }

    pub fn close_db(&self) -> Result<(), (Connection, rusqlite::Error)> {
        self.db.close()
    }

    fn get_dex_by_id(&self, dex_id: i64) -> Option<DbDex> {
        let mut stmt: Statement<'_> = self.db.prepare("SELECT * FROM Dexes WHERE id = ? LIMIT 1")?;
        let dex_iter = stmt.query_map(params![dex_id], |row| {
            return Ok(DbDex {
                id: row.get(0)?,
                name: row.get(1)?,
                dex_protocol_id: row.get(2)?,
                pools_ids: row.get(3)?,
                fee_percentage: row.get(4)?,
                dex_networks_ids: row.get(5)?,
            });
        })?;

        for dex in dex_iter {
            return Some(dex?);
        }

        None
    }

    fn add_pool_by_dex_id_and_tokens(
        &self,
        dex_id: i64,
        network: NetworkKind,
        address: &str,
        token0: &CryptoToken,
        token1: &CryptoToken,
    ) -> Result<()> {
        let token0_result: Option<(DbCryptoToken, DbTokenNetwork)> =
            self.get_token_by_address(&token0.address(), network)?;
        let (db_token0, db_token_network0) = match token0_result {
            Some((token, token_network)) => (token, token_network),
            None => return Err(anyhow!("Failed to add pool, token0 not found")),
        };

        let token1_result: Option<(DbCryptoToken, DbTokenNetwork)> =
            self.get_token_by_address(token1.address().clone(), network)?;
        let (db_token1, db_token_network1) = match token1_result {
            Some((token, token_network)) => (token, token_network),
            None => return Err(Error::new(ErrorKind::Other, "Failed to add pool, token1 not found")),
        };

        let db_network = match self.get_network(network)? {
            Some(network) => network,
            None => return Err(Error::new(ErrorKind::Other, "Failed to add pool, network not found")),
        };

        let mut stmt = self.db.prepare(
            "INSERT INTO DexPools (dexId, networkId, address, token0Id, token1Id) VALUES (?, ?, ?, ?, ?) RETURNING id",
        )?;
        let pool_id: i64 = stmt
            .query(params![
                dex_id,
                db_network.id,
                address,
                db_token0.id,
                db_token1.id
            ])?
            .next()?.unwrap().get(0)?;

        let dex: DbDex = match self.get_dex_by_id(dex_id)? {
            Some(dex) => dex,
            None => return Err(Error::new(ErrorKind::Other, "Failed to add pool, dex not found")),
        };

        if let Some(pools_ids) = dex.pools_ids {
            let pool_ids: Vec<&str> = pools_ids.split(",").collect();
            if pool_ids.iter().any(|&id| id == pool_id.to_string()) {
                return Err(Error::new(ErrorKind::Other, "Pool already exists"));
            }
        }

        let updated_pools_ids = format!("{},{}", dex.pools_ids.unwrap_or_default(), pool_id);
        let mut stmt = self.db.prepare("UPDATE Dexes SET poolsIds = ? WHERE id = ?")?;
        stmt.execute(params![updated_pools_ids, dex_id])?;

        Ok(())
    }

    fn add_pool_by_dex_id(&self, dex_id: i64, pool: &impl AmmPool) -> Result<()> {
        self.add_pool_by_dex_id_and_tokens(dex_id, pool.network, pool.address(), pool.token0(), pool.token1())
    }

    pub fn get_network(&self, network: NetworkKind) -> Option<DbNetwork> {
        let mut stmt: Statement<'_> = self.db.prepare("SELECT * FROM Networks WHERE name LIKE ? LIMIT 1")?;
        let row = stmt.query(params![network])?.next()?.unwrap();

        Some(DbNetwork {
            id: row.get(0)?,
            name: row.get(1)?,
            native_token_id: row.get(2)?,
        })
    }

    pub fn get_token(&self, symbol: &str, network: NetworkKind) -> Option<(DbCryptoToken, DbTokenNetwork)> {
        let mut stmt: Statement<'_> = self.db.prepare("SELECT * FROM Tokens WHERE symbol = ? LIMIT 1")?;
        let token_iter = stmt.query(params![symbol])?.next()?.unwrap();

        for token_row in token_iter {
            let token = token_row?;
            let token_network_query = self.db.prepare("SELECT * FROM TokenNetworks WHERE id IN (?)")?;
            let token_networks_iter = token_network_query.query_map(params![token.token_networks_ids], |row| {
                Ok(DbTokenNetwork {
                    id: row.get(0)?,
                    network: row.get(1)?,
                    address: row.get(2)?,
                })
            })?;

            for token_network_row in token_networks_iter {
                let token_network = token_network_row?;
                if token_network.network == network as i64 {
                    return Ok(Some((token, token_network)));
                }
            }
        }
        Ok(None)
    }

    pub fn get_token_by_address(
        &self,
        address: &str,
        network: CryptoNetwork,
    ) -> Result<Option<(DbCryptoToken, DbTokenNetwork)>> {
        let network_id = self.get_network(network)?.map_or(0, |n| n.id);
        let mut stmt = self
            .db
            .prepare("SELECT * FROM TokenNetworks WHERE address = ? AND networkId = ?")?;
        let token_network_iter = stmt.query_map(params![address, network_id], |row| {
            Ok(DbTokenNetwork {
                id: row.get(0)?,
                network: row.get(1)?,
                address: row.get(2)?,
            })
        })?;

        for token_network_row in token_network_iter {
            let token_network = token_network_row?;
            let mut stmt = self
                .db
                .prepare("SELECT * FROM Tokens WHERE instr(tokenNetworksIds, ?) > 0")?;
            let token_iter = stmt.query_map(params![token_network.id], |row| {
                Ok(DbCryptoToken {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    symbol: row.get(2)?,
                    decimals: row.get(3)?,
                    token_networks_ids: row.get(4)?,
                })
            })?;

            for token_row in token_iter {
                let token = token_row?;
                if token.symbol == address {
                    return Ok(Some((token, token_network)));
                }
            }
        }
        Ok(None)
    }

    pub fn add_token(&self, token: CryptoToken) -> Result<(), Error> {
        let network_name = CryptoNetwork::Eth.to_string();
        let already_exists = self.get_token(&token.symbol, token.network)?.is_some();
        if already_exists {
            return Err(Error::new(
                ErrorKind::Other,
                format!("Token '{}' on '{}' already exists", token.symbol, network_name),
            ));
        }

        let network_id = self
            .get_network(token.network)?
            .ok_or_else(|| Error::new(ErrorKind::Other, format!("Network '{}' does not exist", network_name)))?
            .id;

        let token_network_exists = self
            .db
            .prepare("SELECT * FROM TokenNetworks WHERE networkId = ? AND address = ? LIMIT 1")?;
        let token_network_exist: bool = token_network_exists.exists(&[&network_id, &token.address])?;
        if token_network_exist {
            return Err(Error::new(
                ErrorKind::Other,
                format!("Token '{}' on '{}' already exists", token.symbol, network_name),
            ));
        }

        let token_network_insert = self
            .db
            .prepare("INSERT INTO TokenNetworks (networkId, address) VALUES (?, ?)")?;
        let added_token_network_id = token_network_insert.insert(&[&network_id, &token.address])?;

        let token_insert = self
            .db
            .prepare("INSERT INTO Tokens (name, symbol, decimals, tokenNetworksIds) VALUES (?, ?, ?, ?)")?;
        let token_network_ids = format!("{},", added_token_network_id);
        token_insert.execute(&[&token.name, &token.symbol, &token.decimals, &token_network_ids])?;

        Ok(())
    }

    pub fn get_pool(
        &self,
        token_a: &str,
        token_b: &str,
        network: CryptoNetwork,
        dex: &IDexProtocol,
    ) -> Result<Option<DbPool>, Error> {
        if token_a == token_b {
            return Err(Error::new(ErrorKind::Other, "TokenA and TokenB cannot be the same"));
        }

        let db_dex = self.get_dex(&dex.name)?;
        let db_dex = db_dex.ok_or_else(|| {
            Error::new(
                ErrorKind::Other,
                format!("Dex '{}' not found in the database", dex.name),
            )
        })?;

        let db_network = self.get_network(network)?;
        let db_network = db_network.ok_or_else(|| {
            Error::new(
                ErrorKind::Other,
                format!("Network '{}' not found in the database", network),
            )
        })?;

        let token_a_id = self.get_token_by_address(token_a, network)?.map_or(0, |t| t.0.id);
        let token_b_id = self.get_token_by_address(token_b, network)?.map_or(0, |t| t.0.id);

        let mut stmt = self.db.prepare(
            "SELECT * FROM DexPools WHERE ((token0Id = ?1 AND token1Id = ?2) OR (token1Id = ?1 AND token0Id = ?2)) AND dexId = ?3 AND networkId = ?4 LIMIT 1"
        )?;
        let mut pool_iter = stmt.query_map(params![token_a_id, token_b_id, db_dex.id, db_network.id], |row| {
            Ok(DbPool {
                id: row.get(0)?,
                dex_id: row.get(1)?,
                network_id: row.get(2)?,
                address: row.get(3)?,
                token0_id: row.get(4)?,
                token1_id: row.get(5)?,
            })
        })?;

        if let Some(pool_row) = pool_iter.next() {
            return Ok(Some(pool_row?));
        }

        Ok(None)
    }

    pub fn get_pool_by_address(
        &self,
        address: &str,
        network: CryptoNetwork,
        dex: &IDexProtocol,
    ) -> Result<Option<DbPool>, Error> {
        let db_network = self.get_network(network)?;
        let db_network = db_network.ok_or_else(|| {
            Error::new(
                ErrorKind::Other,
                format!("Network '{}' not found in the database", network),
            )
        })?;

        let db_dex = self.get_dex(&dex.name)?;
        let db_dex = db_dex.ok_or_else(|| {
            Error::new(
                ErrorKind::Other,
                format!("Dex '{}' not found in the database", dex.name),
            )
        })?;

        let mut stmt = self
            .db
            .prepare("SELECT * FROM DexPools WHERE address = ? AND networkId = ? AND dexId = ? LIMIT 1")?;
        let mut pool_iter = stmt.query_map(params![address, db_network.id, db_dex.id], |row| {
            Ok(DbPool {
                id: row.get(0)?,
                dex_id: row.get(1)?,
                network_id: row.get(2)?,
                address: row.get(3)?,
                token0_id: row.get(4)?,
                token1_id: row.get(5)?,
            })
        })?;

        if let Some(pool_row) = pool_iter.next() {
            return Ok(Some(pool_row?));
        }

        Ok(None)
    }

    pub fn add_pool(&self, dex: &IDexProtocol, pool: &DexTokenPoolBase) -> Result<(), Error> {
        if self
            .get_pool_by_address(&pool.contract_address, pool.network, dex)?
            .is_some()
        {
            return Err(Error::new(ErrorKind::Other, "Pool already exists"));
        }

        let db_dex = self.get_dex(&dex.name)?;
        let db_dex = db_dex.ok_or_else(|| Error::new(ErrorKind::Other, format!("Dex '{}' not found", dex.name)))?;

        self.add_pool_by_dex_id(db_dex.id, pool)?;

        Ok(())
    }

    pub fn add_empty_pool(
        &self,
        dex: &IDexProtocol,
        network: CryptoNetwork,
        token_a: &CryptoToken,
        token_b: &CryptoToken,
    ) -> Result<(), Error> {
        if self
            .get_pool(&token_a.address, &token_b.address, network, dex)?
            .is_some()
        {
            return Err(Error::new(ErrorKind::Other, "Pool already exists"));
        }

        let db_dex = self.get_dex(&dex.name)?;
        let db_dex = db_dex.ok_or_else(|| Error::new(ErrorKind::Other, format!("Dex '{}' not found", dex.name)))?;

        self.add_pool_by_dex_id_and_tokens(db_dex.id, network, "", token_a, token_b)?;

        Ok(())
    }

    pub fn get_dex_network(&self, dex: &DbDex, network: CryptoNetwork) -> Result<Option<DbDexNetwork>, Error> {
        let db_network = self.get_network(network)?;
        let db_network = db_network.ok_or_else(|| {
            Error::new(
                ErrorKind::Other,
                format!("Network '{}' not found in the database", network),
            )
        })?;

        let mut stmt = self
            .db
            .prepare("SELECT * FROM DexNetworks WHERE dexId = ? AND networkId = ? LIMIT 1")?;
        let mut dex_network_iter = stmt.query_map(params![dex.id, db_network.id], |row| {
            Ok(DbDexNetwork {
                id: row.get(0)?,
                dex_id: row.get(1)?,
                network_id: row.get(2)?,
                options: row.get(3)?,
            })
        })?;

        if let Some(dex_network_row) = dex_network_iter.next() {
            return Ok(Some(dex_network_row?));
        }

        Ok(None)
    }

    pub fn get_dex_protocol(&self, dex: &DbDex) -> Result<Option<DbDexProtocol>, Error> {
        let mut stmt = self.db.prepare("SELECT * FROM DexProtocols WHERE id = ? LIMIT 1")?;
        let mut dex_protocol_iter = stmt.query_map(params![dex.dex_protocol_id], |row| {
            Ok(DbDexProtocol {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })?;

        if let Some(dex_protocol_row) = dex_protocol_iter.next() {
            return Ok(Some(dex_protocol_row?));
        }

        Ok(None)
    }

    pub fn get_dex(&self, dex_name_or_id: &str) -> Result<Option<DbDex>, Error> {
        if let Ok(dex_id) = dex_name_or_id.parse::<i64>() {
            let mut stmt = self.db.prepare("SELECT * FROM Dexes WHERE id = ? LIMIT 1")?;
            let mut dex_iter = stmt.query_map(params![dex_id], |row| {
                Ok(DbDex {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    dex_protocol_id: row.get(2)?,
                    pools_ids: row.get(3)?,
                    fee_percentage: row.get(4)?,
                    dex_networks_ids: row.get(5)?,
                })
            })?;

            if let Some(dex_row) = dex_iter.next() {
                return Ok(Some(dex_row?));
            }
        } else {
            let mut stmt = self.db.prepare("SELECT * FROM Dexes WHERE name = ? LIMIT 1")?;
            let mut dex_iter = stmt.query_map(params![dex_name_or_id], |row| {
                Ok(DbDex {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    dex_protocol_id: row.get(2)?,
                    pools_ids: row.get(3)?,
                    fee_percentage: row.get(4)?,
                    dex_networks_ids: row.get(5)?,
                })
            })?;

            if let Some(dex_row) = dex_iter.next() {
                return Ok(Some(dex_row?));
            }
        }

        Ok(None)
    }

    pub fn get_dexes(&self) -> Result<Vec<DbDex>, Error> {
        let mut stmt = self.db.prepare("SELECT * FROM Dexes")?;
        let dex_iter = stmt.query_map([], |row| {
            Ok(DbDex {
                id: row.get(0)?,
                name: row.get(1)?,
                dex_protocol_id: row.get(2)?,
                pools_ids: row.get(3)?,
                fee_percentage: row.get(4)?,
                dex_networks_ids: row.get(5)?,
            })
        })?;

        let mut dexes = Vec::new();
        for dex in dex_iter {
            dexes.push(dex?);
        }

        Ok(dexes)
    }

    pub fn add_dex(&self, dex: &IDexProtocol) -> Result<(), Error> {
        let dex_protocol_id = self.get_dex_protocol_id(dex)?;

        let mut dex_networks_ids = Vec::new();
        for pool in &dex.pools {
            let pool_network_id = self.get_network_id(pool.network)?;
            let dex_network_id = self.add_dex_network(pool_network_id, &dex.options)?;

            dex_networks_ids.push(dex_network_id);
            self.add_pool(dex.id, pool)?;
        }

        let mut stmt = self
            .db
            .prepare("INSERT INTO Dexes (name, dexProtocolId, feePercentage, dexNetworksIds) VALUES (?, ?, ?, ?)")?;
        stmt.execute(params![
            dex.name,
            dex_protocol_id,
            dex.get_swap_fee(None).percentage,
            dex_networks_ids.join(",")
        ])?;

        Ok(())
    }

    pub fn get_provider_network(
        &self,
        provider: &IDbProvider,
        network: CryptoNetwork,
    ) -> Result<Option<IDbProviderNetwork>, Error> {
        let db_network = self.get_network(network)?;
        let db_network = db_network.ok_or_else(|| {
            Error::new(
                ErrorKind::Other,
                format!("Network '{}' not found in the database", network),
            )
        })?;

        let mut stmt = self
            .db
            .prepare("SELECT * FROM ProviderNetworks WHERE providerId = ? AND networkId = ? LIMIT 1")?;
        let mut provider_network_iter = stmt.query_map(params![provider.id, db_network.id], |row| {
            Ok(IDbProviderNetwork {
                id: row.get(0)?,
                provider_id: row.get(1)?,
                network_id: row.get(2)?,
                subdomain: row.get(3)?,
                http_api_key: row.get(4)?,
                ws_api_key: row.get(5)?,
            })
        })?;

        if let Some(provider_network_row) = provider_network_iter.next() {
            return Ok(Some(provider_network_row?));
        }

        Ok(None)
    }

    pub fn get_provider(&self, name: &str) -> Result<Option<IDbProvider>, Error> {
        let mut stmt = self.db.prepare("SELECT * FROM Providers WHERE name = ? LIMIT 1")?;
        let mut provider_iter = stmt.query_map(params![name], |row| {
            Ok(IDbProvider {
                id: row.get(0)?,
                name: row.get(1)?,
                http: row.get(2)?,
                websocket: row.get(3)?,
                providers_networks_ids: row.get(4)?,
            })
        })?;

        if let Some(provider_row) = provider_iter.next() {
            return Ok(Some(provider_row?));
        }

        Ok(None)
    }

    pub fn get_providers(&self) -> Result<Vec<IDbProvider>, Error> {
        let mut stmt = self.db.prepare("SELECT * FROM Providers")?;
        let provider_iter = stmt.query_map([], |row| {
            Ok(IDbProvider {
                id: row.get(0)?,
                name: row.get(1)?,
                http: row.get(2)?,
                websocket: row.get(3)?,
                providers_networks_ids: row.get(4)?,
            })
        })?;

        let mut providers = Vec::new();
        for provider in provider_iter {
            providers.push(provider?);
        }

        Ok(providers)
    }
}
*/