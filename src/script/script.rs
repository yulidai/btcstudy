use std::ops::Add;
use super::{CommandElement, operator, Stack, Error, Opcode};
use crate::util::varint;
use crate::transaction::ZProvider;

#[derive(Debug, Clone)]
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

    pub fn parse(bytes: &[u8]) -> Result<(Self, usize), Error> {
        let (length, used) = varint::decode(bytes)?;
        let total = length + used as usize;
        if total > bytes.len() {
            return Err(Error::InvalidBytes);
        }

        let used = used as usize;
        let (script, used_real) = Self::parse_raw(&bytes[used..total])?;
        if length != used_real {
            return Err(Error::InvalidBytes);
        }

        Ok((script, total))
    }

    // without len prefix
    pub fn parse_raw(bytes: &[u8]) -> Result<(Self, usize), Error> {
        let mut cmds = Vec::new();

        let mut index = 0;
        let length = bytes.len();
        while index < length {
            let bytes = &bytes[index..];
            let (element, used) = CommandElement::parse(bytes)?;
            index += used;
            cmds.push(element);
        }
        cmds.reverse();

        Ok((Self { cmds }, index))
    }

    pub fn serialize(&self) -> Result<Vec<u8>, Error> {
        let mut payload = self.raw_serialize()?;
        let len = payload.len() as u64;

        let mut result = varint::encode(len);
        result.append(&mut payload);

        Ok(result)
    }

    fn raw_serialize(&self) -> Result<Vec<u8>, Error> {
        let mut cmds = self.cmds.clone();
        cmds.reverse();

        let mut result = Vec::new();
        for cmd in &cmds {
            cmd.serialize(&mut result)?;
        }

        Ok(result)
    }

    // @param index: index of input in inputs
    pub fn evaluate(&self, index: usize, z_provider: &Box<dyn ZProvider>) -> Result<bool, Error> {
        let mut cmds = self.cmds.clone();
        let mut stack = Stack::new();
        while let Some(cmd) = cmds.pop() {
            if operator::evaluate_command(cmd, &mut stack, index, z_provider)? == false {
                return Ok(false);
            }
            if let Some(hash160) = Self::get_hash160_if_p2sh(&cmds) {
                if operator::evaluate_p2sh(&mut cmds, &mut stack, &hash160)? == false {
                    return Ok(false)
                }
            }
        }

        let ele = stack.pop()?;
        Ok(ele.len() > 0) // 0 is empty vec in stack
    }

    pub fn get_hash160_if_p2sh(cmds: &Vec<CommandElement>) -> Option<Vec<u8>> {
        if cmds.len() != 3 {
            return None;
        }
        match (&cmds[0], &cmds[1], &cmds[2]) {
            (CommandElement::Op(ops0), CommandElement::Data(data), CommandElement::Op(ops1)) => {
                if *ops0 == Opcode::OpEqual && *ops1 == Opcode::OpHash160 {
                    Some(data.clone())
                } else {
                    None
                }
            }
            _ => None
        }
    }

    pub fn cmds(&self) -> &Vec<CommandElement> {
        &self.cmds
    }
}

#[cfg(test)]
mod tests {
    use crate::script::{CommandElement, Opcode, Script};
    use primitive_types::U256;
    use crate::transaction::{ZProvider, ZProviderMocker};
    use crate::util::hash;

    #[test]
    fn script_evaluate_p2pk_success() {
        let sec = CommandElement::Data(hex::decode("04887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34").unwrap());
        let op = CommandElement::Op(Opcode::OpChecksig);
        let script_pubkey = Script::new(vec![op, sec]);

        let sig = CommandElement::Data(hex::decode("3045022000eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c022100c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6").unwrap());
        let script_sig = Script::new(vec![sig]);

        let combined_script = script_pubkey + script_sig;

        let z = U256::from_big_endian(&hex::decode("7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d").unwrap());
        let z = Box::new(ZProviderMocker(z)) as Box<dyn ZProvider>;
        let result = combined_script.evaluate(0, &z).unwrap();
        assert!(result);
    }

