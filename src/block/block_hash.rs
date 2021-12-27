use crate::util::{
    hash::{self, Hash256Value},
    io::{ReaderManager, BytesReader},
};
use super::Error;

pub const GENESIS_BLOCK_HASH: [u8; 32] = [0, 0, 0, 0, 0, 25, 214, 104, 156, 8, 90, 225, 101, 131, 30, 147, 79, 247, 99, 174, 70, 162, 166, 193, 114, 179, 241, 182, 10, 140, 226, 111];

#[derive(Debug, PartialEq)]
pub struct BlockHash(Hash256Value);

impl BlockHash {
    pub fn serialize(&self) -> Hash256Value {
        let mut result = self.0.to_vec();
        result.reverse(); // little endian
        hash::convert_slice_into_hash256(&result)
    }

    pub fn parse(bytes: &[u8]) -> Result<Self, Error> {
        let mut reader = BytesReader::new(bytes);
        let mut reader = ReaderManager::new(&mut reader);
        Self::parse_reader(&mut reader)
    }

    pub fn parse_reader(reader: &mut ReaderManager) -> Result<Self, Error> {
        let mut bytes = reader.more(32)?;
        bytes.reverse(); // little endian
        let result = Self(hash::convert_slice_into_hash256(&bytes));

        Ok(result)
    }
}

impl From<[u8; 32]> for BlockHash {
    fn from(hash: [u8; 32]) -> Self {
        Self(hash)
    }
}
