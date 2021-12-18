use std::convert::From;
use reqwest::Error as ReqwestError;
use hex::FromHexError;

#[derive(Debug)]
pub enum Error {
    InvalidVersion,
    InvalidTxIn,
    InvalidTxFee,
    InvalidSigHash,

    Reqwest(ReqwestError),
    HexDecode(FromHexError),

    Unknown(String),
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Self::Unknown(s.into())
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
