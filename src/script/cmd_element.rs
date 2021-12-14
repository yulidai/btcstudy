use crate::util::math;
use super::{Opcode, Error};
use std::fmt;
use std::convert::TryFrom;

#[derive(Clone)]
pub enum CommandElement {
    Op(Opcode),
    Data(Vec<u8>), // length <= 520
}

impl CommandElement {
    // @return (Self, bytes_used)
    pub fn parse(bytes: &[u8]) -> Result<(Self, usize), Error> {
        let len = bytes.len();
        if len == 0 {
            return Err(Error::EmptyBytes);
        }

        let mut index = 0;
        let payload_len;
        let byte = bytes[index];
        match byte {
            1..=75 => payload_len = byte as usize,
            76 => {
                index = math::check_range_add(index, 1, len)?;
                payload_len = bytes[index] as usize;
            },
            77 => {
                index = math::check_range_add(index, 2, len)?;
                payload_len = (bytes[index-1] as usize) + (bytes[index] as usize) << 8; // little_endian
                if payload_len > 520 {
                    return Err(Error::TooLongBytes);
                }
            },
            _ => {
                match Opcode::from_u8(byte) {
                    Some(code) => {
                        let result = (Self::Op(code), 1);
                        return Ok(result);
                    },
                    None => return Err(Error::InvalidOpcode),
                };
            }
        }

        let index_fst = math::check_range_add(index, 1, len)?;
        let index_lst = math::check_range_add(index, payload_len, len)?;
        let data = bytes[index_fst..(index_lst+1)].to_vec();

        let result = (Self::Data(data), (index_lst+1));
        Ok(result)
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
        }

        Ok(())
    }
}

impl fmt::Debug for CommandElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Self::Op(code) => format!("CommandElement::OpCode({:x})", code.value()),
            Self::Data(data) => format!("CommandElement::Data({})", hex::encode(data)),
        };
        write!(f, "{}", msg)
    }
}
