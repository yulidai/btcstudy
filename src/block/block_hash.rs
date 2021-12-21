use crate::util::{
    hash::{self, Hash256Value},
    Reader,
};
use super::Error;

#[derive(Debug)]
pub struct BlockHash(Hash256Value);

impl BlockHash {
    pub fn serialize(&self) -> Hash256Value {
        let mut result = self.0.to_vec();
        result.reverse(); // little endian
        hash::convert_slice_into_hash256(&result)
    }

    pub fn parse(reader: &mut Reader) -> Result<Self, Error> {
        let mut bytes = reader.more(32)?.to_vec();
        bytes.reverse(); // little endian
        let result = Self(hash::convert_slice_into_hash256(&bytes));

        Ok(result)
    }
}
