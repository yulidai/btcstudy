#[derive(Debug)]
pub enum Error {
    // CommandElement
    EmptyBytes,
    TooLongBytes, // should <= 520 within Script::parse
    InvalidBytes,
    InvalidOpcode,

    // Stack
    EmptyStack,

    // Num
    NumDecodeOverflow,

    // Other
    Unknown(String),
}

impl From<&str> for Error {
    fn from(e: &str) -> Self {
        Self::Unknown(e.into())
    }
}
