use std::collections::*;
use std::ops::Deref;
use std::cell::RefCell;
use persist::Persistable;
use data::*;
use error::*;

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

// A read-only transaction on the dasebase.
pub trait ReadTransaction<K>
    where K: Into<String> + Ord + Clone
{
    fn get(&self, key: K) -> Option<&Data>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn contains_key(&self, key: K) -> bool;
}

// An update transaction on the dasebase.
pub trait WriteTransaction<K>: ReadTransaction<K>
    where K: Into<String> + Ord + Clone
{
    fn update(&mut self, key: K, value: Data) -> Option<Data>;
    fn remove(&mut self, key: K) -> Option<Data>;
    fn clear(&mut self);
}

pub struct Transaction {
    store: BTreeMap<String, Data>,
    persist_store: RefCell<Box<Persistable>>,
    backup_store: Option<BTreeMap<String, Data>>,
    items_to_sync: Vec<Item>,
    rollback_items: Vec<Item>,
}

impl Transaction {
    pub fn new(store: BTreeMap<String, Data>, persist: Box<Persistable>) -> Transaction {
        Transaction {
            store: store,
            backup_store: None,
            persist_store: RefCell::new(persist),
            items_to_sync: Vec::new(),
            rollback_items: Vec::new(),
        }
    }

    pub fn save(&self) -> Result<()> {
        for item in &self.items_to_sync {
            match item.value {
                Some(ref value) => {
                    self.persist_store
                        .borrow_mut()
                        .set(item.key.clone(), value.clone())?
                }
                None => {
                    self.persist_store
                        .borrow_mut()
                        .remove(item.key.clone())?
                }
            }
        }

        Ok(())
    }

    pub fn commit(&mut self) {
        self.rollback_items.clear();
        self.backup_store = None;
    }

    pub fn rollback(&mut self) {
        if self.is_cleared() {
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

    fn record_rollback_item(&mut self, key: String, value: Option<Data>) {
        if self.backup_store.is_none() {
            self.rollback_items.push(Item::new(key, value));
        }
    }

    fn record_item_to_sync(&mut self, key: String, value: Option<Data>) {
        self.items_to_sync.push(Item::new(key, value));
    }

    fn is_cleared(&self) -> bool {
        self.backup_store.is_some()
    }
}

impl Deref for Transaction {
    type Target = BTreeMap<String, Data>;
    fn deref(&self) -> &BTreeMap<String, Data> {
        &self.store
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

    fn contains_key(&self, key: K) -> bool {
        self.store.contains_key(&key.into())
    }
}

impl<K> WriteTransaction<K> for Transaction
    where K: Into<String> + Ord + Clone
{
    fn update(&mut self, key: K, value: Data) -> Option<Data> {
        let previous_value = self.store.insert(key.clone().into(), value.clone());
        self.record_rollback_item(key.clone().into(), previous_value.clone());
        self.record_item_to_sync(key.into(), Some(value));

        previous_value
    }

    fn remove(&mut self, key: K) -> Option<Data> {
        let previous_value = self.store.remove(&key.clone().into());
        self.record_rollback_item(key.clone().into(), previous_value.clone());
        self.record_item_to_sync(key.into(), None);

        previous_value
    }

    fn clear(&mut self) {
        self.backup_store = Some(self.store.clone());
        self.store.clear();
    }
}
