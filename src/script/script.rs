use std::convert::TryFrom;
use std::ops::Add;
use super::CommandElement;
use crate::util::varint;

#[derive(Debug)]
pub struct Script {
    cmds: Vec<CommandElement>
}

impl Add for Script {
    type Output = Self;

    fn add(self, mut other: Self) -> Self {
        let mut cmds = self.cmds;
        cmds.append(&mut other.cmds);

        Self { cmds }
    }
}

impl Script {

    pub fn new(cmds: Vec<CommandElement>) -> Self {
        Self { cmds }
    }

    pub fn parse(bytes: &[u8]) -> Result<Self, &'static str> {
        let (length, used) = varint::decode(bytes)?;
        let total = length + used as u64;
        if total != bytes.len() as u64 {
            return Err("bytes.len() is not eq to needed");
        }

        let used = used as usize;
        Self::parse_raw(&bytes[used..])
    }

    // without len prefix
    fn parse_raw(bytes: &[u8]) -> Result<Self, &'static str> {
        let mut cmds = Vec::new();

        let mut index = 0;
        let length = bytes.len();
        while index < length {
            let bytes = &bytes[index..];
            let (element, used) = CommandElement::parse(bytes)?;

            index += used;
            cmds.push(element);
        }

        Ok(Self { cmds })
    }

    pub fn serialize(&self) -> Result<Vec<u8>, &'static str> {
        let mut payload = self.raw_serialize()?;
        let len = payload.len() as u64;

        let mut result = varint::encode(len);
        result.append(&mut payload);

        Ok(result)
    }

    fn raw_serialize(&self) -> Result<Vec<u8>, &'static str> {
        let mut result = Vec::new();
        for cmd in &self.cmds {
            match cmd {
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
                        return Err("too long an cmd");
                    }
                    result.append(&mut data.clone());
                }
            }
        }

        Ok(result)
    }
}
