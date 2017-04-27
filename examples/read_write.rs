extern crate clementine;

use clementine::*;

fn main() {
    let db = Database::new(Config::default()).unwrap();

    db.update(|txn| -> Result<()> {
                    assert!(txn.get("hello").is_none());
                    txn.update("hello", Data::Int(998)).unwrap();
                    assert_eq!(&Data::Int(998), txn.get("hello").unwrap());
                    Ok(())
                })
        .unwrap();
}
