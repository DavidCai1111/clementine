use std::collections::*;
use std::fs;
use std::io::{Write, BufReader, Read};
use data::*;
use error::*;

const CR: &'static str = "\r";
const LF: &'static str = "\n";
const CRLF: &'static str = "\r\n";
const SET_PREFIX: &'static str = "$";
const REMOVE_PREFIX: &'static str = "#";

macro_rules! serialize_set_template { () => ("{prefix}{key_len}{crlf}{key}{val_len}{crlf}{value}") }
macro_rules! serialize_remove_template { () => ("{prefix}{key_len}{crlf}{key}") }

#[derive(Debug, PartialEq)]
pub enum SyncPolicy {
    Never,
    Always,
}

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

#[derive(Debug, PartialEq)]
enum LoadState {
    Empty,
    BeforeSetKeyCR,
    BeforeSetKeyLF,
    GetSetKey(usize),
    BeforeSetValCR,
    BeforeSetValLF,
    GetSetVal(usize),
    BeforeRemoveKeyCR,
    BeforeRemoveKeyLF,
    GetRemoveKey(usize),
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
               file: fs::OpenOptions::new()
                   .create(true)
                   .truncate(true)
                   .write(true)
                   .read(true)
                   .open(path)?,
           })
    }
}

impl Persistable for FileStore {
    fn set(&mut self, key: String, data: Data) -> Result<()> {
        let value = data.into_string();
        Ok(write!(self.file,
                  serialize_set_template!(),
                  prefix = SET_PREFIX,
                  key_len = key.len(),
                  val_len = value.len(),
                  crlf = CRLF,
                  key = key,
                  value = value)?)
    }

    fn remove(&mut self, key: String) -> Result<()> {
        Ok(write!(self.file,
                  serialize_remove_template!(),
                  prefix = REMOVE_PREFIX,
                  key_len = key.len(),
                  crlf = CRLF,
                  key = key)?)
    }

