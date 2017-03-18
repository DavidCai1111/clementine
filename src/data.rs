use std::convert::From;

pub trait Serializable: From<String> {
    fn to_string(&self) -> Option<String>;
}

#[derive(Debug)]
pub enum Data<S>
    where S: Into<String>
{
    String(S),
    Int(i64),
    Uint(u64),
    Float(f64),
}

impl<S> From<String> for Data<S>
    where S: Into<String>
{
    fn from(_: String) -> Self {
        unimplemented!()
    }
}

impl<S> Serializable for Data<S>
    where S: Into<String>
{
    fn to_string(&self) -> Option<String> {
        match *self {
            // Data::String(s) => Some(s.into()),
            Data::Int(i) => Some(format!("{}", i)),
            _ => None,
        }
    }
}
