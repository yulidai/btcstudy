use crate::util::{converter, varint, Reader};
use crate::network::{Command, Error, NetworkAddr, NetworkEnvelope};
use std::convert::Into;

#[derive(Debug)]
pub struct VersionMessage {
    pub version: u32,
    pub services: u64,
    pub timestamp: u64,
    pub addr_sender: NetworkAddr,
    pub addr_receiver: NetworkAddr,
    pub nonce: u64,
    pub agent: Vec<u8>, // len-prefix when parsing
    pub height: u32,
    pub flag: bool, // BIP37
}

impl VersionMessage {
    pub fn parse(bytes: &[u8]) -> Result<Self, Error> {
        let mut reader = Reader::new(bytes);
        Self::parse_reader(&mut reader)
    }

    pub fn parse_reader(reader: &mut Reader) -> Result<Self, Error> {
        let version = converter::le_bytes_into_u32(reader.more(4)?)?;
        let services = converter::le_bytes_into_u64(reader.more(8)?)?;
        let timestamp = converter::le_bytes_into_u64(reader.more(8)?)?;
        let addr_sender = NetworkAddr::parse(reader.more(26)?)?;
        let addr_receiver = NetworkAddr::parse(reader.more(26)?)?;
        let nonce = converter::le_bytes_into_u64(reader.more(8)?)?;
        let agent_len = varint::decode_with_reader(reader)?;
        let agent = reader.more(agent_len.into())?.to_vec();
        let height = converter::le_bytes_into_u32(reader.more(4)?)?;
        let flag = match reader.more(1) {
            Ok(flag) => flag[0] == 1,
            Err(_) => false,
        };

        Ok(Self { version, services, timestamp, addr_sender, addr_receiver, nonce, agent, height, flag, })
    }

    pub fn serialize(&self) -> Vec<u8> {
        let agent_len = converter::usize_into_u64(self.agent.len()).unwrap();

        let mut result = Vec::new();
        result.append(&mut self.version.to_le_bytes().to_vec());
        result.append(&mut self.services.to_le_bytes().to_vec());
        result.append(&mut self.timestamp.to_le_bytes().to_vec());
        result.append(&mut self.addr_sender.serialize());
        result.append(&mut self.addr_receiver.serialize());
        result.append(&mut self.nonce.to_le_bytes().to_vec());
        result.append(&mut varint::encode(agent_len));
        result.append(&mut self.agent.clone());
        result.append(&mut self.height.to_le_bytes().to_vec());
        result.push(if self.flag { 1u8 } else { 0u8 });

        result
    }

    pub fn default() -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};

        Self {
            version: 70015,
            services: 0,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs(),
            addr_sender: NetworkAddr::default(),
            addr_receiver: NetworkAddr::default(),
            nonce: 0,
            agent: "/programmingbitcoin:0.1/".as_bytes().to_vec(),
            height: 0,
            flag: false,
        }
    }

    pub fn command() -> Command {
        Command::Version
    }
}

impl Into<NetworkEnvelope> for VersionMessage {
    fn into(self) -> NetworkEnvelope {
        let command = Self::command();
        let payload = self.serialize();
        NetworkEnvelope::new(command, payload)
    }
}

#[cfg(test)]
mod tests {
    use crate::network::{Command, VersionMessage};
    use std::net::{Ipv4Addr, IpAddr};

    #[test]
    fn version_message_parse() {
        let bytes = hex::decode("7f11010000000000000000000000000000000000000000000000000000000000000000000000ffff00000000208d000000000000000000000000000000000000ffff00000000208d0000000000000000182f70726f6772616d6d696e67626974636f696e3a302e312f0000000000").unwrap();
        let message = VersionMessage::parse(&bytes).unwrap();

        assert_eq!(message.version, 70015);
        assert_eq!(message.services, 0);
        assert_eq!(message.timestamp, 0);

        assert_eq!(message.addr_sender.services, 0);
        assert_eq!(message.addr_sender.ip, IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)));
        assert_eq!(message.addr_sender.port, 8333);

        assert_eq!(message.addr_receiver.services, 0);
        assert_eq!(message.addr_receiver.ip, IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)));
        assert_eq!(message.addr_receiver.port, 8333);

        assert_eq!(message.nonce, 0);
        assert_eq!(String::from_utf8(message.agent.clone()).unwrap(), "/programmingbitcoin:0.1/");
        assert_eq!(message.height, 0);
        assert_eq!(message.flag, false);
    }

    #[test]
    fn version_message_serialize() {
        let bytes = hex::decode("7f11010000000000000000000000000000000000000000000000000000000000000000000000ffff00000000208d000000000000000000000000000000000000ffff00000000208d0000000000000000182f70726f6772616d6d696e67626974636f696e3a302e312f0000000000").unwrap();
        let message = VersionMessage::parse(&bytes).unwrap();

        assert_eq!(message.serialize(), bytes);
    }

    #[test]
    fn version_command() {
        assert_eq!(VersionMessage::command(), Command::Version);
    }
}
