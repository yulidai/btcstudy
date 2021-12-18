use crate::secp256k1::S256Point;
use crate::util::hash::Hash160Value;
use super::{CommandElement, Opcode, Script};

pub struct ScriptBuilder;

impl ScriptBuilder {
    pub fn p2pk(pk: &S256Point) -> Result<Script, &'static str> {
        let mut commands = vec![CommandElement::Op(Opcode::OpChecksig)];
        let mut pk_bytes = match pk.sec_compressed() { // default to compressed
            Some(bytes) => bytes,
            None => return Err("pk is infinity"),
        };
        pk_bytes.reverse();
        commands.push(CommandElement::Data(pk_bytes));

        Ok(Script::new(commands))
    }

    pub fn p2pkh(pkh: &Hash160Value) -> Script {
        let mut commands = vec![CommandElement::Op(Opcode::OpChecksig), CommandElement::Op(Opcode::OpEqualverify)];
        commands.push(CommandElement::Data(pkh.to_vec()));
        commands.push(CommandElement::Op(Opcode::OpHash160));
        commands.push(CommandElement::Op(Opcode::OpDup));

        Script::new(commands)
    }
}

#[cfg(test)]
mod tests {
    use crate::util::{base58, hash};
    use super::ScriptBuilder;

    #[test]
    fn script_builder_p2pkh() {
        let receiver_byte = base58::decode_btc_addr("mzx5YhAH9kNHtcN481u6WkjeHjYtVeKVh2").unwrap();
        let receiver_h160 = hash::convert_slice_into_hash160(&receiver_byte[1..]); // skip network byte
        let script = ScriptBuilder::p2pkh(&receiver_h160);
        assert_eq!(hex::encode(script.serialize().unwrap()), "19ac8814d52ad7ca9b3d096a38e752c2018e6fbc40cdf26fa976");
    }
}
