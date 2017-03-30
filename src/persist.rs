use std::collections::*;
use std::fs;
use std::io::{Write, BufReader, BufRead};
use data::*;
use error::*;

static CRLF: &'static str = "\r\n";
static SET_PREFIX: &'static str = "SET";
static REMOVE_PREFIX: &'static str = "REMOVE";

macro_rules! serialize_set_template { () => ("{prefix}{key}{crlf}{value}{crlf}") }
macro_rules! serialize_remove_template { () => ("{prefix}{key}{crlf}") }

#[derive(Debug)]
pub enum PersistType {
    Memory,
    File(String),
}

pub trait Persistable {
    fn set(&mut self, String, Data) -> Result<()>;
    fn remove(&mut self, String) -> Result<()>;
    fn load(&self) -> Result<BTreeMap<String, Data>>;
    fn clear(&mut self) -> Result<()>;
}

#[derive(Debug)]
pub struct FileStore {
    file: fs::File,
}

impl FileStore {
    pub fn new(path: String) -> Result<FileStore> {
        Ok(FileStore { file: fs::File::create(path)? })
    }
}

impl FileStore {
    fn extract_set(&self, string: String) -> Result<(String, Data)> {
        let delimiter_index = string.find(CRLF)
            .ok_or(Error::new(ErrorKind::InvalidSerializedString))?;

        let key = String::from(&string[SET_PREFIX.len()..delimiter_index]);
        let value = String::from(&string[delimiter_index + 2..(string.len() - 1)]);

        Ok((key, Data::try_from(value)?))
    }
}

impl Persistable for FileStore {
    fn set(&mut self, key: String, data: Data) -> Result<()> {
        Ok(write!(self.file,
                  serialize_set_template!(),
                  crlf = CRLF,
                  prefix = SET_PREFIX,
                  key = key,
                  value = data.into_string())?)
    }

    fn remove(&mut self, key: String) -> Result<()> {
        Ok(write!(self.file,
                  serialize_remove_template!(),
                  crlf = CRLF,
                  prefix = REMOVE_PREFIX,
                  key = key)?)
    }

    fn load(&self) -> Result<BTreeMap<String, Data>> {
        let reader = BufReader::new(&self.file);
        let mut btree: BTreeMap<String, Data> = BTreeMap::new();

        for line in reader.lines() {
            let l = line?;
            if l.starts_with(SET_PREFIX) {
                let (key, value) = self.extract_set(l)?;
                btree.insert(key, value);
            } else if l.starts_with(REMOVE_PREFIX) {
                let csrf_index = l.find(CRLF);
                if csrf_index.is_none() {
                    return Err(Error::new(ErrorKind::InvalidSerializedString));
                }

                let key = l[REMOVE_PREFIX.len()..csrf_index.unwrap()].to_string();

                btree.remove(&key);
            } else {
                return Err(Error::new(ErrorKind::InvalidSerializedString));
            }
        }

        Ok(btree)
    }

    fn clear(&mut self) -> Result<()> {
        Ok(self.file.set_len(0)?)
    }
}

#[derive(Debug, Default)]
pub struct MemoryStore {}

impl Persistable for MemoryStore {
    fn set(&mut self, _: String, _: Data) -> Result<()> {
        Ok(())
    }

    fn remove(&mut self, _: String) -> Result<()> {
        Ok(())
    }

    fn load(&self) -> Result<BTreeMap<String, Data>> {
        Ok(BTreeMap::new())
    }

    fn clear(&mut self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod memory_store_tests {
    use super::*;

    #[test]
    fn test_set() {
        let mut store = MemoryStore::default();
        assert!(store.set(String::from("test"), Data::Int(1)).is_ok());
    }

    #[test]
    fn test_remove() {
        let mut store = MemoryStore::default();
        assert!(store.remove(String::from("test")).is_ok());
    }

    #[test]
    fn test_load() {
        assert!(MemoryStore::default().load().unwrap().is_empty());
    }

    #[test]
    fn test_clear() {
        let mut store = MemoryStore::default();
        assert!(store.clear().is_ok());
    }
}
