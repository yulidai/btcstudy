use super::Error;
use std::cmp::PartialEq;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SigHash {
    All = 0x1,
    None = 0x2,
    Single = 0x3,
    AllAnyoneCanpay = 0x81,
    NoneAnyoneCanpay = 0x82,
    SingleAnyoneCanpay = 0x83,
}

impl SigHash {
    pub fn parse(byte: u8) -> Result<Self, Error> {
        let sighash = match byte {
            0x1 => Self::All,
            0x2 => Self::None,
            0x3 => Self::Single,
            0x81 => Self::AllAnyoneCanpay,
            0x82 => Self::NoneAnyoneCanpay,
            0x83 => Self::SingleAnyoneCanpay,
            _ => return Err(Error::InvalidSigHash),
        };
        Ok(sighash)
    }

    pub fn value(&self) -> u8 {
        match self {
            Self::All => 0x1,
            Self::None => 0x2,
            Self::Single => 0x3,
            Self::AllAnyoneCanpay => 0x81,
            Self::NoneAnyoneCanpay => 0x82,
            Self::SingleAnyoneCanpay => 0x83,
        }
    }

    // little-endian
    pub fn serialize(&self) -> [u8; 4] {
        let mut result = [0u8; 4];
        result[0] = self.value();

        result
    }

    pub fn is_anyone_can_pay(&self) -> bool {
        match self {
            Self::All | Self::None | Self::Single => false,
            _ => true,
        }
    }
}
