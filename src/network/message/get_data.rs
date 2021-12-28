use crate::util::{
    converter,
    hash,
    varint,
    io::{BytesReader, ReaderManager},
    hash::Hash256Value,
};
use crate::network::{Command, Error, NetworkEnvelope};

#[derive(Debug)]
pub struct GetDataMessage {
    pub datas: Vec<Data>, // (type, hash)
}

impl GetDataMessage {
    pub fn new() -> Self {
        Self { datas: Vec::new() }
    }

    pub fn push(&mut self, data: Data) {
        self.datas.push(data);
    }

    pub fn serialize(&self) -> Vec<u8> {
        let datas_len = converter::usize_into_u64(self.datas.len()).unwrap();
        let mut result = Vec::new();
        result.append(&mut varint::encode(datas_len));
        for data in &self.datas {
            result.append(&mut data.0.u32().to_le_bytes().to_vec());
            result.append(&mut data.1.to_vec());
        }

        result
    }

    pub fn parse(bytes: &[u8]) -> Result<Self, Error> {
        let mut reader = BytesReader::new(bytes);
        let mut reader = ReaderManager::new(&mut reader);
        Self::parse_reader(&mut reader)
    }

    pub fn parse_reader(reader: &mut ReaderManager) -> Result<Self, Error> {
        let datas_len = varint::decode_with_reader_manager(reader)?;

        let mut datas = Vec::new();
        for _ in 0..datas_len {
            let data_type = converter::le_bytes_into_u32(&reader.more(4)?)?;
            let data_type = DataType::parse(data_type)?;
            let data = hash::convert_slice_into_hash256(&reader.more(32)?);
            datas.push((data_type, data));
        }

        Ok(Self { datas })
    }

    pub fn command() -> Command {
        Command::GetData
    }
}

impl Into<NetworkEnvelope> for GetDataMessage {
    fn into(self) -> NetworkEnvelope {
        let command = Self::command();
        let payload = self.serialize();
        NetworkEnvelope::new(command, payload)
    }
}

pub type Data = (DataType, Hash256Value);

#[derive(Debug)]
pub enum DataType {
    Tx,
    Block,
    MerkleBlock,
    CompactBlock,
}

impl DataType {
    pub fn u32(&self) -> u32 {
        match self {
            Self::Tx => 1,
            Self::Block => 2,
            Self::MerkleBlock => 3,
            Self::CompactBlock => 4,
        } 
    }

    pub fn parse(n: u32) -> Result<Self, Error> {
        let r = match n {
            1 => Self::Tx,
            2 => Self::Block,
            3 => Self::MerkleBlock,
            4 => Self::CompactBlock,
            _ => return Err(Error::InvalidGetDataType),
        };
        Ok(r)
    }
}

#[cfg(test)]
mod tests {
    use super::{DataType, GetDataMessage};
    use crate::util::hash;

    #[test]
    fn get_data_message_serialize() {
        let mut message = GetDataMessage::new();

        let mut hash1 = hex::decode("00000000000000cac712b726e4326e596170574c01a16001692510c44025eb30").unwrap();
        hash1.reverse(); // littile endian
        message.push((DataType::MerkleBlock, hash::convert_slice_into_hash256(&hash1)));

        let mut hash2 = hex::decode("00000000000000beb88910c46f6b442312361c6693a7fb52065b583979844910").unwrap();
        hash2.reverse(); // littile endian
        message.push((DataType::MerkleBlock, hash::convert_slice_into_hash256(&hash2)));

        let result = message.serialize();
        let expect = hex::decode("020300000030eb2540c41025690160a1014c577061596e32e426b712c7ca00000000000000030000001049847939585b0652fba793661c361223446b6fc41089b8be00000000000000").unwrap();
        assert_eq!(result, expect);
    }

    #[test]
    fn get_data_message_parse() {
        let bytes = hex::decode("020300000030eb2540c41025690160a1014c577061596e32e426b712c7ca00000000000000030000001049847939585b0652fba793661c361223446b6fc41089b8be00000000000000").unwrap();
        let message = GetDataMessage::parse(&bytes).unwrap();
        assert_eq!(message.datas.len(), 2);
        assert_eq!(message.serialize(), bytes);
    }
}
