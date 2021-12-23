use crate::network::Error;
use crate::util::{
    converter,
    io::ReaderManager,
    varint,
    hash::{self, Hash256Value},
};
use std::fmt;

#[derive(Debug)]
pub struct GetHeadersMessage {
    pub version: u32,
    pub block_ranges: Vec<BlockRange>,
}

impl GetHeadersMessage {
    pub fn serialize(&self) -> Vec<u8> {
        let version = self.version.to_le_bytes();
        let block_range_len = converter::usize_into_u64(self.block_ranges.len()).unwrap();

        let mut result = Vec::new();
        result.append(&mut version.to_vec());
        result.append(&mut varint::encode(block_range_len));
        for range in &self.block_ranges {
            result.append(&mut range.serialize());
        }

        result
    }

    pub fn parse(reader: &mut ReaderManager) -> Result<Self, Error> {
        let version = converter::le_bytes_into_u32(&reader.more(4)?)?;
        let mut block_ranges = Vec::new();

        let range_num = varint::decode_with_reader_manager(reader)?;
        for _ in 0..range_num {
            let range = BlockRange::parse(reader)?;
            block_ranges.push(range);
        }

        Ok(Self { version, block_ranges })
    }
}

pub struct BlockRange {
    pub first: Hash256Value,
    pub last: Hash256Value,
}

impl BlockRange {
    pub fn serialize(&self) -> Vec<u8> {
        let mut first = self.first.to_vec();
        first.reverse();
        let mut last = self.last.to_vec();
        last.reverse();

        let mut result = Vec::new();
        result.append(&mut first);
        result.append(&mut last);

        result
    }

    pub fn parse(reader: &mut ReaderManager) -> Result<Self, Error> {
        let mut first = reader.more(32)?;
        first.reverse();
        let first = hash::convert_slice_into_hash256(&first);

        let mut last = reader.more(32)?;
        last.reverse();
        let last = hash::convert_slice_into_hash256(&last);

        Ok(Self { first, last })
    }
}

impl fmt::Debug for BlockRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BlockRange")
            .field("first", &hex::encode(&self.first))
            .field("last", &hex::encode(&self.last))
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::GetHeadersMessage;
    use crate::util::io::{BytesReader, ReaderManager};

    #[test]
    fn get_headers_message_parse() {
        let bytes = hex::decode("7f11010001a35bd0ca2f4a88c4eda6d213e2378a5758dfcd6af437120000000000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap();
        let mut bytes_reader = BytesReader::new(&bytes);
        let mut reader_manager = ReaderManager::new(&mut bytes_reader);
        let get_headers_message = GetHeadersMessage::parse(&mut reader_manager).unwrap();

        assert_eq!(get_headers_message.version, 70015);
        assert_eq!(get_headers_message.block_ranges.len(), 1);

        let range = &get_headers_message.block_ranges[0];
        assert_eq!(hex::encode(&range.first), "0000000000000000001237f46acddf58578a37e213d2a6edc4884a2fcad05ba3");
        assert_eq!(hex::encode(&range.last), "0000000000000000000000000000000000000000000000000000000000000000");
    }

    #[test]
    fn get_headers_message_seiralize() {
        let bytes = hex::decode("7f11010001a35bd0ca2f4a88c4eda6d213e2378a5758dfcd6af437120000000000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap();
        let mut bytes_reader = BytesReader::new(&bytes);
        let mut reader_manager = ReaderManager::new(&mut bytes_reader);
        let get_headers_message = GetHeadersMessage::parse(&mut reader_manager).unwrap();

        assert_eq!(get_headers_message.serialize(), bytes);
    }
}
