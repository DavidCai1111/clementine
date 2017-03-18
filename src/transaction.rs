use std::collections::BTreeMap;
use error::Result;

#[derive(Debug)]
struct Item<S: Into<String> + Ord + Clone> {
    key: S,
    value: S,
}

pub trait ReadTransaction<S: Into<String> + Ord + Clone> {
    fn get(&self, key: S) -> Option<&S>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}

pub trait WriteTransaction<S: Into<String> + Ord + Clone>: ReadTransaction<S> {
    fn update(&mut self, key: S, value: S) -> Option<S>;
    fn remove(&mut self, key: &S) -> Option<S>;
    fn remove_all(&mut self);
}

#[derive(Debug)]
pub struct Transaction<S: Into<String> + Ord + Clone> {
    pub store: Box<BTreeMap<S, S>>,

    rollback_items: Vec<Item<S>>,
}

impl<S: Into<String> + Ord + Clone> Transaction<S> {
    pub fn new(store: Box<BTreeMap<S, S>>) -> Transaction<S> {
        Transaction {
            store: store,
            rollback_items: Vec::new(),
        }
    }

    pub fn commit(&self) -> Result<()> {
        unimplemented!()
    }

    pub fn rollback(&mut self) -> Result<()> {
        for item in &self.rollback_items {
            Self::update_item(&mut self.store, item);
        }

        Ok(())
    }

    fn update_item(store: &mut BTreeMap<S, S>, item: &Item<S>) {
        store.insert(item.key.clone(), item.value.clone());
    }
}

impl<S: Into<String> + Ord + Clone> ReadTransaction<S> for Transaction<S> {
    fn get(&self, key: S) -> Option<&S> {
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

impl<S: Into<String> + Ord + Clone> WriteTransaction<S> for Transaction<S> {
    fn update(&mut self, key: S, value: S) -> Option<S> {
        self.store.insert(key, value)
    }

    fn remove(&mut self, key: &S) -> Option<S> {
        self.store.remove(key)
    }

    fn remove_all(&mut self) {
        self.store.clear();
    }
}
