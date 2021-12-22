use std::convert::From;

#[derive(Debug)]
pub enum Error {
    InvalidVersion,
    InvalidTarget, // overflow
    TimeDiffIsNegativeNumber,
    Unknown(String),
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Self::Unknown(s.into())
    }
}
