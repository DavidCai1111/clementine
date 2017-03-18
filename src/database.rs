use std::collections::BTreeMap;
use std::sync::RwLock;
use transaction::{Transaction, ReadTransaction, WriteTransaction};
use error::{Error, ErrorKind, Result};

#[derive(Debug)]
pub struct Database<S: Into<String> + Ord + Clone> {
    txn_mut: RwLock<Transaction<S>>,
    closed: bool,
}

impl<S: Into<String> + Ord + Clone> Database<S> {
    pub fn new() -> Result<Database<S>> {
        Ok(Database {
            txn_mut: RwLock::new(Transaction::new(Box::new(BTreeMap::new()))),
            closed: false,
        })
    }

    pub fn read<F>(&self, f: F) -> Result<()>
        where F: Fn(&ReadTransaction<S>) -> Result<()>
    {
        match self.txn_mut.read() {
            Ok(store) => {
                if self.closed {
                    return Err(Error::new(ErrorKind::DataBaseClosed));
                }
                f(&*store)
            }
            Err(_) => unreachable!(),
        }
    }

    pub fn update<F>(&self, f: F) -> Result<()>
        where F: Fn(&mut WriteTransaction<S>) -> Result<()>
    {
        match self.txn_mut.write() {
            Ok(mut store) => {
                if self.closed {
                    return Err(Error::new(ErrorKind::DataBaseClosed));
                }

                if f(&mut *store).is_err() {
                    return store.rollback();
                }
                Ok(())
            }
            Err(_) => unreachable!(),
        }
    }

    pub fn close(&mut self) -> Result<()> {
        if self.closed {
            return Err(Error::new(ErrorKind::DataBaseClosed));
        }

        match self.txn_mut.write() {
            Ok(_) => self.closed = true,
            Err(_) => unreachable!(),
        }
        Ok(())
    }
}

impl<S: Into<String> + Ord + Clone> Drop for Database<S> {
    fn drop(&mut self) {
        if !self.closed {
            self.close().unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let db: Database<String> = Database::new().unwrap();
        assert_eq!(false, db.closed);
    }

    #[test]
    fn test_close() {
        let mut db: Database<String> = Database::new().unwrap();
        assert!(db.close().is_ok());
        assert!(db.close().is_err());
        assert!(db.close().is_err());
    }

    #[test]
    fn test_read_empty() {
        let db = &Database::new().unwrap();
        assert!(db.read(|txn| -> Result<()> {
                assert!(txn.get("123").is_none());
                Ok(())
            })
            .is_ok())
    }

    #[test]
    fn test_update() {
        let db = &Database::new().unwrap();
        assert!(db.update(|txn| -> Result<()> {
                assert_eq!(true, txn.update("1", "1").is_none());
                assert_eq!("1", *txn.get("1").unwrap());
                Ok(())
            })
            .is_ok());
    }

    #[test]
    fn test_remove() {
        let db = &Database::new().unwrap();
        assert!(db.update(|txn| -> Result<()> {
                assert_eq!(true, txn.update("1", "1").is_none());
                assert_eq!("1", *txn.get("1").unwrap());
                assert_eq!(true, txn.remove(&"1").is_some());
                assert_eq!(true, txn.get("1").is_none());
                Ok(())
            })
            .is_ok());
    }

    #[test]
    fn test_remove_all() {
        let db = &Database::new().unwrap();
        assert!(db.update(|txn| -> Result<()> {
                assert_eq!(true, txn.update("1", "1").is_none());
                assert_eq!(true, txn.update("2", "2").is_none());
                assert_eq!(true, txn.update("3", "3").is_none());
                assert_eq!(3, txn.len());
                txn.remove_all();
                assert_eq!(0, txn.len());
                assert_eq!(true, txn.is_empty());
                Ok(())
            })
            .is_ok());
    }
}
