use std::collections::BTreeMap;
use error::Result;

#[derive(Debug)]
struct Item {
    key: String,
    value: String,
}

pub trait ReadTransaction {
    fn get(&self, key: String) -> Option<&str>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}

pub trait WriteTransaction: ReadTransaction {
    fn update(&mut self, key: String, value: String) -> Option<String>;
    fn remove(&mut self, key: &str) -> Option<String>;
    fn remove_all(&mut self);
}

#[derive(Debug)]
pub struct Transaction {
    pub store: Box<BTreeMap<String, String>>,

    rollback_items: Vec<Box<Item>>,
}

impl Transaction {
    pub fn new(store: Box<BTreeMap<String, String>>) -> Transaction {
        Transaction {
            store: store,
            rollback_items: Vec::new(),
        }
    }

    pub fn commit(&self) -> Result<()> {
        unimplemented!()
    }

    pub fn rollback(&mut self) -> Result<()> {
        // for item in &self.rollback_items {
        //     self.store.insert(item.key, item.value);
        // }

        Ok(())
    }
}

impl ReadTransaction for Transaction {
    fn get(&self, key: String) -> Option<&str> {
        match self.store.get(&key) {
            Some(value) => Some(&*value),
            None => None,
        }
    }

    fn len(&self) -> usize {
        self.store.len()
    }

    fn is_empty(&self) -> bool {
        self.store.is_empty()
    }
}

impl WriteTransaction for Transaction {
    fn update(&mut self, key: String, value: String) -> Option<String> {
        self.store.insert(key, value)
    }

    fn remove(&mut self, key: &str) -> Option<String> {
        self.store.remove(key)
    }

    fn remove_all(&mut self) {
        self.store.clear();
    }
}
