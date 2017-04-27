extern crate clementine;

use clementine::*;

fn main() {
    let db = Database::new(Config::default()).unwrap();

    db.read(|txn| -> Result<()> {
                  assert!(txn.get("hello").is_none());
                  Ok(())
              })
        .unwrap();
}
