use std::collections::BTreeMap;
use std::cmp::Ord;
use std::sync::{RwLock, Arc};

use transaction::Transaction;
use error::{Error, ErrorKind, Result};

#[derive(Debug)]
pub struct Database<K, V>
    where K: Ord
{
    store: Arc<BTreeMap<K, V>>,
    txn_mut: RwLock<Transaction<K, V>>,
    closed: bool,
}

impl<K, V> Database<K, V>
    where K: Ord
{
    pub fn new() -> Result<Database<K, V>> {
        let store = Arc::new(BTreeMap::new());
        Ok(Database {
            store: store.clone(),
            txn_mut: RwLock::new(Transaction { store: store.clone() }),
            closed: false,
        })
    }

    pub fn read<F>(&self, f: F) -> Result<()>
        where F: Fn(&Transaction<K, V>) -> Result<()>
    {
        match self.txn_mut.read() {
            Ok(store) => f(&store),
            Err(_) => unreachable!(),
        }
    }

    pub fn update<F>(&self, f: F) -> Result<()>
        where F: Fn(&Transaction<K, V>) -> Result<()>
    {
        match self.txn_mut.write() {
            Ok(store) => f(&store),
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
    fn test_read() {
        let db = &Database::<String, String>::new().unwrap();
        assert!(db.read(|txn| -> Result<()> { Ok(()) }).is_ok())
    }
}