use std::convert::From;

#[derive(Debug)]
pub enum Error {
    NetworkMagicNotMatch,
    ChecksumNotMatch,
    PayloadTooBig,

    InvalidNetworkAddr,

    Unknown(String),
}

impl From<&str> for Error {
    fn from(e: &str) -> Self {
        Self::Unknown(e.into())
    }
}
