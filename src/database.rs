use std::collections::BTreeMap;
use std::cmp::Ord;
use std::sync::RwLock;
use transaction::{Transaction, ReadTransaction, WriteTransaction};
use error::{Error, ErrorKind, Result};

#[derive(Debug)]
pub struct Database<K, V>
    where K: Ord
{
    txn_mut: RwLock<Transaction<K, V>>,
    closed: bool,
}

impl<K, V> Database<K, V>
    where K: Ord
{
    pub fn new() -> Result<Database<K, V>> {
        Ok(Database {
            txn_mut: RwLock::new(Transaction::new(Box::new(BTreeMap::new()))),
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
                // check the result and then commit or rollback
                if f(&mut *store).is_err() {
                    return store.rollback();
                }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let db = Database::<String, String>::new().unwrap();
        assert_eq!(false, db.closed);
    }

    #[test]
    fn test_close() {
        let db = &mut Database::<String, String>::new().unwrap();
        assert!(db.close().is_ok());
        assert!(db.close().is_err());
        assert!(db.close().is_err());
    }

    #[test]
    fn test_read_empty() {
        let db = &Database::<&str, &str>::new().unwrap();
        assert!(db.read(|txn| -> Result<()> {
                assert!(txn.get("not exist").is_none());
                Ok(())
            })
            .is_ok())
    }

    #[test]
    fn test_update() {
        let db = &Database::<&str, &str>::new().unwrap();
        assert!(db.update(|txn| -> Result<()> {
                assert_eq!(true, txn.update("k1", "v1").is_none());
                assert_eq!("v1", *txn.get("k1").unwrap());
                Ok(())
            })
            .is_ok());
    }

    #[test]
    fn test_remove() {
        let db = &Database::<&str, &str>::new().unwrap();
        assert!(db.update(|txn| -> Result<()> {
                assert_eq!(true, txn.update("k1", "v1").is_none());
                assert_eq!("v1", *txn.get("k1").unwrap());
                assert_eq!(true, txn.remove("k1").is_some());
                assert_eq!(true, txn.get("k1").is_none());
                Ok(())
            })
            .is_ok());
    }

    #[test]
    fn test_remove_all() {
        let db = &Database::<&str, &str>::new().unwrap();
        assert!(db.update(|txn| -> Result<()> {
                assert_eq!(true, txn.update("k1", "v1").is_none());
                assert_eq!(true, txn.update("k2", "v2").is_none());
                assert_eq!(true, txn.update("k3", "v3").is_none());
                assert_eq!(3, txn.len());
                txn.remove_all();
                assert_eq!(0, txn.len());
                assert_eq!(true, txn.is_empty());
                Ok(())
            })
            .is_ok());
    }
}
