use crate::util::math;
use super::Opcode;
use std::fmt;

#[derive(Clone)]
pub enum CommandElement {
    Op(Opcode),
    Data(Vec<u8>), // length <= 520
}

impl CommandElement {
    // @return (Self, bytes_used)
    pub fn parse(bytes: &[u8]) -> Result<(Self, usize), &'static str> {
        let len = bytes.len();
        if len == 0 {
            return Err("cannot convert empty bytes into CommandElement");
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
                    return Err("data.len() should <= 520 within Script::parse()");
                }
            },
            _ => {
                match Opcode::from_u8(byte) {
                    Some(code) => {
                        let result = (Self::Op(code), 1);
                        return Ok(result);
                    },
                    None => return Err("invalid opcode"),
                };
            }
        }

        let index_fst = math::check_range_add(index, 1, len)?;
        let index_lst = math::check_range_add(index, payload_len, len)?;
        let data = bytes[index_fst..(index_lst+1)].to_vec();

        let result = (Self::Data(data), (index_lst+1));
        Ok(result)
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
