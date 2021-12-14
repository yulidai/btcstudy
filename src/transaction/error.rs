use std::convert::From;
use crate::script::Error as ScriptError;

#[derive(Debug)]
pub enum Error {
    InvalidVersion,

    Script(ScriptError),

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
