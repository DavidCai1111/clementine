use std::collections::*;
use data::*;

#[derive(Debug)]
struct Item {
    key: String,
    value: Option<Data>,
}

impl Item {
    fn new(k: String, v: Option<Data>) -> Item {
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
pub struct Transaction {
    store: BTreeMap<String, Data>,
    backup_store: Option<BTreeMap<String, Data>>,
    rollback_items: Vec<Item>,
}

impl Transaction {
    pub fn new(store: BTreeMap<String, Data>) -> Transaction {
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

impl<K> ReadTransaction<K> for Transaction
    where K: Into<String> + Ord + Clone
{
    fn get(&self, key: K) -> Option<&Data> {
        self.store.get(&key.into())
    }

    fn len(&self) -> usize {
        self.store.len()
    }

    fn is_empty(&self) -> bool {
        self.store.is_empty()
    }
}

impl<K> WriteTransaction<K> for Transaction
    where K: Into<String> + Ord + Clone
{
    fn update(&mut self, key: K, value: Data) -> Option<Data> {
        let previous_value = self.store.insert(key.clone().into(), value);
        if self.backup_store.is_none() {
            self.rollback_items.push(Item::new(key.into(), previous_value.clone()));
        }
        previous_value
    }

    fn remove(&mut self, key: K) -> Option<Data> {
        let previous_value = self.store.remove(&key.clone().into());
        if self.backup_store.is_none() {
            self.rollback_items.push(Item::new(key.into(), previous_value.clone()));
        }
        previous_value
    }

    fn remove_all(&mut self) {
        self.backup_store = Some(self.store.clone());
        self.store.clear();
    }
}
