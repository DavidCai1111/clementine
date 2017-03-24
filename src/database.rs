use std::collections::BTreeMap;
use std::sync::RwLock;
use transaction::{Transaction, ReadTransaction, WriteTransaction};
use error::{Error, ErrorKind, Result};
use data::Serializable;

#[derive(Debug)]
pub struct Database<K, V>
    where K: Into<String> + Ord + Clone,
          V: Serializable
{
    txn_mut: RwLock<Transaction<K, V>>,
    closed: bool,
}

impl<K, V> Database<K, V>
    where K: Into<String> + Ord + Clone,
          V: Serializable
{
    pub fn new() -> Result<Database<K, V>> {
        Ok(Database {
            txn_mut: RwLock::new(Transaction::new(BTreeMap::new())),
            closed: false,
        })
    }

    pub fn read<F>(&self, f: F) -> Result<()>
        where F: Fn(&ReadTransaction<K, V>) -> Result<()>
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
        where F: Fn(&mut WriteTransaction<K, V>) -> Result<()>
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

impl<K, V> Drop for Database<K, V>
    where K: Into<String> + Ord + Clone,
          V: Serializable
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
    use data::Data;

    #[test]
    fn test_new() {
        let db: Database<String, Data> = Database::new().unwrap();
        assert_eq!(false, db.closed);
    }

    #[test]
    fn test_close() {
        let mut db: Database<String, Data> = Database::new().unwrap();
        assert!(db.close().is_ok());
        assert!(db.close().is_err());
        assert!(db.close().is_err());
    }
}
