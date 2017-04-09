extern crate clementine;

use clementine::{Database, Data, Result, Error, ErrorKind, Config};

#[test]
fn test_rollback_update() {
    let db = &Database::new(Config::default()).unwrap();
    let update_ok_result = db.update(|txn| -> Result<()> {
                                         txn.update("1", Data::Int(1));
                                         Ok(())
                                     });
    assert!(update_ok_result.is_ok());

    let update_fail_result = db.update(|txn| -> Result<()> {
                                           txn.update("1", Data::Int(2));
                                           Err(Error::new(ErrorKind::DataBaseClosed))
                                       });
    assert!(update_fail_result.is_ok());

    let read_result = db.read(|txn| -> Result<()> {
                                  assert_eq!(&Data::Int(1), txn.get("1").unwrap());
                                  Ok(())
                              });
    assert!(read_result.is_ok());
}

#[test]
fn test_rollback_remove() {
    let db = &Database::new(Config::default()).unwrap();
    let update_ok_result = db.update(|txn| -> Result<()> {
                                         txn.update("1", Data::Int(1));
                                         Ok(())
                                     });
    assert!(update_ok_result.is_ok());

    let read_result = db.read(|txn| -> Result<()> {
                                  assert_eq!(&Data::Int(1), txn.get("1").unwrap());
                                  Ok(())
                              });
    assert!(read_result.is_ok());

    let update_fail_result = db.update(|txn| -> Result<()> {
                                           txn.remove("1");
                                           Err(Error::new(ErrorKind::DataBaseClosed))
                                       });
    assert!(update_fail_result.is_ok());

    let read_rollback_result = db.read(|txn| -> Result<()> {
                                           assert_eq!(&Data::Int(1), txn.get("1").unwrap());
                                           Ok(())
                                       });
    assert!(read_rollback_result.is_ok());
}

#[test]
fn test_rollback_remove_all() {
    let db = &Database::new(Config::default()).unwrap();
    let update_result = db.update(|txn| -> Result<()> {
                                      txn.update("1", Data::Int(1));
                                      txn.update("2", Data::Int(2));
                                      Ok(())
                                  });
    assert!(update_result.is_ok());

    let update_fail_result = db.update(|txn| -> Result<()> {
                                           txn.update("1", Data::Int(1));
                                           txn.clear();
                                           Err(Error::new(ErrorKind::DataBaseClosed))
                                       });
    assert!(update_fail_result.is_ok());

    let read_rollback_result = db.read(|txn| -> Result<()> {
                                           assert_eq!(&Data::Int(1), txn.get("1").unwrap());
                                           assert_eq!(&Data::Int(2), txn.get("2").unwrap());
                                           Ok(())
                                       });
    assert!(read_rollback_result.is_ok());
}
