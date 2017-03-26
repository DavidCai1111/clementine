use std::collections::BTreeMap;
use data::Data;
use error::Result;

#[derive(Debug)]
pub enum PersistType {
    Memory,
    File(String),
}

pub trait Persistable<K>
    where K: Into<String> + Ord + Clone
{
    fn set(&mut self, Data) -> Result<()>;
    fn remove(&mut self, Data) -> Result<()>;
    fn load(&self) -> Result<BTreeMap<K, Data>>;
    fn clear(&mut self) -> Result<()>;
}

#[derive(Debug, Default)]
pub struct MemoryStore {}

impl<K> Persistable<K> for MemoryStore
    where K: Into<String> + Ord + Clone
{
    fn set(&mut self, _: Data) -> Result<()> {
        Ok(())
    }

    fn remove(&mut self, _: Data) -> Result<()> {
        Ok(())
    }

    fn load(&self) -> Result<BTreeMap<K, Data>> {
        Ok(BTreeMap::new())
    }

    fn clear(&mut self) -> Result<()> {
        Ok(())
    }
}
