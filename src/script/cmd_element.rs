use crate::util::Reader;
use super::{Opcode, Error};
use std::fmt;
use std::convert::{TryFrom, From};

#[derive(Clone)]
pub enum CommandElement {
    Op(Opcode),
    Data(Vec<u8>), // length <= 520
    Unknown(u8),
}

impl CommandElement {
    pub fn parse(bytes: &[u8]) -> Result<Self, Error> {
        let mut reader = Reader::new(bytes);
        Self::parse_reader(&mut reader)
    }

    pub fn parse_reader(reader: &mut Reader) -> Result<Self, Error> {
        let payload_len;
        let byte = reader.more(1)?[0];
        match byte {
            1..=75 => payload_len = byte as usize,
            76 => payload_len = reader.more(1)?[0] as usize,
            77 => {
                payload_len = (reader.more(1)?[0] as usize) + (reader.more(1)?[0] as usize) << 8; // little_endian
                if payload_len > 520 {
                    return Err(Error::TooLongBytes);
                }
            },
            _ => {
                match Opcode::from_u8(byte) {
                    Some(code) => return Ok(Self::Op(code)),
                    None => return Ok(Self::Unknown(byte)), // coinbase tx can have any bytes
                };
            }
        }
        let data = reader.more(payload_len)?.to_vec();
        Ok(Self::Data(data))
    }

    pub fn parse_witness(bytes: &[u8]) -> Result<Self, Error> {
        if bytes.is_empty() {
            return Err(Error::InvalidWitnessElement);
        }
        if bytes.len() == 1 {
            let code = match Opcode::from_u8(bytes[0]) {
                Some(code) => Self::Op(code),
                None => Self::Unknown(bytes[0]),
            };
            return Ok(code);
        }
        Ok(Self::Data(bytes.to_vec()))
    }

    pub fn serialize(&self, result: &mut Vec<u8>) -> Result<(), Error> {
        match self {
            CommandElement::Op(op) => result.push(op.value()),
            CommandElement::Data(data) => {
                let len = data.len();
                if len <= 75 { // op
                    let len = u8::try_from(len).expect("len is out of range of u8");
                    result.push(len);
                } else if len <= 255 {
                    result.push(76u8);
                    let len = u8::try_from(len).expect("len is out of range of u8");
                    result.push(len);
                } else if len <= 520 {
                    result.push(77u8);
                    let len = u16::try_from(len).expect("len is out of range of u16");
                    let len_bytes = len.to_le_bytes();
                    result.push(len_bytes[0]);
                    result.push(len_bytes[1]);
                } else {
                    return Err(Error::TooLongBytes);
                }
                result.append(&mut data.clone());
            }
            CommandElement::Unknown(byte) => result.push(*byte),
        }

        Ok(())
    }

    pub fn is_data(&self) -> bool {
        match self {
            CommandElement::Data(_) => true,
            _ => false,
        }
    }

    pub fn is_op(&self) -> bool {
        match self {
            CommandElement::Op(_) => true,
            _ => false,
        }
    }
}

impl fmt::Debug for CommandElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Self::Op(code) => format!("OpCode({:?})", code),
            Self::Data(data) => format!("Data({})", hex::encode(data)),
            Self::Unknown(byte) => format!("Unknown({:x})", byte),
        };
        write!(f, "{}", msg)
    }
}

impl From<Opcode> for CommandElement {
    fn from(opcode: Opcode) -> Self {
        CommandElement::Op(opcode)
    }
}

impl From<Vec<u8>> for CommandElement {
    fn from(data: Vec<u8>) -> Self {
        CommandElement::Data(data)
    }
}
