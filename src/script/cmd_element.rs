use crate::util::math;
use super::Opcode;

#[derive(Debug, Clone)]
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
        if len == 1 {
            match Opcode::from_u8(bytes[0]) {
                Some(code) => {
                    let result = (Self::Op(code), 1);
                    return Ok(result);
                },
                None => return Err("invalid opcode"),
            };
        }

        let mut index = 0;
        let payload_len;
        let byte = bytes[index];
        match byte {
            75 => payload_len = byte as usize,
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
            _ => return Err("invalid bytes for CommandElement"),
        }

        let index_fst = math::check_range_add(index, 1, len)?;
        let index_lst = math::check_range_add(index_fst, payload_len, len)?;
        let data = bytes[index_fst..index_lst].to_vec();

        let result = (Self::Data(data), index_lst);
        Ok(result)
    }
}
