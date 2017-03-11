use std::collections::BTreeMap;
use std::cmp::Ord;
use std::sync::{RwLock, Arc};
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
            Ok(store) => f(&*store),
            Err(_) => unreachable!(),
        }
    }

    pub fn update<F>(&self, f: F) -> Result<()>
        where F: Fn(&mut WriteTransaction<K, V>) -> Result<()>
    {
        match self.txn_mut.write() {
            Ok(mut store) => f(&mut *store),
            Err(_) => unreachable!(),
        }
    }

    pub fn close(&mut self) -> Result<()> {
        if self.closed {
            return Err(Error::new(ErrorKind::DataBaseClosed));
        }
        self.closed = true;
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
                assert_eq!(true, txn.update("k1", "v1").unwrap().is_none());
                assert_eq!("v1", *txn.get("k1").unwrap());
                Ok(())
            })
            .is_ok());
    }
}
