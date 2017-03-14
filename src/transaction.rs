use std::collections::BTreeMap;
use std::cmp::Ord;
use error::Result;

#[derive(Debug, Clone, Copy)]
struct Item<K: Ord, V> {
    key: K,
    value: V,
}

pub trait ReadTransaction<K, V>
    where K: Ord
{
    fn get(&self, key: K) -> Option<&V>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}

pub trait WriteTransaction<K, V>: ReadTransaction<K, V>
    where K: Ord
{
    fn update(&mut self, key: K, value: V) -> Option<V>;
    fn remove(&mut self, key: K) -> Option<V>;
    fn remove_all(&mut self);
}

#[derive(Debug)]
pub struct Transaction<K: Ord, V> {
    pub store: Box<BTreeMap<K, V>>,

    rollback_items: Box<Vec<Item<K, V>>>,
}

impl<K, V> Transaction<K, V>
    where K: Ord
{
    pub fn new(store: Box<BTreeMap<K, V>>) -> Transaction<K, V> {
        Transaction {
            store: store,
            rollback_items: Box::new(Vec::new()),
        }
    }

    pub fn commit(&self) -> Result<()> {
        unimplemented!()
    }

    pub fn rollback(&mut self) -> Result<()> {
        for item in self.rollback_items.into_iter() {
            self.remove(item.key);
        }

        Ok(())
    }
}

impl<K, V> ReadTransaction<K, V> for Transaction<K, V>
    where K: Ord
{
    fn get(&self, key: K) -> Option<&V> {
        self.store.get(&key)
    }

    fn len(&self) -> usize {
        self.store.len()
    }

    fn is_empty(&self) -> bool {
        self.store.is_empty()
    }
}

impl<K, V> WriteTransaction<K, V> for Transaction<K, V>
    where K: Ord
{
    fn update(&mut self, key: K, value: V) -> Option<V> {
        self.store.insert(key, value)
    }

    fn remove(&mut self, key: K) -> Option<V> {
        self.store.remove(&key)
    }

    fn remove_all(&mut self) {
        self.store.clear();
    }
}
