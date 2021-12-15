use std::convert::From;
use crate::script::Error as ScriptError;
use reqwest::Error as ReqwestError;
use hex::FromHexError;

#[derive(Debug)]
pub enum Error {
    InvalidVersion,

    Script(ScriptError),

    Reqwest(ReqwestError),
    HexDecode(FromHexError),

    Unknown(String),
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Self::Unknown(s.into())
    }
}

impl From<ScriptError> for Error {
    fn from(e: ScriptError) -> Self {
        Self::Script(e)
    }
}

impl From<ReqwestError> for Error {
    fn from(e: ReqwestError) -> Self {
        Self::Reqwest(e)
    }
}

impl From<FromHexError> for Error {
    fn from(e: FromHexError) -> Self {
        Self::HexDecode(e)
    }
}
