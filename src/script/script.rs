use std::convert::TryFrom;
use std::ops::Add;
use super::{CommandElement, operator, Stack, Error};
use crate::util::varint;
use primitive_types::U256;

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

    pub fn parse(bytes: &[u8]) -> Result<Self, Error> {
        let (length, used) = varint::decode(bytes)?;
        let total = length + used as u64;
        if total != bytes.len() as u64 {
            return Err(Error::InvalidBytes);
        }

        let used = used as usize;
        Self::parse_raw(&bytes[used..])
    }

    // without len prefix
    pub fn parse_raw(bytes: &[u8]) -> Result<Self, Error> {
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

    pub fn serialize(&self) -> Result<Vec<u8>, Error> {
        let mut payload = self.raw_serialize()?;
        let len = payload.len() as u64;

        let mut result = varint::encode(len);
        result.append(&mut payload);

        Ok(result)
    }

    fn raw_serialize(&self) -> Result<Vec<u8>, Error> {
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
                        return Err(Error::TooLongBytes);
                    }
                    result.append(&mut data.clone());
                }
            }
        }

        Ok(result)
    }

    pub fn evaluate(&self, z: U256) -> Result<bool, Error> {
        let cmds = self.cmds.clone();
        let mut stack = Stack::new();
        for cmd in cmds {
            if operator::evaluate_command(cmd, &mut stack, z)? == false {
                return Ok(false);
            }
        }

        let ele = stack.pop()?;
        Ok(ele.len() > 0) // 0 is empty vec in stack
    }
}

#[cfg(test)]
mod tests {
    use super::Script;
    use crate::script::{CommandElement, Opcode};
    use primitive_types::U256;

    #[test]
    fn script_evaluate_p2pk_success() {
        let sec = CommandElement::Data(hex::decode("04887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34").unwrap());
        let op = CommandElement::Op(Opcode::OpChecksig);
        let script_pubkey = Script::new(vec![sec, op]);

        let sig = CommandElement::Data(hex::decode("3045022000eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c022100c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6").unwrap());
        let script_sig = Script::new(vec![sig]);

        let combined_script = script_sig + script_pubkey;

        let z = U256::from_big_endian(&hex::decode("7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d").unwrap());
        let result = combined_script.evaluate(z).unwrap();
        assert!(result);
    }

    #[test]
    fn script_evaluate_p2pk_failed() {
        let sec = CommandElement::Data(hex::decode("04887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34").unwrap());
        let op = CommandElement::Op(Opcode::OpChecksig);
        let script_pubkey = Script::new(vec![sec, op]);

        let sig = CommandElement::Data(hex::decode("3045022000eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c022100c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6").unwrap());
        let script_sig = Script::new(vec![sig]);

        let combined_script = script_sig + script_pubkey;
        let z = U256::from_big_endian(&hex::decode("7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3e").unwrap());
        let result = combined_script.evaluate(z).unwrap();
        assert!(!result);
    }

    #[test]
    fn script_evaluate_p2pkh_for_uncompressed_pk() {
        let script_pubkey = hex::decode("76a914fb6c931433c83e8bb5a4c6588c7fc24c08dac6e388ac").unwrap();
        let script_pubkey = Script::parse_raw(&script_pubkey).unwrap();

        let script_sig = hex::decode("473045022000eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c022100c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab64104887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34").unwrap();
        let script_sig = Script::parse_raw(&script_sig).unwrap();

        let combined_script = script_sig + script_pubkey;
        let z = U256::from_big_endian(&hex::decode("7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d").unwrap());
        let result = combined_script.evaluate(z).unwrap();
        assert!(result);
    }

    #[test]
    fn script_evaluate_add_euqal() {
        let script_pubkey = hex::decode("55935987").unwrap();
        let script_pubkey = Script::parse_raw(&script_pubkey).unwrap();

        let script_sig = hex::decode("54").unwrap();
        let script_sig = Script::parse_raw(&script_sig).unwrap();

        let combined_script = script_sig + script_pubkey;
        let result = combined_script.evaluate(U256::zero()).unwrap();
        assert!(result);
    }
}
