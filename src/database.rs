use std::collections::BTreeMap;
use std::sync::RwLock;
use std::cmp::Ord;
use error::{Error, ErrorKind, Result};
use transaction::Transaction;

#[derive(Debug)]
pub struct Database<K, V>
    where K: Ord
{
    store: BTreeMap<K, V>,
    closed: bool,
    txn_mut: RwLock<Transaction>,
}

impl<'a, K, V> Database<K, V>
    where K: Ord
{
    pub fn new() -> Result<Database<K, V>> {
        Ok(Database {
            store: BTreeMap::new(),
            closed: false,
            txn_mut: RwLock::new(Transaction {}),
        })
    }

    pub fn read() -> Result<()> {
        Ok(())
    }

    pub fn update() -> Result<()> {
        Ok(())
    }

    pub fn close(&self) -> Result<()> {
        if self.closed {
            return Err(Error::new(ErrorKind::DataBaseClosed));
        }
        Ok(())
    }
}
