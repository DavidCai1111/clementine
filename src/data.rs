use error::{Result, Error, ErrorKind};
use std::time::SystemTime;

static CRLF: &'static str = "\r\n";
static STRING_PREFIX: &'static str = "+";
static INT_PREFIX: &'static str = ":";

pub trait Serializable
    where Self: Sized
{
    fn try_from(String) -> Result<Self>;
    fn into(Self) -> String;
}

#[derive(Debug)]
pub enum Data {
    String(String),
    Int(i64), 
    // JSON(String),
}

impl Serializable for Data {
    fn into(data: Self) -> String {
        match data {
            Data::String(string) => STRING_PREFIX.to_string() + &string + CRLF,
            Data::Int(int) => INT_PREFIX.to_string() + &int.to_string() + CRLF,
        }
    }

    fn try_from(s: String) -> Result<Self> {
        // TODO: convert the string.
        if s.starts_with(STRING_PREFIX) {
            return Ok(Data::Int(1));
        }
        if s.starts_with(INT_PREFIX) {
            return Ok(Data::Int(1));
        }
        Err(Error::new(ErrorKind::InvalidSerializedString))
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
