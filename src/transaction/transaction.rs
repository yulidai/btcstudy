use std::convert::TryFrom;
use super::{Error, TxIn, TxOut, Version, LockTime, SigHash, ZProvider};
use crate::util::{
    hash::{self, Hash256Value},
    varint,
    Reader,
};

#[derive(Debug, Clone)]
pub struct Transaction {
    pub version: Version,
    pub inputs: Vec<TxIn>,
    pub outputs: Vec<TxOut>,
    pub locktime: LockTime,
}

impl Transaction {
    pub fn id(&self) -> Result<Hash256Value, Error> {
        let bytes = self.serialize()?;
        let mut result = hash::hash256(&bytes);
        result.reverse(); // little endian
        result = hash::convert_slice_into_hash256(&result);

        Ok(result)
    }

    pub fn parse(bytes: &[u8]) -> Result<Self, Error> {
        println!(">>0 len: {}", bytes.len());
        let mut reader = Reader::new(bytes);
        Self::parse_reader(&mut reader)
    }

    pub fn parse_reader(reader: &mut Reader) -> Result<Self, Error> {
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

        Ok(Self { version, inputs, outputs, locktime })
    }

    pub fn serialize(&self) -> Result<Vec<u8>, Error> {
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
}

impl ZProvider for Transaction {
    fn z(&self, _input: usize, sighash: SigHash) -> Result<Hash256Value, Error> {
        match sighash {
            SigHash::All => {
                let mut tx = self.clone();
                for input in &mut tx.inputs {
                    let output_ref = input.get_output_ref()?;
                    input.script = output_ref.script().clone();
                }
                let mut tx_bytes = tx.serialize()?;
                tx_bytes.append(&mut sighash.serialize().to_vec());

                Ok(hash::hash256(&tx_bytes))
            },
            _ => Err(Error::InvalidSigHash),
        }
    }

    // for test
    fn z_without_replace_script(&self, _index: usize, sighash: SigHash) -> Result<Hash256Value, Error> {
        match sighash {
            SigHash::All => {
                let mut tx_bytes = self.serialize()?;
                tx_bytes.append(&mut sighash.serialize().to_vec());

                Ok(hash::hash256(&tx_bytes))
            },
            _ => Err(Error::InvalidSigHash),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::transaction::{TxFetcher, Transaction, SigHash, ZProvider};

    #[test]
    fn transaction_parse() {
        let bytes = hex::decode("0100000001813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303c6a989c7d1000000006b483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278afeffffff02a135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac99c39800000000001976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac19430600").unwrap();
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

    #[test]
    fn transaction_z_sighash_all() {
        let tx = get_tx_from_parsed();
        let z = tx.z(0, SigHash::All).unwrap();
        assert_eq!("27e0c5994dec7824e56dec6b2fcb342eb7cdb0d0957c2fce9882f715e85d81a6", hex::encode(z));
    }
}
