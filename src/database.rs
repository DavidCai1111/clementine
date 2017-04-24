use std::sync::*;
use std::default::*;
use transaction::*;
use error::*;
use persist::*;

// The Clementine database.
pub struct Database {
    pub flushes: i32,

    txn_mut: RwLock<Transaction>,
    sync_policy: SyncPolicy,
    closed: bool,
}

// The configuration of the Database.
pub struct Config {
    // Whether the database should persist its data on
    // dist or just in memory.
    persist_type: PersistType,
    // The sync prolicy.
    sync_policy: SyncPolicy,
}

// The default configuration of the Database.
impl Default for Config {
    fn default() -> Self {
        Config {
            persist_type: PersistType::Memory,
            sync_policy: SyncPolicy::Never,
        }
    }
}

impl Database {
    // Return a new instance of the Database.
    pub fn new(config: Config) -> Result<Database> {
        let mut persist_store: Box<Persistable> = match config.persist_type {
            PersistType::Memory => Box::new(MemoryStore::default()),
            PersistType::File(path) => Box::new(FileStore::new(path)?),
        };

        Ok(Database {
               flushes: 0,
               txn_mut: RwLock::new(Transaction::new(persist_store.load()?, persist_store)),
               sync_policy: config.sync_policy,
               closed: false,
           })
    }

    // Start a read transaction.
    pub fn read<F, K>(&self, f: F) -> Result<()>
        where F: Fn(&ReadTransaction<K>) -> Result<()>,
              K: Into<String> + Ord + Clone
    {
        let store = self.txn_mut.read()?;
        if self.closed {
            return Err(Error::new(ErrorKind::DataBaseClosed));
        }
        f(&*store)
    }

    // Start an update transaction. If all operations in the
    // transaction are successful, then the result will be persisted
    // recording to the sync policy, otherwise will just rollback.
    pub fn update<F, K>(&self, f: F) -> Result<()>
        where F: Fn(&mut WriteTransaction<K>) -> Result<()>,
              K: Into<String> + Ord + Clone
    {
        let mut store = self.txn_mut.write()?;
        if self.closed {
            return Err(Error::new(ErrorKind::DataBaseClosed));
        }

        // If transaction is failed, do the rollback, else do the
        // sync job if specified.
        if f(&mut *store).is_err() {
            store.rollback();
        } else if self.sync_policy == SyncPolicy::Always {
            store.save()?;
        }

        store.commit();
        Ok(())
    }

    // Close this database.
    pub fn close(&mut self) -> Result<()> {
        if self.closed {
            return Err(Error::new(ErrorKind::DataBaseClosed));
        }
        let _ = self.txn_mut.write()?;
        self.closed = true;
        Ok(())
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        if !self.closed {
            self.close().unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let db = Database::new(Config::default()).unwrap();
        assert_eq!(false, db.closed);
        assert_eq!(SyncPolicy::Never, db.sync_policy)
    }

    #[test]
    fn test_close() {
        let mut db = Database::new(Config::default()).unwrap();
        assert!(db.close().is_ok());
        assert!(db.close().is_err());
        assert!(db.close().is_err());
    }
}
