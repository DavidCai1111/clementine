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

    pub fn read() -> Result<()> {
        unimplemented!()
    }

    pub fn update() -> Result<()> {
        unimplemented!()
    }

    pub fn close(&self) -> Result<()> {
        if self.closed {
            return Err(Error::new(ErrorKind::DataBaseClosed));
        }
        Ok(())
    }
}
