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
    pub file: fs::File,
}

impl FileStore {
    pub fn new(path: String) -> Result<FileStore> {
        Ok(FileStore { file: fs::File::create(path)? })
    }
}

impl FileStore {
    fn extract_set(line: String) -> Result<(String, Data)> {
        let delimiter_index = line.find(CRLF)
            .ok_or(Error::new(ErrorKind::InvalidSerializedString))?;

        let key = String::from(&line[SET_PREFIX.len()..delimiter_index]);
        let value = String::from(&line[delimiter_index + 2..line.len() - 2]);

        Ok((key, Data::try_from(value)?))
    }

    fn extract_remove(line: String) -> Result<String> {
        let delimiter_index = line.find(CRLF)
            .ok_or(Error::new(ErrorKind::InvalidSerializedString))?;

        Ok(String::from(&line[REMOVE_PREFIX.len()..delimiter_index]))
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

        for line_result in reader.lines() {
            let line = line_result?;
            if line.starts_with(SET_PREFIX) {
                let (key, value) = Self::extract_set(line)?;
                btree.insert(key, value);
            } else if line.starts_with(REMOVE_PREFIX) {
                btree.remove(&Self::extract_remove(line)?);
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

#[cfg(test)]
mod file_store_tests {
    use super::*;
    use std::env;
    use std::io::Read;

    #[test]
    fn test_extract_set_string() {
        let line = String::from("SETkey\r\n+value\r\n\r\n");
        let (key, value) = FileStore::extract_set(line).unwrap();
        assert_eq!("key", key);
        assert_eq!(Data::String(String::from("value")), value);
    }

    #[test]
    fn test_extract_set_int() {
        let line = String::from("SETkey\r\n:888\r\n\r\n");
        let (key, value) = FileStore::extract_set(line).unwrap();
        assert_eq!("key", key);
        assert_eq!(Data::Int(888), value);
    }

    #[test]
    fn test_extract_remove() {
        let line = String::from("REMOVEkey\r\n");
        let key = FileStore::extract_remove(line).unwrap();
        assert_eq!("key", key);
    }

    #[test]
    fn test_new() {
        let mut cdb_path = env::current_dir().unwrap();
        cdb_path.push("tests/test.cdb");
        let path = String::from(cdb_path.as_path().to_str().unwrap());
        let store = FileStore::new(path).unwrap();
        assert!(store.file.metadata().unwrap().is_file());
    }

    #[test]
    fn test_set() {
        let mut cdb_path = env::current_dir().unwrap();
        cdb_path.push("tests/test.cdb");
        let path = String::from(cdb_path.as_path().to_str().unwrap());
        let mut store = FileStore::new(path.clone()).unwrap();
        store.set(String::from("key"), Data::String(String::from("value")))
            .unwrap();
        store.set(String::from("key"), Data::String(String::from("value")))
            .unwrap();

        let mut content = String::new();
        fs::File::open(path).unwrap().read_to_string(&mut content).unwrap();
        assert_eq!("SETkey\r\n+value\r\n\r\nSETkey\r\n+value\r\n\r\n", content);
    }
}
