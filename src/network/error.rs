use std::convert::From;

#[derive(Debug)]
pub enum Error {
    NetworkMagicNotMatch,
    ChecksumNotMatch,

    Unknown(String),
}

impl From<&str> for Error {
    fn from(e: &str) -> Self {
        Self::Unknown(e.into())
    }
}
