use std::convert::From;
use std::io::Error as IoError;

#[derive(Debug)]
pub enum Error {
    NetworkMagicNotMatch,
    ChecksumNotMatch,
    PayloadTooBig,

    InvalidCommand,
    InvalidNetworkAddr,

    Unknown(String),
    Io(IoError),
}

impl From<&str> for Error {
    fn from(e: &str) -> Self {
        Self::Unknown(e.into())
    }
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Self {
        Self::Io(e)
    }
}
