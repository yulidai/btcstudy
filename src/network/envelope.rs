use std::fmt;
use crate::util::{converter, hash, Reader};
use super::{Error, Command};

pub const NETWORK_MAGIC: [u8; 4] = [0xf9, 0xbe, 0xb4, 0xd9];

pub struct NetworkEnvelope {
    command: Command,
    payload: Vec<u8>,
    // only used in parse():
    // - payload_len: [u8; 4],
    // - payload_checksum: [u8; 4],
}

impl fmt::Debug for NetworkEnvelope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NetworkEnvelope")
            .field("command", &self.command.text())
            .field("payload", &hex::encode(&self.payload))
            .finish()
    }
}

impl NetworkEnvelope {
    pub fn new(command: Command, payload: Vec<u8>) -> Self {
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

        let command = Command::parse(reader.more(12)?)?;
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
        result.append(&mut self.command.serialize().to_vec());
        result.append(&mut payload_len.to_le_bytes().to_vec());
        result.append(&mut checksum.to_vec());
        result.append(&mut self.payload.clone());

        result
    }

    pub fn command(&self) -> Command {
        self.command
    }

    pub fn payload(&self) -> &Vec<u8> {
        &self.payload
    }
}

#[cfg(test)]
mod tests {
    use crate::network::{Command, NetworkEnvelope};

    #[test]
    fn network_envelope_parse_serialize() {
        let bytes = hex::decode("f9beb4d976657261636b000000000000000000005df6e0e2").unwrap();
        let envelope = NetworkEnvelope::parse(&bytes).unwrap();

        assert_eq!(envelope.command(), Command::Verack);
        assert_eq!(envelope.payload().len(), 0);
        assert_eq!(bytes, envelope.serialize());
    }
}
