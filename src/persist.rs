use std::collections::*;
use std::fs;
use std::io::{Write, BufReader, BufRead};
use data::*;
use error::*;

static CRLF: &'static str = "\r\n";
static SET_PREFIX: &'static str = "SET";
static REMOVE_PREFIX: &'static str = "REMOVE";

#[derive(Debug)]
pub enum PersistType {
    Memory,
    File(String),
}

pub trait Persistable<K>
    where K: Into<String> + Ord + Clone
{
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

impl<K> Persistable<K> for FileStore
    where K: Into<String> + Ord + Clone
{
    fn set(&mut self, key: String, data: Data) -> Result<()> {
        Ok(write!(self.file,
                  "{prefix}{key}\r\n{value}\r\n",
                  prefix = SET_PREFIX,
                  key = key,
                  value = data.into_string())?)
    }

    fn remove(&mut self, key: String) -> Result<()> {
        Ok(write!(self.file,
                  "{prefix}{key}\r\n",
                  prefix = REMOVE_PREFIX,
                  key = key)?)
    }

    fn load(&self) -> Result<BTreeMap<String, Data>> {
        let reader = BufReader::new(&self.file);
        let mut btree: BTreeMap<String, Data> = BTreeMap::new();

        for line in reader.lines() {
            let l = line?;
            if l.starts_with(SET_PREFIX) {
                let csrf_index = l.find(CRLF);
                if csrf_index.is_none() {
                    return Err(Error::new(ErrorKind::InvalidSerializedString));
                }

                let key = l[SET_PREFIX.len()..csrf_index.unwrap()].to_string();
                let value = l[(csrf_index.unwrap()) + 2..(l.len() - 1)].to_string();

                btree.insert(key, Data::try_from(value)?);
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

impl<K> Persistable<K> for MemoryStore
    where K: Into<String> + Ord + Clone
{
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
