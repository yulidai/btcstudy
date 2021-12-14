use crate::util::math;
use crate::script::Script;
use super::Error;

#[derive(Debug)]
pub struct TxOut {
    amount: u64,
    script: Script,
}

impl TxOut {
    pub fn parse(bytes: &[u8]) -> Result<(Self, usize), Error> {
        let len = bytes.len();
        let mut index = 0;

        index = math::check_range_add_with_max(index, 8, len)?;
        let mut param: [u8; 8] = Default::default();
        param.copy_from_slice(&bytes[(index-8)..index]);
        let amount = u64::from_le_bytes(param);

        let (script, used) = Script::parse(&bytes[index..])?;
        index = math::check_range_add_with_max(index, used, len)?;

        let result = Self { amount, script };
        Ok((result, index))
    }

    pub fn serialize(&self) -> Result<Vec<u8>, Error> {
        let mut result = Vec::new();
        result.append(&mut self.amount.to_le_bytes().to_vec());
        result.append(&mut self.script.serialize()?);

        Ok(result)
    }
}


#[cfg(test)]
mod tests {
    use super::TxOut;

    #[test]
    fn tx_out_parse() {
        let bytes = hex::decode("a135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac").unwrap();
        let (_tx_out, used) = TxOut::parse(&bytes).unwrap();
        assert_eq!(used, 34);
    }
}