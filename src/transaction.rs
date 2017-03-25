use std::collections::BTreeMap;
use data::Data;

#[derive(Debug)]
struct Item<K>
    where K: Into<String> + Ord + Clone
{
    key: K,
    value: Option<Data>,
}

impl<K> Item<K>
    where K: Into<String> + Ord + Clone
{
    fn new(k: K, v: Option<Data>) -> Item<K> {
        Item { key: k, value: v }
    }
}

pub trait ReadTransaction<K>
    where K: Into<String> + Ord + Clone
{
    fn get(&self, key: K) -> Option<&Data>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}

pub trait WriteTransaction<K>: ReadTransaction<K>
    where K: Into<String> + Ord + Clone
{
    fn update(&mut self, key: K, value: Data) -> Option<Data>;
    fn remove(&mut self, key: K) -> Option<Data>;
    fn remove_all(&mut self);
}

#[derive(Debug)]
pub struct Transaction<K>
    where K: Into<String> + Ord + Clone
{
    store: BTreeMap<K, Data>,
    backup_store: Option<BTreeMap<K, Data>>,
    rollback_items: Vec<Item<K>>,
}

impl<K> Transaction<K>
    where K: Into<String> + Ord + Clone
{
    pub fn new(store: BTreeMap<K, Data>) -> Transaction<K> {
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

impl<K> ReadTransaction<K> for Transaction<K>
    where K: Into<String> + Ord + Clone
{
    fn get(&self, key: K) -> Option<&Data> {
        self.store.get(&key)
    }

    fn len(&self) -> usize {
        self.store.len()
    }

    fn is_empty(&self) -> bool {
        self.store.is_empty()
    }
}

impl<K> WriteTransaction<K> for Transaction<K>
    where K: Into<String> + Ord + Clone
{
    fn update(&mut self, key: K, value: Data) -> Option<Data> {
        let previous_value = self.store.insert(key.clone(), value);
        if self.backup_store.is_none() {
            self.rollback_items.push(Item::new(key, previous_value.clone()));
        }
        previous_value
    }

    fn remove(&mut self, key: K) -> Option<Data> {
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