    #[test]
    fn script_evaluate_p2pk_failed() {
        let sec = CommandElement::Data(hex::decode("04887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34").unwrap());
        let op = CommandElement::Op(Opcode::OpChecksig);
        let script_pubkey = Script::new(vec![op, sec]);

        let sig = CommandElement::Data(hex::decode("3045022000eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c022100c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6").unwrap());
        let script_sig = Script::new(vec![sig]);

        let combined_script = script_pubkey + script_sig;
        let z = U256::from_big_endian(&hex::decode("7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3e").unwrap());
        let z = Box::new(ZProviderMocker(z)) as Box<dyn ZProvider>;
        let result = combined_script.evaluate(0, &z).unwrap();
        assert!(!result);
    }

    #[test]
    fn script_evaluate_p2pkh_for_uncompressed_pk() {
        let script_pubkey = hex::decode("76a914fb6c931433c83e8bb5a4c6588c7fc24c08dac6e388ac").unwrap();
        let (script_pubkey, script_pubkey_used) = Script::parse_raw(&script_pubkey).unwrap();
        assert_eq!(script_pubkey_used, 25);

        let script_sig = hex::decode("473045022000eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c022100c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab64104887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34").unwrap();
        let (script_sig, script_sig_used) = Script::parse_raw(&script_sig).unwrap();
        assert_eq!(script_sig_used, 138);

        let combined_script = script_pubkey + script_sig;
        let z = U256::from_big_endian(&hex::decode("7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d").unwrap());
        let z = Box::new(ZProviderMocker(z)) as Box<dyn ZProvider>;
        let result = combined_script.evaluate(0, &z).unwrap();
        assert!(result);
    }

    #[test]
    fn script_evaluate_add_euqal() {
        let script_pubkey = hex::decode("55935987").unwrap();
        let (script_pubkey, script_pubkey_used) = Script::parse_raw(&script_pubkey).unwrap();
        assert_eq!(script_pubkey_used, 4);

        let script_sig = hex::decode("54").unwrap();
        let (script_sig, script_sig_used) = Script::parse_raw(&script_sig).unwrap();
        assert_eq!(script_sig_used, 1);

        let combined_script = script_pubkey + script_sig;
        let z = Box::new(ZProviderMocker(U256::zero())) as Box<dyn ZProvider>;
        let result = combined_script.evaluate(0, &z).unwrap();
        assert!(result);
    }

    fn script_pubkey_for_check_multisig_and_z() -> (Script, Box<dyn ZProvider>) {
        // new key for test
        use crate::secp256k1::PrivateKey;
        let sk = PrivateKey::new(1.into()).unwrap();
        let sec0 = CommandElement::Data(sk.pk_point().sec_compressed().unwrap());

        let sec1 = CommandElement::Data(hex::decode("04887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34").unwrap());
        let op_checkmultisig = CommandElement::Op(Opcode::OpCheckmultisig);
        let op_2 = CommandElement::Op(Opcode::Op2);
        let op_1 = CommandElement::Op(Opcode::Op1);
        let script_pubkey = Script::new(vec![op_checkmultisig, op_2, sec0, sec1, op_1]);

        let z = U256::from_big_endian(&hex::decode("7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d").unwrap());
        let z = Box::new(ZProviderMocker(z)) as Box<dyn ZProvider>;

        (script_pubkey, z)
    }

    #[test]
    fn script_check_multisig_success() {
        let sig = CommandElement::Data(hex::decode("3045022000eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c022100c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6").unwrap());
        let op_0 = CommandElement::Op(Opcode::Op0); // for satoshi bug
        let script_sig = Script::new(vec![sig, op_0]);

        let (script_pubkey, z) = script_pubkey_for_check_multisig_and_z();
        let combined_script = script_pubkey + script_sig;

        let result = combined_script.evaluate(0, &z).unwrap();
        assert!(result);
    }

    #[test]
    fn script_check_multisig_failed() {
        let sig = CommandElement::Data(hex::decode("3045022000eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c022100c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab7").unwrap());
        let op_0 = CommandElement::Op(Opcode::Op0); // for satoshi bug
        let script_sig = Script::new(vec![sig, op_0]);

        let (script_pubkey, z) = script_pubkey_for_check_multisig_and_z();
        let combined_script = script_pubkey + script_sig;

        let result = combined_script.evaluate(0, &z).unwrap();
        assert!(!result);
    }

