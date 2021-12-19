use crate::util::{math, hash::Hash256Value};
use crate::script::Script;
use super::{Error, Version, TxFetcher, TxOut};
use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub struct TxIn {
    pub prev_tx: Hash256Value,
    pub prev_index: PrevIndex,
    pub script: Script,
    pub sequence: Sequence,
}

impl TxIn {
    pub fn parse(bytes: &[u8]) -> Result<(Self, usize), Error> {
        let len = bytes.len();
        let mut index = 0;

        // prev_tx(hash)
        index = math::check_range_add_with_max(index, 32, len)?;
        let mut prev_tx: Hash256Value = Default::default();
        prev_tx.copy_from_slice(&bytes[(index-32)..index]);
        prev_tx.reverse(); // little endian
        // prev_in
        index = math::check_range_add_with_max(index, 4, len)?;
        let prev_index = PrevIndex::parse(&bytes[(index-4)..index])?;
        // script
        let (script, used) = Script::parse(&bytes[index..]).expect("script parse error");
        index = math::check_range_add_with_max(index, used, len)?;
        // sequence
        index = math::check_range_add_with_max(index, 4, len)?;
        let sequence = Sequence::parse(&bytes[(index-4)..index])?;

        let result = Self { prev_tx, prev_index, script, sequence };

        Ok((result, index))
    }

    pub fn serialize(&self) -> Result<Vec<u8>, Error> {
        let mut prev_tx = self.prev_tx.to_vec();
        prev_tx.reverse(); // little endian

        let mut result = Vec::new();
        result.append(&mut prev_tx);
        result.append(&mut self.prev_index.serialize().to_vec());
        result.append(&mut self.script.serialize().expect("script serialize error"));
        result.append(&mut self.sequence.serialize().to_vec());

        Ok(result)
    }

    pub fn value(&self) -> Result<u64, Error> {
        let output = self.get_output_ref()?;
        Ok(output.amount())
    }

    pub fn get_output_ref(&self) -> Result<TxOut, Error> {
        let tx = TxFetcher::fetch_without_cache(&self.prev_tx, false)?;
        let prev_index = usize::try_from(self.prev_index.value()).expect("failed convert u32 into usize");
        if prev_index >= tx.outputs.len() {
            return Err(Error::InvalidTxIn);
        }
        Ok(tx.outputs[prev_index].clone())
    }
}

pub type Sequence = Version;
pub type PrevIndex = Version;

#[cfg(test)]
mod tests {
    use super::TxIn;

    #[test]
    fn tx_in_parse_serialize() {
        let bytes = hex::decode("813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303c6a989c7d1000000006b483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278afeffffff").unwrap();
        let (tx_in, used) = TxIn::parse(&bytes).unwrap();
        assert_eq!(used, 148);

        let bytes_serialized = tx_in.serialize().unwrap();
        assert_eq!(bytes, bytes_serialized);
    }
}
