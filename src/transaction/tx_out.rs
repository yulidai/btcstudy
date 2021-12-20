use crate::util::{varint, Reader};
use super::Error;
use std::fmt;
use std::convert::TryInto;

#[derive(Clone)]
pub struct TxOut {
    amount: u64,
    script: Vec<u8>,
}

impl TxOut {
    pub fn new(amount: u64, script: Vec<u8>) -> Self {
        Self { amount, script }
    }

    pub fn parse(bytes: &[u8]) -> Result<Self, Error> {
        let mut reader = Reader::new(bytes);
        Self::parse_reader(&mut reader)
    }

    pub fn parse_reader(reader: &mut Reader) -> Result<Self, Error> {
        let mut amount: [u8; 8] = Default::default();
        amount.copy_from_slice(reader.more(8)?);
        let amount = u64::from_le_bytes(amount);

        let script_len = varint::decode_with_reader(reader)?;
        let script = reader.more(script_len)?.to_vec();

        Ok(Self { amount, script })
    }

    pub fn serialize(&self) -> Result<Vec<u8>, Error> {
        let mut result = Vec::new();
        result.append(&mut self.amount.to_le_bytes().to_vec());
        result.append(&mut varint::encode(self.script.len().try_into().unwrap()));
        result.append(&mut self.script.clone());

        Ok(result)
    }

    pub fn amount(&self) -> u64 {
        self.amount
    }

    pub fn script(&self) -> &Vec<u8> {
        &self.script
    }
}

impl fmt::Debug for TxOut {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TxOut")
            .field("amount", &self.amount)
            .field("script", &hex::encode(&self.script))
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::TxOut;

    #[test]
    fn tx_out_parse_serialize() {
        let bytes = hex::decode("a135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac").unwrap();
        let tx_out = TxOut::parse(&bytes).unwrap();
        let bytes_serialized = tx_out.serialize().unwrap();
        assert_eq!(bytes, bytes_serialized);
    }
}