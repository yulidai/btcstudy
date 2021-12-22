use crate::util::{converter, Reader};
use crate::network::{Error, NetworkAddr};

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
        let agent_len = reader.more(1)?[0];
        let agent = reader.more(agent_len.into())?.to_vec();
        let height = converter::le_bytes_into_u32(reader.more(4)?)?;
        let flag = match reader.more(1) {
            Ok(flag) => flag[0] == 1,
            Err(_) => false,
        };

        Ok(Self { version, services, timestamp, addr_sender, addr_receiver, nonce, agent, height, flag, })
    }
}

#[cfg(test)]
mod tests {
    use super::VersionMessage;
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
}
