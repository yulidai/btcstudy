use std::convert::TryFrom;
use super::{Error, TxIn, TxOut, Version, LockTime, SigHash};
use crate::util::{
    hash::{self, Hash256Value},
    varint,
    converter,
    Reader,
};
use crate::script::Script;

#[derive(Debug, Clone)]
pub struct SegwitField {
    pub marker: u8,
    pub flag: u8,
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub version: Version,
    pub inputs: Vec<TxIn>,
    pub outputs: Vec<TxOut>,
    pub locktime: LockTime,
    pub segwit: Option<SegwitField>,
}

impl Transaction {
    pub fn id(&self) -> Result<Hash256Value, Error> {
        let bytes = self.serialize_legacy()?;
        let mut result = hash::hash256(&bytes);
        result.reverse(); // little endian
        result = hash::convert_slice_into_hash256(&result);

        Ok(result)
    }

    pub fn parse(bytes: &[u8]) -> Result<Self, Error> {
        let mut reader = Reader::new(bytes);
        let is_segwit = reader.more(5)?[4] == 0u8;
        reader.reset();

        if is_segwit {
            Self::parse_segwit(&mut reader)
        } else {
            Self::parse_legacy(&mut reader)
        }
    }

    pub fn parse_segwit(reader: &mut Reader) -> Result<Self, Error> {
        let version = Version::parse(&reader.more(4)?)?;
        let marker = reader.more(1)?[0];
        let flag = reader.more(1)?[0];
        if marker != 0u8 || flag == 0u8 {
            return Err(Error::InvalidSegwitTx);
        }
        let segwit = Some( SegwitField { marker, flag } );

        let input_count = varint::decode_with_reader(reader)?;
        let mut inputs = Vec::new();
        for _ in 0..input_count {
            let input = TxIn::parse_reader(reader)?;
            inputs.push(input);
        }

        let output_count = varint::decode_with_reader(reader)?;
        let mut outputs = Vec::new();
        for _ in 0..output_count {
            let output = TxOut::parse_reader(reader)?;
            outputs.push(output);
        }

        for input in &mut inputs {
            let num_items = varint::decode_with_reader(reader)?;
            let mut items = Vec::new();
            for _ in 0..num_items {
                let item_len = varint::decode_with_reader(reader)?;
                if item_len == 0 {
                    items.push(vec![]); // why should push vec![0] in book?
                } else {
                    items.push(reader.more(item_len)?.to_vec());
                }
            }
            input.set_witness(items);
        }
        let locktime = LockTime::parse(reader.more(4)?)?;

        Ok(Self { version, inputs, outputs, locktime, segwit })
    }

    pub fn parse_legacy(reader: &mut Reader) -> Result<Self, Error> {
        let version = Version::parse(&reader.more(4)?)?;

        let input_count = varint::decode_with_reader(reader)?;
        let mut inputs = Vec::new();
        for _ in 0..input_count {
            let input = TxIn::parse_reader(reader)?;
            inputs.push(input);
        }

        let output_count = varint::decode_with_reader(reader)?;
        let mut outputs = Vec::new();
        for _ in 0..output_count {
            let output = TxOut::parse_reader(reader)?;
            outputs.push(output);
        }

        let locktime = LockTime::parse(reader.more(4)?)?;
        let segwit = None;

        Ok(Self { version, inputs, outputs, locktime, segwit })
    }

    pub fn serialize(&self) -> Result<Vec<u8>, Error> {
        let input_count = self.inputs.len();
        let output_count = self.outputs.len();

        let mut result = Vec::new();
        result.append(&mut self.version.serialize().to_vec());
        if let Some(ref segwit) = &self.segwit {
            result.push(segwit.marker);
            result.push(segwit.flag);
        }
        result.append(&mut varint::encode(u64::try_from(input_count).expect("failed to convert usize into u64 within Transaction::parse()")));
        for i in 0..input_count {
            result.append(&mut self.inputs[i].serialize()?);
        }
        result.append(&mut varint::encode(u64::try_from(output_count).expect("failed to convert usize into u64 within Transaction::parse()")));
        for i in 0..output_count {
            result.append(&mut self.outputs[i].serialize()?);
        }

        for input in &self.inputs {
            if input.witness.len() == 0 {
                continue;
            }
            result.append(&mut varint::encode( converter::usize_into_u64(input.witness.len())? ));
            for item in &input.witness {
                result.append(&mut varint::encode( converter::usize_into_u64(item.len())? ));
                result.append(&mut item.clone());
            }
        }

        result.append(&mut self.locktime.serialize().to_vec());

        Ok(result)
    }

    // for calculate TxHash
    pub fn serialize_legacy(&self) -> Result<Vec<u8>, Error> {
        let input_count = self.inputs.len();
        let output_count = self.outputs.len();

        let mut result = Vec::new();
        result.append(&mut self.version.serialize().to_vec());
        result.append(&mut varint::encode(u64::try_from(input_count).expect("failed to convert usize into u64 within Transaction::parse()")));
        for i in 0..input_count {
            result.append(&mut self.inputs[i].serialize()?);
        }
        result.append(&mut varint::encode(u64::try_from(output_count).expect("failed to convert usize into u64 within Transaction::parse()")));
        for i in 0..output_count {
            result.append(&mut self.outputs[i].serialize()?);
        }
        result.append(&mut self.locktime.serialize().to_vec());

        Ok(result)
    }

