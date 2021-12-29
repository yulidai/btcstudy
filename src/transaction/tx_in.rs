use crate::util::{
    hash::{self, Hash256Value},
    varint,
    Reader,
};
use super::{Error, Version, TxFetcher, TxOut};
use std::convert::{TryFrom, TryInto};
use std::fmt;

#[derive(Clone)]
pub struct TxIn {
    pub prev_tx: Hash256Value,
    pub prev_index: PrevIndex,
    pub script: Vec<u8>,
    pub sequence: Sequence,
    pub witness: Option<Vec<Vec<u8>>>,
}

impl TxIn {
    pub fn parse(bytes: &[u8]) -> Result<Self, Error> {
        let mut reader = Reader::new(bytes);
        Self::parse_reader(&mut reader)
    }

    pub fn parse_reader(reader: &mut Reader) -> Result<Self, Error> {
        let mut prev_tx = hash::convert_slice_into_hash256(reader.more(32)?);
        prev_tx.reverse(); // little endian
        let prev_index = PrevIndex::parse(reader.more(4)?)?;

        let script_len = varint::decode_with_reader(reader)?;
        let script = reader.more(script_len)?.to_vec();

        let sequence = Sequence::parse(reader.more(4)?)?;
        let witness = None;

        Ok(Self { prev_tx, prev_index, script, sequence, witness })
    }

    pub fn serialize(&self) -> Result<Vec<u8>, Error> {
        let mut prev_tx = self.prev_tx.to_vec();
        prev_tx.reverse(); // little endian

        let mut result = Vec::new();
        result.append(&mut prev_tx);
        result.append(&mut self.prev_index.serialize().to_vec());
        result.append(&mut varint::encode(self.script.len().try_into().unwrap()));
        result.append(&mut self.script.clone());
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

    pub fn is_coinbase(&self) -> bool {
        self.prev_tx == [0u8; 32] && self.prev_index.value() == u32::MAX
    }

    pub fn set_witness(&mut self, witness: Vec<Vec<u8>>) {
        self.witness = Some(witness)
    }
}

impl fmt::Debug for TxIn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let witness = match self.witness {
            None => "None".to_string(),
            Some(ref witness) => {
                let mut r = String::new();
                r.push('[');
                for item in witness {
                    r += &format!("{}, ", hex::encode(item));
                }
                r.push(']');
                r
            }
        };
        f.debug_struct("TxIn")
            .field("prev_tx", &hex::encode(&self.prev_tx))
            .field("pref_index", &self.prev_index.value())
            .field("script", &hex::encode(&self.script))
            .field("sequence", &self.sequence.value())
            .field("witness", &witness)
            .finish()
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
        let tx_in = TxIn::parse(&bytes).unwrap();

        let bytes_serialized = tx_in.serialize().unwrap();
        assert_eq!(bytes, bytes_serialized);
    }
}
