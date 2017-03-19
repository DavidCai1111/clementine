use std::collections::BTreeMap;

#[derive(Debug)]
struct Item<S>
    where S: Into<String> + Ord + Clone
{
    key: S,
    value: Option<S>,
}

pub trait ReadTransaction<S>
    where S: Into<String> + Ord + Clone
{
    fn get(&self, key: S) -> Option<&S>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}

pub trait WriteTransaction<S>: ReadTransaction<S>
    where S: Into<String> + Ord + Clone
{
    fn update(&mut self, key: S, value: S) -> Option<S>;
    fn remove(&mut self, key: &S) -> Option<S>;
    fn remove_all(&mut self);
}

#[derive(Debug)]
pub struct Transaction<S>
    where S: Into<String> + Ord + Clone
{
    store: BTreeMap<S, S>,
    backup_store: Option<BTreeMap<S, S>>,
    rollback_items: Vec<Item<S>>,
}

impl<S> Transaction<S>
    where S: Into<String> + Ord + Clone
{
    pub fn new(store: BTreeMap<S, S>) -> Transaction<S> {
        Transaction {
            store: store,
            backup_store: None,
            rollback_items: Vec::new(),
        }
    }

    pub fn commit(&mut self) {
        self.rollback_items.clear();
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

impl<S> ReadTransaction<S> for Transaction<S>
    where S: Into<String> + Ord + Clone
{
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

impl<S> WriteTransaction<S> for Transaction<S>
    where S: Into<String> + Ord + Clone
{
    fn update(&mut self, key: S, value: S) -> Option<S> {
        let opt = self.store.insert(key.clone(), value);
        if self.backup_store.is_none() {
            self.rollback_items.push(Item {
                key: key.clone(),
                value: opt.clone(),
            });
        }
        opt
    }

    fn remove(&mut self, key: &S) -> Option<S> {
        let opt = self.store.remove(key);
        if self.backup_store.is_none() {
            self.rollback_items.push(Item {
                key: key.clone(),
                value: opt.clone(),
            });
        }
        opt
    }

    fn remove_all(&mut self) {
        self.backup_store = Some(self.store.clone());
        self.store.clear();
    }
}
