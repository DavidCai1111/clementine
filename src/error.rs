use std::error;
use std::fmt;
use std::result;
use std::io;
use std::sync;
use std::num;

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
    // IO errors
    IOError,
    RWLockPoisonError,
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
            ErrorKind::IOError => "io error",
            ErrorKind::RWLockPoisonError => "rwlock poison error",
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

impl From<io::Error> for Error {
    fn from(_: io::Error) -> Error {
        Error { kind: ErrorKind::IOError }
    }
}

impl<T> From<sync::PoisonError<T>> for Error {
    fn from(_: sync::PoisonError<T>) -> Error {
        Error { kind: ErrorKind::IOError }
    }
}

impl From<num::ParseIntError> for Error {
    fn from(_: num::ParseIntError) -> Error {
        Error { kind: ErrorKind::InvalidSerializedString }
    }
}
