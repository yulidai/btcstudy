use std::convert::From;
use std::io::Error as IoError;

#[derive(Debug)]
pub enum Error {
    InvalidVersion,
    InvalidTarget, // overflow
    TimeDiffIsNegativeNumber,
    Io(IoError),
    Unknown(String),
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Self::Unknown(s.into())
    }
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Self {
        Self::Io(e)
    }
}