    pub fn fee(&self) -> Result<u64, Error> {
        let mut amount_in = 0;
        for input in &self.inputs {
            amount_in += input.value()?
        }
        let mut amount_out = 0;
        for output in &self.outputs {
            amount_out += output.amount()
        }

        if amount_in < amount_out {
            Err(Error::InvalidTxFee)
        } else {
            Ok(amount_in - amount_out)
        }
    }

    pub fn is_coinbase(&self) -> bool {
        self.inputs.len() == 1 && self.inputs[0].is_coinbase()
    }

    // TODO move to intermediator
    pub fn get_block_height_from_coinbase(&self) -> Result<u32, Error> {
        if !self.is_coinbase() {
            return Err(Error::NotCoinbaseTx);
        }
        let script = Script::parse_raw(&self.inputs[0].script).map_err(|_| Error::InvalidScript)?;
        let height = script.get_block_height().map_err(|_| Error::InvalidBlockHeightInCoinbase)?;

        Ok(height)
    }

    pub fn hash_prevouts(&self, sighash: SigHash) -> Result<Hash256Value, Error> {
        if sighash.is_anyone_can_pay() {
            return Err(Error::InvalidSigHash);
        }
        let mut result = Vec::new();
        for input in &self.inputs {
            let mut prev_tx = input.prev_tx.to_vec();
            prev_tx.reverse();
            result.append(&mut prev_tx);
            result.append(&mut input.prev_index.serialize().to_vec());
        }
        Ok(hash::hash256(&result))
    }

    pub fn hash_sequence(&self, sighash: SigHash) -> Result<Hash256Value, Error> {
        if sighash != SigHash::All {
            return Ok([0u8; 32]);
        }
        let mut result = Vec::new();
        for input in &self.inputs {
            result.append(&mut input.sequence.serialize().to_vec());
        }
        Ok(hash::hash256(&result))
    }

    pub fn hash_outputs(&self, input_index: usize, sighash: SigHash) -> Result<Hash256Value, Error> {
        match sighash {
            SigHash::All => {
                let mut result = Vec::new();
                for output in &self.outputs {
                    result.append(&mut output.serialize()?);
                }
                Ok(hash::hash256(&result))
            },
            SigHash::Single if input_index < self.outputs.len() => {
                let mut result = Vec::new();
                result.append(&mut self.outputs[input_index].serialize()?);
                Ok(hash::hash256(&result))
            },
            SigHash::Single => Ok([0u8; 32]),
            _ => Err(Error::InvalidSigHash) // impl in the future
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::transaction::{TxFetcher, Transaction};

    #[test]
    fn transaction_parse_legacy() {
        let bytes = hex::decode("0100000001813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303c6a989c7d1000000006b483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278afeffffff02a135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac99c39800000000001976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac19430600").unwrap();
        let tx = Transaction::parse(&bytes).unwrap();
        let bytes_serialized = tx.serialize().unwrap();
        assert_eq!(bytes, bytes_serialized);
    }

    #[test]
    fn transaction_parse_segwit() {
        let bytes = hex::decode("0100000000010115e180dc28a2327e687facc33f10f2a20da717e5548406f7ae8b4c811072f8560100000000ffffffff0100b4f505000000001976a9141d7cd6c75c2e86f4cbf98eaed221b30bd9a0b92888ac02483045022100df7b7e5cda14ddf91290e02ea10786e03eb11ee36ec02dd862fe9a326bbcb7fd02203f5b4496b667e6e281cc654a2da9e4f08660c620a1051337fa8965f727eb19190121038262a6c6cec93c2d3ecd6c6072efea86d02ff8e3328bbd0242b20af3425990ac00000000").unwrap();
        let tx = Transaction::parse(&bytes).unwrap();
        let bytes_serialized = tx.serialize().unwrap();
        assert_eq!(bytes, bytes_serialized);
    }

    #[test]
    fn transaction_id() {
        let tx_id = "f4184fc596403b9d638783cf57adfe4c75c605f6356fbc91338530e9831e9e16";
        let mut fetcher = TxFetcher::new();
        let mut tx_hash = [0u8; 32];
        tx_hash.copy_from_slice(&hex::decode(&tx_id).unwrap());
        let first_tx = fetcher.fetch(&tx_hash, false, false).unwrap();
        assert_eq!(hex::encode(first_tx.id().unwrap()), tx_id);
    }

    #[test]
    fn transaction_fee_1() {
        let mut fetcher = TxFetcher::new();
        let mut tx_hash = [0u8; 32];
        tx_hash.copy_from_slice(&hex::decode("f4184fc596403b9d638783cf57adfe4c75c605f6356fbc91338530e9831e9e16").unwrap());
        let first_tx = fetcher.fetch(&tx_hash, false, false).unwrap();
        assert_eq!(first_tx.fee().unwrap(), 0);
    }

    fn get_tx_from_parsed() -> Transaction {
        let bytes = hex::decode("0100000001813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf830\
            3c6a989c7d1000000006b483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccf\
            cf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8\
            e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278\
            afeffffff02a135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88a\
            c99c39800000000001976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac19430600").unwrap();
        Transaction::parse(&bytes).unwrap()
    }

    #[test]
    fn transaction_fee_2() {
        let tx = get_tx_from_parsed();
        let fee = tx.fee().unwrap();

        assert!(fee > 0);
    }
}
