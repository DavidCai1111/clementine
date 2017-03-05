use std::collections::BTreeMap;
use std::cmp::Ord;
use std::sync::Arc;
use error::Result;

pub trait ReadTransaction<K, V>
    where K: Ord
{
    fn get(&self, key: K) -> Result<V>;
    fn len(&self) -> usize;
}

pub trait WriteTransaction<K, V>
    where K: Ord
{
    fn update(&self, key: K, value: V) -> Result<V>;
    fn remove(&self, key: K) -> Result<V>;
}

#[derive(Debug)]
pub struct Transaction<K: Ord, V> {
    pub store: Arc<BTreeMap<K, V>>,
}

impl<K, V> Transaction<K, V>
    where K: Ord
{
    pub fn new(store: Arc<BTreeMap<K, V>>) -> Transaction<K, V> {
        Transaction { store: store }
    }

    fn commit() -> Result<()> {
        unimplemented!()
    }

    fn rollback() -> Result<()> {
        unimplemented!()
    }
}

impl<K, V> ReadTransaction<K, V> for Transaction<K, V>
    where K: Ord
{
    fn get(&self, key: K) -> Result<V> {
        unimplemented!()
    }

    fn len(&self) -> usize {
        self.store.len()
    }
}

impl<K, V> WriteTransaction<K, V> for Transaction<K, V>
    where K: Ord
{
    fn update(&self, key: K, value: V) -> Result<V> {
        unimplemented!()
    }

    fn remove(&self, key: K) -> Result<V> {
        unimplemented!()
    }
}
