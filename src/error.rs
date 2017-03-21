use std::error;
use std::fmt;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum ErrorKind {
    // Database errors.
    DataBaseClosed,
    // Transaction errors.
    TransactionNotWritable,
    ItemNotFound,
    // Data errors.
    InvalidSerializedString,
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
}

impl Error {
    pub fn new(kind: ErrorKind) -> Error {
        Error { kind: kind }
    }

    pub fn message(&self) -> &str {
        match self.kind {
            ErrorKind::DataBaseClosed => "database already closed",
            ErrorKind::TransactionNotWritable => "transaction is not writable",
            ErrorKind::ItemNotFound => "item not found",
            ErrorKind::InvalidSerializedString => "invalid serialized string",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[clementine error]: {:?}", self.message())
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        self.message()
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error { kind: kind }
    }
}
