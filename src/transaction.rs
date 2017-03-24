use std::collections::BTreeMap;
use data::Serializable;

#[derive(Debug)]
struct Item<K, V>
    where K: Into<String> + Ord + Clone,
          V: Serializable
{
    key: K,
    value: Option<V>,
}

impl<K, V> Item<K, V>
    where K: Into<String> + Ord + Clone,
          V: Serializable
{
    fn new(k: K, v: Option<V>) -> Item<K, V> {
        Item { key: k, value: v }
    }
}

pub trait ReadTransaction<K, V>
    where K: Into<String> + Ord + Clone,
          V: Serializable
{
    fn get(&self, key: K) -> Option<&V>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}

pub trait WriteTransaction<K, V>: ReadTransaction<K, V>
    where K: Into<String> + Ord + Clone,
          V: Serializable
{
    fn update(&mut self, key: K, value: V) -> Option<V>;
    fn remove(&mut self, key: K) -> Option<V>;
    fn remove_all(&mut self);
}

#[derive(Debug)]
pub struct Transaction<K, V>
    where K: Into<String> + Ord + Clone,
          V: Serializable
{
    store: BTreeMap<K, V>,
    backup_store: Option<BTreeMap<K, V>>,
    rollback_items: Vec<Item<K, V>>,
}

impl<K, V> Transaction<K, V>
    where K: Into<String> + Ord + Clone,
          V: Serializable
{
    pub fn new(store: BTreeMap<K, V>) -> Transaction<K, V> {
        Transaction {
            store: store,
            backup_store: None,
            rollback_items: Vec::new(),
        }
    }

    pub fn commit(&mut self) {
        self.rollback_items.clear();
        self.backup_store = None;
    }

    pub fn rollback(&mut self) {
        if self.backup_store.is_some() {
            self.store = self.backup_store.clone().unwrap();
            self.backup_store = None;
        }
        for item in &self.rollback_items {
            if item.value.is_none() {
                self.store.remove(&item.key.clone());
            } else {
                self.store
                    .insert(item.key.clone(), item.value.clone().unwrap());
            }
        }
    }
}

impl<K, V> ReadTransaction<K, V> for Transaction<K, V>
    where K: Into<String> + Ord + Clone,
          V: Serializable
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
    where K: Into<String> + Ord + Clone,
          V: Serializable
{
    fn update(&mut self, key: K, value: V) -> Option<V> {
        let previous_value = self.store.insert(key.clone(), value);
        if self.backup_store.is_none() {
            self.rollback_items.push(Item::new(key, previous_value.clone()));
        }
        previous_value
    }

    fn remove(&mut self, key: K) -> Option<V> {
        let previous_value = self.store.remove(&key);
        if self.backup_store.is_none() {
            self.rollback_items.push(Item::new(key, previous_value.clone()));
        }
        previous_value
    }

    fn remove_all(&mut self) {
        self.backup_store = Some(self.store.clone());
        self.store.clear();
    }
}
