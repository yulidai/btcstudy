use std::fmt;
use crate::util::{converter, hash, Reader};
use super::{Error};

pub const NETWORK_MAGIC: [u8; 4] = [0xf9, 0xbe, 0xb4, 0xd9];

pub struct NetworkEnvelope {
    command: [u8; 12],
    payload: Vec<u8>,
    // only used in parse():
    // - payload_len: [u8; 4],
    // - payload_checksum: [u8; 4],
}

impl fmt::Debug for NetworkEnvelope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let command = String::from_utf8(self.command.to_vec()).unwrap_or(hex::encode(self.command));
        f.debug_struct("NetworkEnvelope")
            .field("command", &command)
            .field("payload", &hex::encode(&self.payload))
            .finish()
    }
}

impl NetworkEnvelope {
    pub fn new(command: [u8; 12], payload: Vec<u8>) -> Self {
        Self { command, payload }
    }

    pub fn parse(bytes: &[u8]) -> Result<Self, Error> {
        let mut reader = Reader::new(bytes);
        Self::parse_reader(&mut reader)
    }

    pub fn parse_reader(reader: &mut Reader) -> Result<Self, Error> {
        let network = reader.more(4)?;
        if network != NETWORK_MAGIC {
            return Err(Error::NetworkMagicNotMatch);
        }

        let mut command = [0u8; 12];
        command.copy_from_slice(reader.more(12)?);
        let payload_len = converter::le_bytes_into_u32(reader.more(4)?)?;
        let payload_len = converter::u32_into_usize(payload_len)?;
        if payload_len > 0x2000000 {
            return Err(Error::PayloadTooBig);
        }

        let payload_checksum = reader.more(4)?.to_vec();
        let payload = reader.more(payload_len)?.to_vec();

        let checksum = &hash::hash256(&payload)[..4];
        if checksum != &payload_checksum[..] {
            return Err(Error::ChecksumNotMatch);
        }

        Ok(Self { command, payload })
    }

    pub fn serialize(&self) -> Vec<u8> {
        let payload_len = self.payload.len();
        let payload_len = converter::usize_into_u32(payload_len).unwrap();
        let checksum = &hash::hash256(&self.payload)[..4];

        let mut result = Vec::new();
        result.append(&mut NETWORK_MAGIC.to_vec());
        result.append(&mut self.command.to_vec());
        result.append(&mut payload_len.to_le_bytes().to_vec());
        result.append(&mut checksum.to_vec());
        result.append(&mut self.payload.clone());

        result
    }

    pub fn command(&self) -> &[u8; 12] {
        &self.command
    }

    pub fn payload(&self) -> &Vec<u8> {
        &self.payload
    }
}

#[cfg(test)]
mod tests {
    use super::NetworkEnvelope;

    #[test]
    fn network_envelope_parse_serialize() {
        let bytes = hex::decode("f9beb4d976657261636b000000000000000000005df6e0e2").unwrap();
        let envelope = NetworkEnvelope::parse(&bytes).unwrap();

        assert_eq!(hex::encode(envelope.command()), "76657261636b000000000000");
        assert_eq!(envelope.payload().len(), 0);
        assert_eq!(bytes, envelope.serialize());
    }
}
