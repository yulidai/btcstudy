use crate::util::io::{ReaderManager, BytesReader};
use super::Error;
use primitive_types::U256;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
pub struct Bits(u32);

impl Bits {
    pub fn parse(bytes: &[u8]) -> Result<Self, Error> {
        let mut reader = BytesReader::new(bytes);
        let mut reader = ReaderManager::new(&mut reader);
        Self::parse_reader(&mut reader)
    }

    pub fn parse_reader(reader: &mut ReaderManager) -> Result<Self, Error> {
        let mut param = [0u8; 4];
        param.copy_from_slice(&reader.more(4)?);
        let num = u32::from_le_bytes(param);

        Ok(Self(num))
    }

    pub fn serialize(&self) -> [u8; 4] {
        self.0.to_le_bytes()
    }

    pub fn from_target(target: U256) -> Result<Self, Error> {
        let mut target_be_bytes = vec![0u8; 32];
        target.to_big_endian(&mut target_be_bytes);

        // remove prefix zero
        while let Some(byte) = target_be_bytes.first() {
            if *byte != 0 {
                break;
            }
            target_be_bytes.remove(0);
        }
        // add prefix-zero if need
        if let Some(byte) = target_be_bytes.first() {
            if *byte > 0x7fu8 {
                target_be_bytes.insert(0, 0x00);
            }
        }

        let exponent: u8 = target_be_bytes.len().try_into().map_err(|_| Error::InvalidTarget)?;
        while target_be_bytes.len() < 3 {
            target_be_bytes.push(0);
        }
        while target_be_bytes.len() > 3 {
            target_be_bytes.pop();
        }
        target_be_bytes.insert(0, exponent);

        let mut result = [0u8; 4];
        result.copy_from_slice(&target_be_bytes[..]);
        let result = u32::from_be_bytes(result);

        Ok(Self(result))
    }

    pub fn to_target(&self) -> Result<U256, Error> {
        let le_bytes = self.serialize();

        let exponent = le_bytes[3] - 3;
        let intermediator = U256::from(256).checked_pow(exponent.into()).ok_or(Error::InvalidTarget)?;

        let coefficient = U256::from_little_endian(&le_bytes[..3]);
        let result = coefficient.checked_mul(intermediator).ok_or(Error::InvalidTarget)?;

        Ok(result)
    }

    pub fn to_diff(&self) -> Result<U256, Error> {
        // 0x1d00ffff is the target of basic diff
        let difficulty_1_target = Self::parse(&[0xff, 0xff, 0, 0x1du8])?.to_target()?; // le

        let target = self.to_target()?;
        Ok(difficulty_1_target / target)
    }
}

#[cfg(test)]
mod tests {
    use super::Bits;

    #[test]
    fn bits_to_target() {
        let bytes = hex::decode("e93c0118").unwrap();
        let bits = Bits::parse(&bytes).unwrap();
        let target = bits.to_target().unwrap();
        assert_eq!(format!("{:x}", target), "13ce9000000000000000000000000000000000000000000");
    }

    #[test]
    fn bits_to_diff() {
        let bytes = hex::decode("e93c0118").unwrap();
        let bits = Bits::parse(&bytes).unwrap();
        let diff = bits.to_diff().unwrap();
        assert_eq!(format!("{}", diff), "888171856257");
    }

    #[test]
    fn bits_from_target() {
        let bytes = hex::decode("e93c0118").unwrap();
        let bits = Bits::parse(&bytes).unwrap();

        let target = bits.to_target().unwrap();
        let bits_new = Bits::from_target(target).unwrap();

        assert_eq!(bits, bits_new);
    }
}
