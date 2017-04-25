extern crate clementine;

use clementine::*;

fn main() {
    let db = Database::new(Config::default()).unwrap();

    db.update(|txn| -> Result<()> {
                    assert!(txn.get("hello").is_none());
                    txn.update("hello", Data::String(String::from("world")));
                    assert_eq!(&Data::String(String::from("world")),
                               txn.get("hello").unwrap());
                    Ok(())
                })
        .unwrap();
}
