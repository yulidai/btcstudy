use std::net::{
    SocketAddr,
    SocketAddrV4,
    SocketAddrV6,
    IpAddr,
    TcpStream,
};
use std::io::Write;
use crate::network::{Command, NetworkEnvelope, VersionMessage, VerackMessage, Error};

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
        NetworkEnvelope::parse_from_tcpstream(&mut self.stream)
    }

    pub fn handshake(&mut self) -> Result<(), Error> {
        self.send(&VersionMessage::default().into())?;
        let (mut ack, mut version) = (false, false);
        while !ack || !version {
            let envelope = self.read()?;
            println!("receive envelope: {:?}", envelope);
            match envelope.command() {
                Command::Version => {
                    version = true;
                    self.send(&VerackMessage::new().into())?;
                },
                Command::Verack => ack = true,
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
}
