use std::time::SystemTime;
use std::convert::From;
use error::{Result, Error, ErrorKind};

static CRLF: &'static str = "\r\n";
static STRING_PREFIX: &'static str = "+";
static INT_PREFIX: &'static str = ":";

pub trait Serializable: Clone
    where Self: Sized
{
    fn try_from(String) -> Result<Self>;
    fn into(Self) -> String;
}

// TODO: JSON support: https://github.com/serde-rs/json .
#[derive(Debug, Clone, PartialEq)]
pub enum Data {
    String(String),
    Int(i64),
}

impl Data {
    fn from_string(s: String) -> Result<Self> {
        Ok(Data::String(s[1..s.len() - 2].to_string()))
    }

    fn from_int(s: String) -> Result<Self> {
        match s[1..s.len() - 2].parse::<i64>() {
            Ok(int) => Ok(Data::Int(int)),
            Err(_) => Err(Error::new(ErrorKind::InvalidSerializedString)),
        }
    }

    fn serialize_string(s: String) -> String {
        STRING_PREFIX.to_string() + &s + CRLF
    }

    fn serialize_int(i: i64) -> String {
        INT_PREFIX.to_string() + &i.to_string() + CRLF
    }
}

impl Serializable for Data {
    fn into(data: Self) -> String {
        match data {
            Data::String(string) => Self::serialize_string(string),
            Data::Int(int) => Self::serialize_int(int),
        }
    }

    fn try_from(string: String) -> Result<Self> {
        if string.len() <= 2 || string.ends_with(CRLF) {
            return Err(Error::new(ErrorKind::InvalidSerializedString));
        }

        if string.starts_with(STRING_PREFIX) {
            Ok(Self::from_string(string)?)
        } else if string.starts_with(INT_PREFIX) {
            Ok(Self::from_int(string)?)
        } else {
            Err(Error::new(ErrorKind::InvalidSerializedString))
        }
    }
}

#[derive(Debug)]
pub struct DataWithTimestamp {
    data: Data,
    timestamp: SystemTime,
}

impl DataWithTimestamp {
    pub fn new(data: Data) -> DataWithTimestamp {
        DataWithTimestamp {
            data: data,
            timestamp: SystemTime::now(),
        }
    }
}

impl From<Data> for DataWithTimestamp {
    fn from(data: Data) -> DataWithTimestamp {
        Self::new(data)
    }
}
