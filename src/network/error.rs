use std::convert::From;
use std::io::Error as IoError;
use crate::block::Error as BlockError;

#[derive(Debug)]
pub enum Error {
    NetworkMagicNotMatch,
    ChecksumNotMatch,
    PayloadTooBig,

    InvalidCommand,
    InvalidNetworkAddr,
    InvalidTxLength,
    InvalidGetDataType,

    Unknown(String),
    Io(IoError),
    Block(BlockError),
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

impl From<BlockError> for Error {
    fn from(e: BlockError) -> Self {
        Self::Block(e)
    }
}
