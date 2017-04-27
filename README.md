# clementine
[![Build Status](https://travis-ci.org/DavidCai1993/clementine.svg?branch=master)](https://travis-ci.org/DavidCai1993/clementine)

A tiny, embeddable, ACID compliant in-memory key/value database.

## Installation

```tmol
[dependencies]
clementine = "0.0.1"
```

## Usage

```rust
let db = Database::new(Config::default()).unwrap();

db.read(|txn| -> Result<()> {
    assert!(txn.get("hello").is_none());
    Ok(())
}).unwrap();
```

```rust
let db = Database::new(Config::default()).unwrap();

db.update(|txn| -> Result<()> {
    assert!(txn.get("hello").is_none());
    txn.update("hello", Data::Int(998)).unwrap();
    assert_eq!(&Data::Int(998), txn.get("hello").unwrap());
    Ok(())
}).unwrap();
```
