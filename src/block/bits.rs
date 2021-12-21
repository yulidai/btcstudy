use crate::util::Reader;
use super::Error;
use primitive_types::U256;

pub struct Bits(u32);

impl Bits {
    pub fn parse(bytes: &[u8]) -> Result<Self, Error> {
        let mut reader = Reader::new(bytes);
        Self::parse_reader(&mut reader)
    }

    pub fn parse_reader(reader: &mut Reader) -> Result<Self, Error> {
        let mut param = [0u8; 4];
        param.copy_from_slice(reader.more(4)?);
        let num = u32::from_le_bytes(param);

        Ok(Self(num))
    }

    pub fn serialize(&self) -> [u8; 4] {
        self.0.to_le_bytes()
    }

    pub fn to_target(&self) -> Result<U256, Error> {
        let le_bytes = self.serialize();

        let exponent = le_bytes[3] - 3;
        let intermediator = U256::from(256).checked_pow(exponent.into()).ok_or(Error::InvalidTarget)?;

        let coefficient = U256::from_little_endian(&le_bytes[..3]);
        let result = coefficient.checked_mul(intermediator).ok_or(Error::InvalidTarget)?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::Bits;

    #[test]
    fn bits_get_target() {
        let bytes = hex::decode("e93c0118").unwrap();
        let bits = Bits::parse(&bytes).unwrap();
        let target = bits.to_target().unwrap();
        assert_eq!(format!("{:x}", target), "13ce9000000000000000000000000000000000000000000");
    }
}
