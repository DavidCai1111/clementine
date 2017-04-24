//! A tiny, embeddable, ACID compliant in-memory key/value database.

#![feature(io)]
#[macro_use]
extern crate serde_json;

pub use self::database::*;
pub use self::transaction::*;
pub use self::error::*;
pub use self::data::*;
pub use self::persist::*;

mod database;
mod error;
mod transaction;
mod data;
mod persist;
