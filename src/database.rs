use std::sync::*;
use transaction::*;
use error::*;
use persist::*;

pub struct Database<K>
    where K: Into<String> + Ord + Clone
{
    txn_mut: RwLock<Transaction>,
    closed: bool,
    persist_store: Box<Persistable<K>>,
}

impl<K> Database<K>
    where K: Into<String> + Ord + Clone
{
    pub fn new(persist_type: PersistType) -> Result<Database<K>> {
        let persist_store: Box<Persistable<K>> = match persist_type {
            PersistType::Memory => Box::new(MemoryStore::default()),
            PersistType::File(path) => Box::new(FileStore::new(path)?),
        };

        Ok(Database {
            txn_mut: RwLock::new(Transaction::new(persist_store.load()?)),
            closed: false,
            persist_store: persist_store,
        })
    }

    pub fn read<F>(&self, f: F) -> Result<()>
        where F: Fn(&ReadTransaction<K>) -> Result<()>
    {
        let store = self.txn_mut.read()?;
        if self.closed {
            return Err(Error::new(ErrorKind::DataBaseClosed));
        }
        f(&*store)
    }

    pub fn update<F>(&self, f: F) -> Result<()>
        where F: Fn(&mut WriteTransaction<K>) -> Result<()>
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

impl<K> Drop for Database<K>
    where K: Into<String> + Ord + Clone
{
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
        let db: Database<String> = Database::new(PersistType::Memory).unwrap();
        assert_eq!(false, db.closed);
    }

    #[test]
    fn test_close() {
        let mut db: Database<String> = Database::new(PersistType::Memory).unwrap();
        assert!(db.close().is_ok());
        assert!(db.close().is_err());
        assert!(db.close().is_err());
    }
}
