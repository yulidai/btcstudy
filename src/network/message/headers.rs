use crate::block::BlockHeader;
use crate::network::{Command, Error, NetworkEnvelope};
use crate::util::{
    varint,
    io::{ReaderManager, BytesReader},
    converter,
};

#[derive(Debug)]
pub struct HeadersMessage {
    pub block_headers: Vec<BlockHeader>,
}

impl HeadersMessage {
    pub fn serialize(&self) -> Vec<u8> {
        let header_len = converter::usize_into_u64(self.block_headers.len()).unwrap();

        let mut result = Vec::new();
        result.append(&mut varint::encode(header_len));
        for header in &self.block_headers {
            result.append(&mut header.serialize());
            result.append(&mut varint::encode(0));
        }

        result
    }

    pub fn parse(bytes: &[u8]) -> Result<Self, Error> {
        let mut reader = BytesReader::new(bytes);
        let mut reader = ReaderManager::new(&mut reader);
        Self::parse_reader(&mut reader)
    }

    pub fn parse_reader(reader: &mut ReaderManager) -> Result<Self, Error> {
        let mut block_headers = Vec::new();
        let block_len = varint::decode_with_reader_manager(reader)?;
        for _ in 0..block_len {
            let header = BlockHeader::parse_reader(reader)?;
            block_headers.push(header);
            let tx_len = varint::decode_with_reader_manager(reader)?;
            if tx_len != 0 {
                return Err(Error::InvalidTxLength);
            }
        }

        Ok(Self { block_headers })
    }

    pub fn command() -> Command {
        Command::Headers
    }
}

impl Into<NetworkEnvelope> for HeadersMessage {
    fn into(self) -> NetworkEnvelope {
        let command = Self::command();
        let payload = self.serialize();
        NetworkEnvelope::new(command, payload)
    }
}

#[cfg(test)]
mod tests {
    use super::HeadersMessage;

    #[test]
    fn header_message_parse() {
        let bytes = hex::decode("0200000020df3b053dc46f162a9b00c7f0d5124e2676d47bbe7c5d0793a500000000000000ef445fef2ed495c275892206ca533e7411907971013ab83e3b47bd0d692d14d4dc7c835b67d8001ac157e670000000002030eb2540c41025690160a1014c577061596e32e426b712c7ca00000000000000768b89f07044e6130ead292a3f51951adbd2202df447d98789339937fd006bd44880835b67d8001ade09204600").unwrap();
        let headers_msg = HeadersMessage::parse(&bytes).unwrap();
        assert_eq!(headers_msg.block_headers.len(), 2);
    }

    #[test]
    fn header_message_serialize() {
        let bytes = hex::decode("0200000020df3b053dc46f162a9b00c7f0d5124e2676d47bbe7c5d0793a500000000000000ef445fef2ed495c275892206ca533e7411907971013ab83e3b47bd0d692d14d4dc7c835b67d8001ac157e670000000002030eb2540c41025690160a1014c577061596e32e426b712c7ca00000000000000768b89f07044e6130ead292a3f51951adbd2202df447d98789339937fd006bd44880835b67d8001ade09204600").unwrap();
        let headers_msg = HeadersMessage::parse(&bytes).unwrap();
        assert_eq!(headers_msg.serialize(), bytes);
    }
}
