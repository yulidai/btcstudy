use super::Error;

#[derive(Debug, Clone)]
pub struct Version(u32);

impl Version {
    pub fn new(v: u32) -> Self {
        Self(v)
    }

    pub fn parse(bytes: &[u8]) -> Result<Self, Error> {
        if bytes.len() < 4 {
            return Err(Error::InvalidVersion)
        }

        let mut param: [u8; 4] = Default::default();
        param.copy_from_slice(&bytes[0..4]);
        let result = Self(u32::from_le_bytes(param));

        Ok(result)
    }

    pub fn serialize(&self) -> [u8; 4] {
        self.0.to_le_bytes()
    }

    pub fn value(&self) -> u32 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Version;

    #[test]
    fn version_parse_and_serialize() {
        let bytes = [1u8, 0, 0, 0];
        let version = Version::parse(&bytes).unwrap();
        assert_eq!(version.value(), 1);
        assert_eq!(version.serialize(), bytes);
    }
}
