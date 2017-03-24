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
    fn into_string(self) -> String;
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

    fn serialize_string(s: S) -> String {
        STRING_PREFIX.to_string() + &s.into() + CRLF
    }

    fn serialize_int(i: i64) -> String {
        INT_PREFIX.to_string() + &i.to_string() + CRLF
    }
}

impl Serializable for Data<String> {
    fn into_string(self) -> String {
        match self {
            Data::String(string) => Self::serialize_string(string),
            Data::Int(int) => Self::serialize_int(int),
        }
    }

    fn try_from(string: String) -> Result<Data<String>> {
        if string.len() < 2 || !string.ends_with(CRLF) {
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
        assert_eq!(Data::String("test_\r\nfrom_string".to_string()),
                   Data::from_string("+test_\r\nfrom_string\r\n").unwrap());
    }

    #[test]
    fn test_from_int() {
        assert_eq!(Data::Int(22), Data::from_int(":22\r\n").unwrap());
    }

    #[test]
    fn test_from_invalid_int() {
        assert!(Data::from_int(":22invalid888\r\n").is_err());
    }

    #[test]
    fn test_serialize_string() {
        assert_eq!("+test\r\n_serialize\r\n_string\r\n",
                   Data::serialize_string("test\r\n_serialize\r\n_string"));
    }

    #[test]
    fn test_serialize_int() {
        assert_eq!(":666\r\n", Data::<String>::serialize_int(666));
    }

    #[test]
    fn test_serializble_into() {
        assert_eq!("+666\r\n", Data::String("666".to_string()).into_string());
        assert_eq!(":666\r\n", Data::Int(666).into_string());
    }

    #[test]
    fn test_try_from_invalid() {
        assert!(Data::try_from("".to_string()).is_err());
        assert!(Data::try_from("\r\n".to_string()).is_err());
        assert!(Data::try_from("11111".to_string()).is_err());
    }

    #[test]
    fn test_try_from_string() {
        assert_eq!(Data::String("666".to_string()),
                   Data::try_from("+666\r\n".to_string()).unwrap());
    }

    #[test]
    fn test_try_from_int() {
        assert_eq!(Data::Int(666),
                   Data::try_from(":666\r\n".to_string()).unwrap());
    }
}
