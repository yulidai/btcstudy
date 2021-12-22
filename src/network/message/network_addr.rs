use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
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
}

fn bytes_to_ip(bytes: &[u8]) -> Result<IpAddr, &'static str> {
    if bytes.len() != 16 {
        return Err("IpAddr needs 16 bytes");
    }
    let ip = if bytes[0..10] == [0u8; 10] && bytes[10..12] == [0xff, 0xff] {
        IpAddr::V4(Ipv4Addr::new(bytes[12], bytes[13], bytes[14], bytes[15]))
    } else {
        IpAddr::V6(Ipv6Addr::new(
            converter::le_bytes_into_u16(&bytes[0..2])?,
            converter::le_bytes_into_u16(&bytes[2..4])?,
            converter::le_bytes_into_u16(&bytes[4..6])?,
            converter::le_bytes_into_u16(&bytes[6..8])?,
            converter::le_bytes_into_u16(&bytes[8..10])?,
            converter::le_bytes_into_u16(&bytes[10..12])?,
            converter::le_bytes_into_u16(&bytes[12..14])?,
            converter::le_bytes_into_u16(&bytes[14..16])?,
        ))
    };

    Ok(ip)
}
