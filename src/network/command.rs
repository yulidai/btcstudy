use super::Error;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Command {
    Version,
    Verack,
    GetHeaders,
}

impl Command {
    pub fn parse(bytes: &[u8]) -> Result<Self, Error> {
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
            _ => return Err(Error::InvalidCommand),
        };

        Ok(command)
    }

    pub fn serialize(&self) -> [u8; 12] {
        let bytes = match self {
            Self::Version => b"version".to_vec(),
            Self::Verack => b"verack".to_vec(),
            Self::GetHeaders => b"getheaders".to_vec(),
        };

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
}
