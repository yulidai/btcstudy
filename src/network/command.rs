use super::Error;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Command {
    Version,
    Verack,
    GetHeaders,
    GetData,
    Headers,
    MerkleBlock,
    FilterLoad,
    Unknown, // remove in the future
}

impl Command {
    pub fn parse(bytes: &[u8]) -> Result<Self, Error> {
        println!("parse command: {:?}", String::from_utf8(bytes.to_vec()).unwrap());
        if bytes.len() != 12 {
            return Err(Error::InvalidCommand);
        }
        let mut bytes = bytes.to_vec();

        // remove useless bytes
        while let Some(byte) = bytes.last() {
            if *byte != 0 {
                break;
            }
            bytes.pop();
        }

        // check
        let command = String::from_utf8(bytes).map_err(|_| Error::InvalidCommand)?;
        let command = match command.as_str() {
            "version" => Self::Version,
            "verack" => Self::Verack,
            "getheaders" => Self::GetHeaders,
            "headers" => Self::Headers,
            "merkleblock" => Self::MerkleBlock,
            "filterload" => Self::FilterLoad,
            "getdata" => Self::GetData,
            _ => {
                println!("receive unknown command: {}", command);
                Self::Unknown
            }
        };

        Ok(command)
    }

    pub fn serialize(&self) -> [u8; 12] {
        let bytes = self.text().as_bytes();
        let mut result = [0u8; 12];
        for (i, byte) in bytes.iter().enumerate() {
            result[i] = *byte;
        }

        result
    }

    pub fn text(&self) -> &'static str {
        match self {
            Self::Version => "version",
            Self::Verack => "verack",
            Self::GetHeaders => "getheaders",
            Self::Headers => "headers",
            Self::MerkleBlock => "merkleblock",
            Self::FilterLoad => "filterload",
            Self::GetData => "getdata",
            Self::Unknown => "unknown",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Command;

    #[test]
    fn network_command_parse_version() {
        let bytes = hex::decode("76657273696f6e0000000000").unwrap();
        let command = Command::parse(&bytes).unwrap();
        assert_eq!(command, Command::Version);
        assert_eq!(command.serialize()[..], bytes[..]);
    }

    #[test]
    fn network_command_parse_verack() {
        let bytes = hex::decode("76657261636b000000000000").unwrap();
        let command = Command::parse(&bytes).unwrap();
        assert_eq!(command, Command::Verack);
        assert_eq!(command.serialize()[..], bytes[..]);
    }

    #[test]
    fn network_command_parse_getheaders() {
        let bytes = hex::decode("676574686561646572730000").unwrap();
        let command = Command::parse(&bytes).unwrap();
        assert_eq!(command, Command::GetHeaders);
        assert_eq!(command.serialize()[..], bytes[..]);
    }

    #[test]
    fn network_command_parse_headers() {
        let bytes = hex::decode("686561646572730000000000").unwrap();
        let command = Command::parse(&bytes).unwrap();
        assert_eq!(command, Command::Headers);
        assert_eq!(command.serialize()[..], bytes[..]);
    }

    #[test]
    fn network_command_parse_merkle_block() {
        let bytes = hex::decode("6d65726b6c65626c6f636b00").unwrap();
        let command = Command::parse(&bytes).unwrap();
        assert_eq!(command, Command::MerkleBlock);
        assert_eq!(command.serialize()[..], bytes[..]);
    }

    #[test]
    fn network_command_parse_filter_load() {
        let bytes = hex::decode("66696c7465726c6f61640000").unwrap();
        let command = Command::parse(&bytes).unwrap();
        assert_eq!(command, Command::FilterLoad);
        assert_eq!(command.serialize()[..], bytes[..]);
    }

    #[test]
    fn network_command_parse_getdata() {
        let bytes = hex::decode("676574646174610000000000").unwrap();
        let command = Command::parse(&bytes).unwrap();
        assert_eq!(command, Command::GetData);
        assert_eq!(command.serialize()[..], bytes[..]);
    }
}
