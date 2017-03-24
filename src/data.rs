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
pub enum Data<S: Into<String> + Clone> {
    String(S),
    Int(i64),
}

impl<S: Into<String> + Clone> Data<S> {
    fn from_string(s: S) -> Result<Data<String>> {
        let string = s.into();
        Ok(Data::String(string[1..string.len() - 2].to_string()))
    }

    fn from_int(s: S) -> Result<Data<S>> {
        let string = s.into();
        match string[1..string.len() - 2].parse::<i64>() {
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

impl Serializable for Data<String> {
    fn into(data: Self) -> String {
        match data {
            Data::String(string) => Self::serialize_string(string.into()),
            Data::Int(int) => Self::serialize_int(int),
        }
    }

    fn try_from(string: String) -> Result<Data<String>> {
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
pub struct DataWithTimestamp<S: Into<String> + Clone> {
    data: Data<S>,
    timestamp: SystemTime,
}

impl<S: Into<String> + Clone> DataWithTimestamp<S> {
    pub fn new(data: Data<S>) -> DataWithTimestamp<S> {
        DataWithTimestamp {
            data: data,
            timestamp: SystemTime::now(),
        }
    }
}

impl<S: Into<String> + Clone> From<Data<S>> for DataWithTimestamp<S> {
    fn from(data: Data<S>) -> DataWithTimestamp<S> {
        Self::new(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        assert_eq!(Data::String("test_from_string".to_string()),
                   Data::from_string("+test_from_string\r\n").unwrap());
    }

    #[test]
    fn test_from_int() {
        assert_eq!(Data::Int(22), Data::from_int(":22\r\n").unwrap());
    }
}
