use std::net::{IpAddr, Ipv4Addr};
use crate::util::converter;
use crate::network::Error;

#[derive(Debug)]
pub struct NetworkAddr {
    pub services: u64,
    pub ip: IpAddr,
    pub port: u16,
}

impl NetworkAddr {
    pub fn parse(bytes: &[u8]) -> Result<Self, Error> {
        if bytes.len() != 26 {
            return Err(Error::InvalidNetworkAddr);
        }
        let services = converter::le_bytes_into_u64(&bytes[0..8])?;
        let ip = bytes_to_ip(&bytes[8..24])?;
        let port = converter::be_bytes_into_u16(&bytes[24..26])?;

        Ok(Self { services, ip, port })
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.append(&mut self.services.to_le_bytes().to_vec());
        match self.ip {
            IpAddr::V4(ipv4) => {
                result.append(&mut vec![0u8; 10]);
                result.append(&mut vec![0xff; 2]);
                result.append(&mut ipv4.octets().to_vec());
            }
            IpAddr::V6(ipv6) => result.append(&mut ipv6.octets().to_vec()),
        }
        result.append(&mut self.port.to_be_bytes().to_vec());

        result
    }

    pub fn default() -> Self {
        Self {
            services: 0,
            ip: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            port: 8333,
        }
    }
}

fn bytes_to_ip(bytes: &[u8]) -> Result<IpAddr, &'static str> {
    if bytes.len() != 16 {
        return Err("IpAddr needs 16 bytes");
    }
    let ip = if bytes[0..10] == [0u8; 10] && bytes[10..12] == [0xff, 0xff] {
        IpAddr::V4(Ipv4Addr::new(bytes[12], bytes[13], bytes[14], bytes[15]))
    } else {
        let mut ip_bytes = [0u8; 16];
        ip_bytes.copy_from_slice(bytes);
        IpAddr::from(ip_bytes)
    };

    Ok(ip)
}
