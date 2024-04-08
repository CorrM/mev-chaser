use std::sync::{Arc, RwLock};

use revm::db::{CacheDB, EmptyDB};
use revm::primitives::{Account, AccountInfo, Address, Bytecode, B256, U256};
use revm::{Database, DatabaseCommit, DatabaseRef};

/// A [Database] implementation that stores all state changes in memory and in a thread-safe way.
pub type SharedInMemoryDB = ThreadSafeCacheDB<EmptyDB>;

/// A [Database] implementation that stores all state changes in memory and in a thread-safe way.
///
/// This implementation wraps a [DatabaseRef] that is used to load data ([AccountInfo]).
///
/// Accounts and code are stored in two separate maps, the `accounts` map maps addresses to [DbAccount],
/// whereas contracts are identified by their code hash, and are stored in the `contracts` map.
/// The [DbAccount] holds the code hash of the contract, which is used to look up the contract in the `contracts` map.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ThreadSafeCacheDB<ExtDB>(pub Arc<RwLock<CacheDB<ExtDB>>>);

impl<ExtDB: Default> Default for ThreadSafeCacheDB<ExtDB> {
    fn default() -> Self {
        Self::new(ExtDB::default())
    }
}

impl<ExtDB> ThreadSafeCacheDB<ExtDB> {
    pub fn new(db: ExtDB) -> Self {
        Self {
            0: Arc::new(RwLock::new(CacheDB::new(db))),
        }
    }

    /// Inserts the account's code into the cache.
    ///
    /// Accounts objects and code are stored separately in the cache, this will take the code from the account and instead map it to the code hash.
    ///
    /// Note: This will not insert into the underlying external database.
    pub fn insert_contract(&mut self, account: &mut AccountInfo) {
        self.0.write().unwrap().insert_contract(account)
    }

    /// Insert account info but not override storage
    pub fn insert_account_info(&mut self, address: Address, info: AccountInfo) {
        self.0.write().unwrap().insert_account_info(address, info)
    }
}

impl<ExtDB: DatabaseRef> ThreadSafeCacheDB<ExtDB> {
    /// Returns the account for the given address.
    ///
    /// If the account was not found in the cache, it will be loaded from the underlying database.
    //pub fn load_account<'a>(&'a mut self, address: Address) -> Result<&'a mut DbAccount, ExtDB::Error> {
    //    let result = self.0.write().unwrap().load_account(address);
    //    result
    //}

    /// insert account storage without overriding account info
    pub fn insert_account_storage(&mut self, address: Address, slot: U256, value: U256) -> Result<(), ExtDB::Error> {
        self.0.write().unwrap().insert_account_storage(address, slot, value)
    }

    /// replace account storage without overriding account info
    pub fn replace_account_storage(
        &mut self,
        address: Address,
        storage: std::collections::HashMap<U256, U256>,
    ) -> Result<(), ExtDB::Error> {
        self.0.write().unwrap().replace_account_storage(address, storage)
    }
}

impl<ExtDB> DatabaseCommit for ThreadSafeCacheDB<ExtDB> {
    fn commit(&mut self, changes: std::collections::HashMap<Address, Account>) {
        self.0.write().unwrap().commit(changes)
    }
}

impl<ExtDB: DatabaseRef> Database for ThreadSafeCacheDB<ExtDB> {
    type Error = ExtDB::Error;

    fn basic(&mut self, address: Address) -> Result<Option<AccountInfo>, Self::Error> {
        self.0.write().unwrap().basic(address)
    }

    fn code_by_hash(&mut self, code_hash: B256) -> Result<Bytecode, Self::Error> {
        self.0.write().unwrap().code_by_hash(code_hash)
    }

    /// Get the value in an account's storage slot.
    ///
    /// It is assumed that account is already loaded.
    fn storage(&mut self, address: Address, index: U256) -> Result<U256, Self::Error> {
        self.0.write().unwrap().storage(address, index)
    }

    fn block_hash(&mut self, number: U256) -> Result<B256, Self::Error> {
        self.0.write().unwrap().block_hash(number)
    }
}

impl<ExtDB: DatabaseRef> DatabaseRef for ThreadSafeCacheDB<ExtDB> {
    type Error = ExtDB::Error;

    fn basic_ref(&self, address: Address) -> Result<Option<AccountInfo>, Self::Error> {
        self.0.read().unwrap().basic_ref(address)
    }

    fn code_by_hash_ref(&self, code_hash: B256) -> Result<Bytecode, Self::Error> {
        self.0.read().unwrap().code_by_hash_ref(code_hash)
    }

    fn storage_ref(&self, address: Address, index: U256) -> Result<U256, Self::Error> {
        self.0.read().unwrap().storage_ref(address, index)
    }

    fn block_hash_ref(&self, number: U256) -> Result<B256, Self::Error> {
        self.0.read().unwrap().block_hash_ref(number)
    }
}
