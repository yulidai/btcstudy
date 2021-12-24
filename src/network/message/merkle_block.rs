use crate::block::BlockHeader;
use crate::util::hash::Hash256Value;
use crate::network::{Command, Error, NetworkEnvelope};
use crate::util::{
    varint,
    io::{ReaderManager, BytesReader},
    converter,
    hash,
};

#[derive(Debug)]
pub struct MerkleBlockMessage {
    pub header: BlockHeader,
    pub tx_total: u32,
    pub tx_hashes: Vec<Hash256Value>,
    pub flag_bits: u32,
}

impl MerkleBlockMessage {
    pub fn serialize(&self) -> Vec<u8> {
        let tx_hashes_len = converter::usize_into_u64(self.tx_hashes.len()).unwrap();

        let mut result = Vec::new();
        result.append(&mut self.header.serialize());
        result.append(&mut self.tx_total.to_le_bytes().to_vec());
        result.append(&mut varint::encode(tx_hashes_len));
        for tx_hash in &self.tx_hashes {
            result.append(&mut tx_hash.to_vec());
        }
        result.append(&mut self.flag_bits.to_be_bytes().to_vec());

        result
    }

    pub fn parse(bytes: &[u8]) -> Result<Self, Error> {
        let mut reader = BytesReader::new(bytes);
        let mut reader = ReaderManager::new(&mut reader);
        Self::parse_reader(&mut reader)
    }

    pub fn parse_reader(reader: &mut ReaderManager) -> Result<Self, Error> {
        let header = BlockHeader::parse_reader(reader)?;
        let tx_total = converter::le_bytes_into_u32(&reader.more(4)?)?;

        let mut tx_hashes_count = varint::decode_with_reader_manager(reader)?;
        let mut tx_hashes = Vec::new();
        while tx_hashes_count > 0 {
            let tx_hash = hash::convert_slice_into_hash256(&reader.more(32)?);
            tx_hashes.push(tx_hash);

            tx_hashes_count -= 1;
        }
        let flag_bits = converter::be_bytes_into_u32(&reader.more(4)?)?;

        Ok(Self { header, tx_total, tx_hashes, flag_bits })
    }

    pub fn command() -> Command {
        Command::MerkleBlock
    }
}

impl Into<NetworkEnvelope> for MerkleBlockMessage {
    fn into(self) -> NetworkEnvelope {
        let command = Self::command();
        let payload = self.serialize();
        NetworkEnvelope::new(command, payload)
    }
}

#[cfg(test)]
mod tests {
    use super::MerkleBlockMessage;

    #[test]
    fn merkle_block_message_parse_serialize() {
        let bytes = hex::decode("00000020df3b053dc46f162a9b00c7f0d5124e2676d47bbe7c5d0793a500000000000000ef445fef2ed495c275892206ca533e7411907971013ab83e3b47bd0d692d14d4dc7c835b67d8001ac157e670bf0d00000aba412a0d1480e370173072c9562becffe87aa661c1e4a6dbc305d38ec5dc088a7cf92e6458aca7b32edae818f9c2c98c37e06bf72ae0ce80649a38655ee1e27d34d9421d940b16732f24b94023e9d572a7f9ab8023434a4feb532d2adfc8c2c2158785d1bd04eb99df2e86c54bc13e139862897217400def5d72c280222c4cbaee7261831e1550dbb8fa82853e9fe506fc5fda3f7b919d8fe74b6282f92763cef8e625f977af7c8619c32a369b832bc2d051ecd9c73c51e76370ceabd4f25097c256597fa898d404ed53425de608ac6bfe426f6e2bb457f1c554866eb69dcb8d6bf6f880e9a59b3cd053e6c7060eeacaacf4dac6697dac20e4bd3f38a2ea2543d1ab7953e3430790a9f81e1c67f5b58c825acf46bd02848384eebe9af917274cdfbb1a28a5d58a23a17977def0de10d644258d9c54f886d47d293a411cb6226103b55635").unwrap();
        let message = MerkleBlockMessage::parse(&bytes).unwrap();
        assert_eq!(message.header.version.value(), 536870912);
        assert_eq!(message.tx_total, 3519);
        assert_eq!(message.tx_hashes.len(), 10);
        assert_eq!(message.flag_bits, 62215733);

        assert_eq!(message.serialize(), bytes);
    }
}
