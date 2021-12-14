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

    // Secp256k1
    // TODO: move into scecp256k1 module
    InvalidPublicKey,
    InvalidSignature,

    // Other
    Unknown(String),
}

impl From<&str> for Error {
    fn from(e: &str) -> Self {
        Self::Unknown(e.into())
    }
}
