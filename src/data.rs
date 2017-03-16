use std::convert::From;

pub trait DataKind: From<String> {
    fn to_string(&self) -> Option<String>;
}

#[derive(Debug)]
pub enum DataKinds {
    String(String),
    Int(i64),
    Uint(u64),
    Float(f64),
}

impl From<String> for DataKinds {
    fn from(s: String) -> Self {
        unimplemented!()
    }
}

impl DataKind for DataKinds {
    fn to_string(&self) -> Option<String> {
        unimplemented!()
    }
}
