use std::sync::*;
use transaction::*;
use error::*;
use persist::*;

pub struct Database {
    txn_mut: RwLock<Transaction>,
    closed: bool,
    persist_store: Box<Persistable>,
}

impl Database {
    pub fn new(persist_type: PersistType) -> Result<Database> {
        let mut persist_store: Box<Persistable> = match persist_type {
            PersistType::Memory => Box::new(MemoryStore::default()),
            PersistType::File(path) => Box::new(FileStore::new(path)?),
        };

        Ok(Database {
            txn_mut: RwLock::new(Transaction::new(persist_store.load()?)),
            closed: false,
            persist_store: persist_store,
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
        let db = Database::new(PersistType::Memory).unwrap();
        assert_eq!(false, db.closed);
    }

    #[test]
    fn test_close() {
        let mut db = Database::new(PersistType::Memory).unwrap();
        assert!(db.close().is_ok());
        assert!(db.close().is_err());
        assert!(db.close().is_err());
    }
}
