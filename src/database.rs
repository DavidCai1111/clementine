use std::sync::*;
use std::default::*;
use transaction::*;
use error::*;
use persist::*;

pub struct Database {
    pub flushes: i32,

    txn_mut: RwLock<Transaction>,
    sync_policy: SyncPolicy,
    closed: bool,
}

pub struct Config {
    persist_type: PersistType,
    sync_policy: SyncPolicy,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            persist_type: PersistType::Memory,
            sync_policy: SyncPolicy::Never,
        }
    }
}

impl Database {
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

    pub fn update<F, K>(&self, f: F) -> Result<()>
        where F: Fn(&mut WriteTransaction<K>) -> Result<()>,
              K: Into<String> + Ord + Clone
    {
        let mut store = self.txn_mut.write()?;
        if self.closed {
            return Err(Error::new(ErrorKind::DataBaseClosed));
        }
        if f(&mut *store).is_err() {
            store.rollback();
        }
        store.commit();
        Ok(())
    }

    pub fn save(&mut self) -> Result<()> {
        self.txn_mut.write()?.save()?;
        self.flushes += 1;
        Ok(())
    }

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
