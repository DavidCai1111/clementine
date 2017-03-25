use std::collections::BTreeMap;
use std::sync::RwLock;
use transaction::{Transaction, ReadTransaction, WriteTransaction};
use error::{Error, ErrorKind, Result};
use persist::PersistType;

#[derive(Debug)]
pub struct Database<K>
    where K: Into<String> + Ord + Clone
{
    txn_mut: RwLock<Transaction<K>>,
    closed: bool,
}

impl<K> Database<K>
    where K: Into<String> + Ord + Clone
{
    pub fn new(persist_type: PersistType) -> Result<Database<K>> {
        Ok(Database {
            txn_mut: RwLock::new(Transaction::new(BTreeMap::new())),
            closed: false,
        })
    }

    pub fn read<F>(&self, f: F) -> Result<()>
        where F: Fn(&ReadTransaction<K>) -> Result<()>
    {
        match self.txn_mut.read() {
            Ok(store) => {
                if self.closed {
                    return Err(Error::new(ErrorKind::DataBaseClosed));
                }
                f(&*store)
            }
            Err(_) => unreachable!(),
        }
    }

    pub fn update<F>(&self, f: F) -> Result<()>
        where F: Fn(&mut WriteTransaction<K>) -> Result<()>
    {
        match self.txn_mut.write() {
            Ok(mut store) => {
                if self.closed {
                    return Err(Error::new(ErrorKind::DataBaseClosed));
                }

                if f(&mut *store).is_err() {
                    store.rollback();
                }
                store.commit();
                Ok(())
            }
            Err(_) => unreachable!(),
        }
    }

    pub fn close(&mut self) -> Result<()> {
        if self.closed {
            return Err(Error::new(ErrorKind::DataBaseClosed));
        }

        match self.txn_mut.write() {
            Ok(_) => self.closed = true,
            Err(_) => unreachable!(),
        }
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
