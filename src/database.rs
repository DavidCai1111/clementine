use std::collections::BTreeMap;
use std::sync::RwLock;
use error::{Error, ErrorKind, Result};

#[derive(Debug)]
pub struct Database {
    store: BTreeMap<String, String>,
    mutex: RwLock<()>,
    closed: bool,
}

impl Database {
    pub fn new() -> Result<Database> {
        Ok(Database {
            store: BTreeMap::new(),
            mutex: RwLock::new(()),
            closed: false,
        })
    }

    pub fn read() -> Result<()> {
        Ok(())
    }

    pub fn update() -> Result<()> {
        Ok(())
    }

    pub fn close(&self) -> Result<()> {
        if self.closed {
            return Err(Error::new(ErrorKind::DataBaseClosed));
        }
        Ok(())
    }
}