    fn load(&mut self) -> Result<BTreeMap<String, Data>> {
        let mut btree: BTreeMap<String, Data> = BTreeMap::new();

        let mut buf_reader = BufReader::new(fs::File::open(&self.path)?);
        let mut content_buffer = Vec::new();

        buf_reader.read_to_end(&mut content_buffer)?;

        let content = String::from_utf8(content_buffer)?;

        if content.is_empty() {
            return Ok(btree);
        }

        let mut buffer = String::new();
        let mut cache = String::new();
        let mut state = LoadState::Empty;

        for ch in content.chars() {
            let char_string = ch.to_string();
            match state {
                LoadState::Empty => {
                    if char_string == SET_PREFIX {
                        state = LoadState::BeforeSetKeyCR;
                        continue;
                    }
                    if char_string == REMOVE_PREFIX {
                        state = LoadState::BeforeRemoveKeyCR;
                        continue;
                    }
                    return Err(Error::new(ErrorKind::InvalidSerializedString));
                }
                LoadState::BeforeSetKeyCR => {
                    if char_string == CR {
                        state = LoadState::BeforeSetKeyLF;
                        continue;
                    }
                    buffer.push(ch);
                }
                LoadState::BeforeSetKeyLF => {
                    if char_string != LF {
                        return Err(Error::new(ErrorKind::InvalidSerializedString));
                    }

                    let key_len = String::from(&buffer[0..buffer.len()]).parse()?;
                    state = LoadState::GetSetKey(key_len);
                    buffer.clear();
                    continue;
                }
                LoadState::GetSetKey(len) => {
                    if buffer.len() < len - 1 {
                        buffer.push(ch);
                        continue;
                    }
                    if buffer.len() > len - 1 {
                        unreachable!();
                    }

                    buffer.push(ch);
                    cache = buffer.clone();
                    buffer.clear();
                    state = LoadState::BeforeSetValCR;
                }
                LoadState::BeforeSetValCR => {
                    if char_string == CR {
                        state = LoadState::BeforeSetValLF;
                        continue;
                    }
                    buffer.push(ch);
                }
                LoadState::BeforeSetValLF => {
                    if char_string != LF {
                        return Err(Error::new(ErrorKind::InvalidSerializedString));
                    }

                    let val_len = String::from(&buffer[0..buffer.len()]).parse()?;
                    state = LoadState::GetSetVal(val_len);
                    buffer.clear();
                    continue;
                }
                LoadState::GetSetVal(len) => {
                    if buffer.len() < len - 1 {
                        buffer.push(ch);
                        continue;
                    }
                    if buffer.len() > len - 1 {
                        unreachable!();
                    }

                    buffer.push(ch);
                    btree.insert(cache.clone(), Data::try_from(buffer.clone())?);
                    cache.clear();
                    buffer.clear();
                    state = LoadState::Empty;
                }
                LoadState::BeforeRemoveKeyCR => {
                    if char_string == CR {
                        state = LoadState::BeforeRemoveKeyLF;
                        continue;
                    }
                    buffer.push(ch);
                }
                LoadState::BeforeRemoveKeyLF => {
                    if char_string != LF {
                        return Err(Error::new(ErrorKind::InvalidSerializedString));
                    }

                    let key_len = String::from(&buffer[0..buffer.len()]).parse()?;
                    state = LoadState::GetRemoveKey(key_len);
                    buffer.clear();
                    continue;
                }
                LoadState::GetRemoveKey(len) => {
                    if buffer.len() < len - 1 {
                        buffer.push(ch);
                        continue;
                    }
                    if buffer.len() > len - 1 {
                        unreachable!();
                    }

                    buffer.push(ch);
                    btree.remove(&buffer);
                    buffer.clear();
                    state = LoadState::Empty;
                }
            }
        }

        if state != LoadState::Empty {
            return Err(Error::new(ErrorKind::InvalidSerializedString));
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

    fn get_cdb_path(name: &str) -> String {
        let mut cdb_path = env::current_dir().unwrap();
        cdb_path.push(String::from("tests/") + name);
        String::from(cdb_path.as_path().to_str().unwrap())
    }

    #[test]
    fn test_new() {
        let mut store = FileStore::new(get_cdb_path("test_new.cdb")).unwrap();
        store.clear().unwrap();
        assert!(store.file.metadata().unwrap().is_file());
    }

    #[test]
    fn test_set() {
        let mut store = FileStore::new(get_cdb_path("test_set.cdb")).unwrap();
        store.clear().unwrap();

        store
            .set(String::from("key"), Data::String(String::from("value")))
            .unwrap();
        store
            .set(String::from("key"), Data::String(String::from("value")))
            .unwrap();

        let mut content = String::new();
        fs::File::open(get_cdb_path("test_set.cdb"))
            .unwrap()
            .read_to_string(&mut content)
            .unwrap();
        assert_eq!("$3\r\nkey8\r\n+value\r\n$3\r\nkey8\r\n+value\r\n", content);
        store.clear().unwrap();
    }

    #[test]
    fn test_clear() {
        let mut store = FileStore::new(get_cdb_path("test_clear.cdb")).unwrap();
        store
            .set(String::from("key"), Data::String(String::from("value")))
            .unwrap();

        let mut content = String::new();
        fs::File::open(get_cdb_path("test_clear.cdb"))
            .unwrap()
            .read_to_string(&mut content)
            .unwrap();
        assert!(content.len() != 0);

        store.clear().unwrap();

        let mut content_after_clear = String::new();
        fs::File::open(get_cdb_path("test_clear.cdb"))
            .unwrap()
            .read_to_string(&mut content_after_clear)
            .unwrap();
        assert!(content_after_clear.len() == 0);
    }

    #[test]
    fn test_remove() {
        let mut store = FileStore::new(get_cdb_path("test_remove.cdb")).unwrap();
        store.clear().unwrap();

        store.remove(String::from("key1")).unwrap();
        store.remove(String::from("key2")).unwrap();

        let mut content = String::new();
        fs::File::open(get_cdb_path("test_remove.cdb"))
            .unwrap()
            .read_to_string(&mut content)
            .unwrap();
        assert_eq!("#4\r\nkey1#4\r\nkey2", content);
        store.clear().unwrap();
    }

    #[test]
    fn test_load() {
        let mut store = FileStore::new(get_cdb_path("test_load.cdb")).unwrap();
        write!(store.file, "$3\r\nkey8\r\n+value\r\n").unwrap();

        let tree = store.load().unwrap();
        assert_eq!(1, tree.len());
        store.clear().unwrap();
    }
}
