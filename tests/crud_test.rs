extern crate clementine;

use clementine::{Database, Data, Result};

#[test]
fn test_read_empty() {
    let db: &Database<&str, Data> = &Database::new().unwrap();
    let result = db.read(|txn| -> Result<()> {
        assert!(txn.get("123").is_none());
        Ok(())
    });
    assert!(result.is_ok());
}

#[test]
fn test_update() {
    let db = &Database::new().unwrap();
    let result = db.update(|txn| -> Result<()> {
        assert!(txn.update("1", Data::Int(1)).is_none());
        assert_eq!(Data::Int(1), *txn.get("1").unwrap());
        Ok(())
    });
    assert!(result.is_ok());
}

#[test]
fn test_remove() {
    let db = &Database::new().unwrap();
    let update_result = db.update(|txn| -> Result<()> {
        assert!(txn.update("1", Data::Int(1)).is_none());
        assert_eq!(&Data::Int(1), txn.get("1").unwrap());
        assert!(txn.remove(&"1").is_some());
        assert!(txn.get("1").is_none());
        Ok(())
    });
    assert!(update_result.is_ok());

    let read_result = db.read(|txn| -> Result<()> {
        assert!(txn.get("1").is_none());
        Ok(())
    });
    assert!(read_result.is_ok());
}
#[test]
fn test_remove_all() {
    let db = &Database::new().unwrap();
    let result = db.update(|txn| -> Result<()> {
        assert!(txn.update("1", Data::Int(1)).is_none());
        assert!(txn.update("2", Data::Int(2)).is_none());
        assert!(txn.update("3", Data::Int(3)).is_none());
        assert_eq!(3, txn.len());
        txn.remove_all();
        assert_eq!(0, txn.len());
        assert!(txn.is_empty());
        Ok(())
    });
    assert!(result.is_ok());
}
