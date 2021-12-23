use std::net::{
    SocketAddr,
    SocketAddrV4,
    SocketAddrV6,
    IpAddr,
    TcpStream,
};
use std::io::{Read, Write};
use crate::network::{BlockRange, Command, GetHeadersMessage, HeadersMessage, NetworkEnvelope, VersionMessage, VerackMessage, Error};
use crate::block::GENESIS_BLOCK_HASH;
use crate::util::io::ReaderManager;

#[derive(Debug)]
pub struct Node {
    stream: TcpStream,
}

impl Node {
    pub fn new(ip: IpAddr, port: u16) -> Self {
        let socket_addr = match ip {
            IpAddr::V4(ipv4) => SocketAddr::V4(SocketAddrV4::new(ipv4, port)),
            IpAddr::V6(ipv6) => SocketAddr::V6(SocketAddrV6::new(ipv6, port, 0, 0)), // flowinfo and scope_id set 0 as default
        };
        let stream = TcpStream::connect(socket_addr).unwrap();

        Self { stream }
    }

    pub fn send(&mut self, envelope: &NetworkEnvelope) -> Result<(), Error> {
        let message = envelope.serialize();
        self.stream.write(&message)?;

        Ok(())
    }

    pub fn read(&mut self) -> Result<NetworkEnvelope, Error> {
        let mut reader = ReaderManager::new(&mut self.stream as &mut dyn Read);
        NetworkEnvelope::parse_reader(&mut reader)
    }

    pub fn handshake(&mut self) -> Result<(), Error> {
        self.send(&VersionMessage::default().into())?;
        let (mut ack, mut version) = (false, false);
        while !ack || !version {
            let envelope = self.read()?;
            match envelope.command() {
                Command::Version => {
                    version = true;
                    self.send(&VerackMessage::new().into())?;
                },
                Command::Verack => ack = true,
                _ => {},
            }
        }

        Ok(())
    }

    pub fn handle(&mut self) -> Result<(), Error> {
        self.handshake()?;

        let block_range = BlockRange {
            first: GENESIS_BLOCK_HASH.into(),
            last: [0u8; 32].into(),
        };
        let message = GetHeadersMessage { version: 70015, block_ranges: vec![block_range] };
        self.send(&message.into())?;

        while let Ok(envelope) = self.read() {
            match envelope.command() {
                Command::Headers => {
                    let headers = HeadersMessage::parse(&envelope.payload())?;
                    println!("get {} headers", headers.block_headers.len());
                    if headers.block_headers.len() > 0 {
                        println!("{:?}", headers.block_headers[0]);
                    }
                    // TODO check headers by pow and so on
                },
                _ => println!("get other envelops: {}", envelope.command().text()),
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::network::Node;

    #[test]
    fn node_handshake() {
        // "72.48.253.168" is mainnet.programmingbitcoin.com
        let mut node = Node::new("72.48.253.168".parse().unwrap(), 8333);
        node.handshake().unwrap();
    }

    #[test]
    fn node_handle() {
        let mut node = Node::new("72.48.253.168".parse().unwrap(), 8333);
        node.handle().unwrap();
    }
}