    #[test]
    fn script_get_hash160_if_p2sh_true() {
        let cmds = vec![Opcode::OpEqual.into(), vec![0u8].into(), Opcode::OpHash160.into()];
        let hash160 = Script::get_hash160_if_p2sh(&cmds);
        assert_eq!(hash160, Some(vec![0u8]));
    }

    #[test]
    fn script_get_hash160_if_p2sh_false() {
        let cmds = vec![Opcode::OpEqualverify.into(), vec![0u8].into(), Opcode::OpHash160.into()];
        let hash160 = Script::get_hash160_if_p2sh(&cmds);
        assert!(hash160.is_none());
    }

    #[test]
    fn script_p2sh_success() {
        use crate::secp256k1::PrivateKey;
        // test data
        let z_raw = U256::one();
        let z = ZProviderMocker(z_raw);

        let sk1 = PrivateKey::new(911.into()).unwrap();
        let sk1_pk = sk1.pk_point().clone();
        let sk1_sig = sk1.sign_deterministic(z_raw).unwrap();

        let sk2 = PrivateKey::new(500.into()).unwrap();
        let sk2_pk = sk2.pk_point().clone();
        let sk2_sig = sk2.sign_deterministic(z_raw).unwrap();

        let script_redeem = Script::new(
            vec![
                Opcode::OpCheckmultisig.into(),
                Opcode::Op2.into(),
                sk1_pk.sec_compressed().unwrap().into(),
                sk2_pk.sec_compressed().unwrap().into(),
                Opcode::Op2.into(),
            ]
        );

        let script_redeem_hash = hash::hash160(&script_redeem.serialize().unwrap());
        let script_pubkey = Script::new(
            vec![
                Opcode::OpEqual.into(),
                script_redeem_hash.to_vec().into(),
                Opcode::OpHash160.into(),
            ]
        );

        let script_sig = Script::new(
            vec![
                script_redeem.serialize().unwrap().into(),
                [hex::decode(sk1_sig.der()).unwrap(), vec![1u8]].concat().into(), // 1u8 for SigHash::All
                [hex::decode(sk2_sig.der()).unwrap(), vec![1u8]].concat().into(),
                Opcode::Op0.into()
            ]
        );

        let combined_script = script_pubkey + script_sig;
        let z = Box::new(z) as Box<dyn ZProvider>;
        assert!(combined_script.evaluate(0, &z).unwrap());
    }

    #[test]
    fn script_p2sh_fail() {
        use crate::secp256k1::PrivateKey;
        // test data
        let z_raw = U256::one();
        let z = ZProviderMocker(z_raw);

        let sk1 = PrivateKey::new(911.into()).unwrap();
        let sk1_pk = sk1.pk_point().clone();
        let sk1_sig = sk1.sign_deterministic(z_raw).unwrap();

        let sk2 = PrivateKey::new(500.into()).unwrap();
        let sk2_pk = sk2.pk_point().clone();
        let sk2_sig = sk2.sign_deterministic(222.into()).unwrap(); // sign different content

        let script_redeem = Script::new(
            vec![
                Opcode::OpCheckmultisig.into(),
                Opcode::Op2.into(),
                sk1_pk.sec_compressed().unwrap().into(),
                sk2_pk.sec_compressed().unwrap().into(),
                Opcode::Op2.into(),
            ]
        );

        let script_redeem_hash = hash::hash160(&script_redeem.serialize().unwrap());
        let script_pubkey = Script::new(
            vec![
                Opcode::OpEqual.into(),
                script_redeem_hash.to_vec().into(),
                Opcode::OpHash160.into(),
            ]
        );

        let script_sig = Script::new(
            vec![
                script_redeem.serialize().unwrap().into(),
                [hex::decode(sk1_sig.der()).unwrap(), vec![1u8]].concat().into(), // 1u8 for SigHash::All
                [hex::decode(sk2_sig.der()).unwrap(), vec![1u8]].concat().into(),
                Opcode::Op0.into()
            ]
        );

        let combined_script = script_pubkey + script_sig;
        let z = Box::new(z) as Box<dyn ZProvider>;
        assert!(!combined_script.evaluate(0, &z).unwrap());
    }
}
