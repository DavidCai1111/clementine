use std::collections::*;
use std::fs;
use std::io::{Write, BufReader, BufRead};
use data::*;
use error::*;

const CRLF: &'static str = "\r\n";
const SPACE: &'static str = " ";
const SET_PREFIX: &'static str = "$";
const REMOVE_PREFIX: &'static str = "#";

macro_rules! serialize_set_template { () => ("{prefix}{key}{space}{value}") }
macro_rules! serialize_remove_template { () => ("{prefix}{key}{crlf}") }

#[derive(Debug)]
pub enum PersistType {
    Memory,
    File(String),
}

pub trait Persistable {
    fn set(&mut self, String, Data) -> Result<()>;
    fn remove(&mut self, String) -> Result<()>;
    fn load(&mut self) -> Result<BTreeMap<String, Data>>;
    fn clear(&mut self) -> Result<()>;
}

#[derive(Debug)]
pub struct FileStore {
    path: String,
    file: fs::File,
}

impl FileStore {
    pub fn new(path: String) -> Result<FileStore> {
        Ok(FileStore {
            path: path.clone(),
            file: fs::OpenOptions::new().create(true)
                .truncate(true)
                .write(true)
                .read(true)
                .open(path)?,
        })
    }
}

impl FileStore {
    fn extract_set(line: String) -> Result<(String, Data)> {
        let delimiter_index = line.find(SPACE)
            .ok_or(Error::new(ErrorKind::InvalidSerializedString))?;

        let key = String::from(&line[SET_PREFIX.len()..delimiter_index]);
        let value = String::from(&line[delimiter_index + 1..line.len()]);

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
                  prefix = SET_PREFIX,
                  key = key,
                  space = SPACE,
                  value = data.into_string())?)
    }

    fn remove(&mut self, key: String) -> Result<()> {
        Ok(write!(self.file,
                  serialize_remove_template!(),
                  crlf = CRLF,
                  prefix = REMOVE_PREFIX,
                  key = key)?)
    }

    fn load(&mut self) -> Result<BTreeMap<String, Data>> {
        let mut btree: BTreeMap<String, Data> = BTreeMap::new();

        for line in BufReader::new(fs::File::open(&self.path)?).lines() {
            let line = line? + CRLF;
            if line.starts_with(SET_PREFIX) {
                let (key, value) = Self::extract_set(String::from(line))?;
                btree.insert(key, value);
            } else if line.starts_with(REMOVE_PREFIX) {
                btree.remove(&Self::extract_remove(String::from(line))?);
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

    fn load(&mut self) -> Result<BTreeMap<String, Data>> {
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
        let line = String::from("$key +value\r\n");
        let (key, value) = FileStore::extract_set(line).unwrap();
        assert_eq!("key", key);
        assert_eq!(Data::String(String::from("value")), value);
    }

    #[test]
    fn test_extract_set_int() {
        let line = String::from("$key :888\r\n");
        let (key, value) = FileStore::extract_set(line).unwrap();
        assert_eq!("key", key);
        assert_eq!(Data::Int(888), value);
    }

    #[test]
    fn test_extract_remove() {
        let line = String::from("#key\r\n");
        let key = FileStore::extract_remove(line).unwrap();
        assert_eq!("key", key);
    }

    #[test]
    fn test_new() {
        let mut cdb_path = env::current_dir().unwrap();
        cdb_path.push("tests/test_new.cdb");
        let path = String::from(cdb_path.as_path().to_str().unwrap());
        let mut store = FileStore::new(path).unwrap();
        store.clear().unwrap();
        assert!(store.file.metadata().unwrap().is_file());
    }

    #[test]
    fn test_set() {
        let mut cdb_path = env::current_dir().unwrap();
        cdb_path.push("tests/test_set.cdb");
        let path = String::from(cdb_path.as_path().to_str().unwrap());
        let mut store = FileStore::new(path.clone()).unwrap();
        store.clear().unwrap();

        store.set(String::from("key"), Data::String(String::from("value")))
            .unwrap();
        store.set(String::from("key"), Data::String(String::from("value")))
            .unwrap();

        let mut content = String::new();
        fs::File::open(path).unwrap().read_to_string(&mut content).unwrap();
        assert_eq!("$key +value\r\n$key +value\r\n", content);
        store.clear().unwrap();
    }

    #[test]
    fn test_clear() {
        let mut cdb_path = env::current_dir().unwrap();
        cdb_path.push("tests/test_clear.cdb");
        let path = String::from(cdb_path.as_path().to_str().unwrap());
        let mut store = FileStore::new(path.clone()).unwrap();
        store.set(String::from("key"), Data::String(String::from("value")))
            .unwrap();

        let mut content = String::new();
        fs::File::open(path.clone())
            .unwrap()
            .read_to_string(&mut content)
            .unwrap();
        assert!(content.len() != 0);

        store.clear().unwrap();

        let mut content_after_clear = String::new();
        fs::File::open(path)
            .unwrap()
            .read_to_string(&mut content_after_clear)
            .unwrap();
        assert!(content_after_clear.len() == 0);
    }

    #[test]
    fn test_remove() {
        let mut cdb_path = env::current_dir().unwrap();
        cdb_path.push("tests/test_remove.cdb");
        let path = String::from(cdb_path.as_path().to_str().unwrap());
        let mut store = FileStore::new(path.clone()).unwrap();
        store.clear().unwrap();

        store.remove(String::from("key1")).unwrap();
        store.remove(String::from("key2")).unwrap();

        let mut content = String::new();
        fs::File::open(path).unwrap().read_to_string(&mut content).unwrap();
        assert_eq!("#key1\r\n#key2\r\n", content);
        store.clear().unwrap();
    }

    #[test]
    fn test_load() {
        let mut cdb_path = env::current_dir().unwrap();
        cdb_path.push("tests/test_load.cdb");
        let path = String::from(cdb_path.as_path().to_str().unwrap());
        let mut store = FileStore::new(path.clone()).unwrap();

        write!(store.file, "$key +value\r\n").unwrap();

        let tree = store.load().unwrap();
        assert_eq!(1, tree.len());
        store.clear().unwrap();
    }
}
