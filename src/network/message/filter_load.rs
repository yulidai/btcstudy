use crate::util::{
    converter,
    varint,
    io::{BytesReader, ReaderManager},
};
use crate::network::{Command, Error, NetworkEnvelope};

#[derive(Debug)]
pub struct FilterLoadMessage {
    pub bit_field: Vec<u8>,
    pub hash_count: u32,
    pub tweak: u32,
    pub flag: u8,
}

impl FilterLoadMessage {
    pub fn serialize(&self) -> Vec<u8> {
        let bit_field_len = converter::usize_into_u64(self.bit_field.len()).unwrap();
        let mut result = Vec::new();
        result.append(&mut varint::encode(bit_field_len));
        result.append(&mut self.bit_field.clone());
        result.append(&mut self.hash_count.to_le_bytes().to_vec());
        result.append(&mut self.tweak.to_le_bytes().to_vec());
        result.push(self.flag);

        result
    }

    pub fn parse(bytes: &[u8]) -> Result<Self, Error> {
        let mut reader = BytesReader::new(bytes);
        let mut reader = ReaderManager::new(&mut reader);
        Self::parse_reader(&mut reader)
    }

    pub fn parse_reader(reader: &mut ReaderManager) -> Result<Self, Error> {
        let bit_field_count = varint::decode_with_reader_manager(reader)?;
        let bit_field = reader.more(bit_field_count)?;

        let hash_count = converter::le_bytes_into_u32(&reader.more(4)?)?;
        let tweak = converter::le_bytes_into_u32(&reader.more(4)?)?;
        let flag = reader.more(1)?[0];

        Ok(Self { bit_field, hash_count, tweak, flag })
    }

    pub fn command() -> Command {
        Command::FilterLoad
    }
}

impl Into<NetworkEnvelope> for FilterLoadMessage {
    fn into(self) -> NetworkEnvelope {
        let command = Self::command();
        let payload = self.serialize();
        NetworkEnvelope::new(command, payload)
    }
}

#[cfg(test)]
mod tests {
    use super::FilterLoadMessage;

    #[test]
    fn filter_load_message_parse() {
        let bytes = hex::decode("0a4000600a080000010940050000006300000001").unwrap();
        let message = FilterLoadMessage::parse(&bytes).unwrap();

        assert_eq!(hex::encode(&message.bit_field), "4000600a080000010940");
        assert_eq!(message.hash_count, 5);
        assert_eq!(message.tweak, 99);
        assert_eq!(message.flag, 1);
    }
}
