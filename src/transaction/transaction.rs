use super::{Error, TxIn, TxOut, Version, LockTime};
use crate::util::{math, varint};

#[derive(Debug)]
pub struct Transaction {
    version: Version,
    inputs: Vec<TxIn>,
    outputs: Vec<TxOut>,
    locktime: LockTime,
}

impl Transaction {
    pub fn parse(bytes: &[u8]) -> Result<(Self, usize), Error> {
        let len = bytes.len();
        let mut index = 0;

        index = math::check_range_add_with_max(index, 4, len)?;
        let version = Version::parse(&bytes[(index-4)..index])?;

        let (input_count, used) = varint::decode(&bytes[index..])?;
        index = math::check_range_add_with_max(index, used as usize, len)?;
        let mut inputs = Vec::new();
        for _ in 0..input_count {
            let (input, used) = TxIn::parse(&bytes[index..])?;
            index += used;
            inputs.push(input);
        }

        let (output_count, used) = varint::decode(&bytes[index..])?;
        index = math::check_range_add_with_max(index, used as usize, len)?;
        let mut outputs = Vec::new();
        for _ in 0..output_count {
            let (output, used) = TxOut::parse(&bytes[index..])?;
            index += used;
            outputs.push(output);
        }

        index = math::check_range_add_with_max(index, 4, len)?;
        let locktime = LockTime::parse(&bytes[(index-4)..index])?;

        let result = Self { version, inputs, outputs, locktime };
        Ok((result, index))
    }
}

#[cfg(test)]
mod tests {
    use super::Transaction;

    #[test]
    fn transaction_parse() {
        let bytes = hex::decode("0100000001813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303c6a989c7d1000000006b483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278afeffffff02a135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac99c39800000000001976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac19430600").unwrap();
        let (tx, used) = Transaction::parse(&bytes).unwrap();
        println!("tx: {:?}", tx);
        assert_eq!(used, 226);
    }
}
