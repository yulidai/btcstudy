use crate::util::{
    hash::{self, Hash256Value},
};
use crate::script::{Script, Error, ZProvider};
use crate::transaction::{Transaction, TxOut, SigHash};
use std::collections::HashMap;

pub struct TransactionWitnessP2pkhZProvider {
    pub tx: Transaction,
    pub prevout_cache: HashMap::<Vec<u8>, (TxOut, Script)>, //  Script is script_pubkey after convert for p2sh or p2wpkh
}

impl From<Transaction> for TransactionWitnessP2pkhZProvider {
    fn from(tx: Transaction) -> Self {
        let prevout_cache = HashMap::new();
        Self { tx, prevout_cache }
    }
}

impl ZProvider for TransactionWitnessP2pkhZProvider {
    fn z(&self, index: usize, sighash: SigHash, _redeem_script: Option<Script>, _witness_script: Option<Script>) -> Result<Hash256Value, Error> {
        let input = &self.tx.inputs[index];
        let cache_key = [input.prev_tx.to_vec(), input.prev_index.serialize().to_vec()].concat();
        let (prevout, script_pubkey) = match self.prevout_cache.get(&cache_key) {
            Some(data) => data.clone(),
            None => {
                let prevout = input.get_output_ref()?;
                let script_pubkey = Script::parse_raw(prevout.script())?;
                (prevout, script_pubkey)
            }
        };
        match sighash {
            SigHash::All => {
                let mut result = Vec::new();
                result.append(&mut self.tx.version.serialize().to_vec());
                result.append(&mut self.tx.hash_prevouts(sighash)?.to_vec());
                result.append(&mut self.tx.hash_sequence(sighash)?.to_vec());

                let mut prevout_tx = input.prev_tx.to_vec();
                prevout_tx.reverse();
                result.append(&mut prevout_tx);
                result.append(&mut input.prev_index.serialize().to_vec());

                // p2pkh only now
                // TODO p2wsh
                let mut script_code = script_pubkey.serialize()?;
                result.append(&mut script_code);
                result.append(&mut prevout.amount().to_le_bytes().to_vec());
                result.append(&mut input.sequence.serialize().to_vec());
                result.append(&mut self.tx.hash_outputs(sighash)?.to_vec());
                result.append(&mut self.tx.locktime.serialize().to_vec());
                result.append(&mut sighash.serialize().to_vec());

                Ok(hash::hash256(&result))
            },
            _ => Err(Error::NotImpl),
        }
    }

    // for test
    fn z_without_replace_script(&self, _index: usize, _sighash: SigHash, _redeem_script: Option<Script>, _witness_script: Option<Script>) -> Result<Hash256Value, Error> {
        Ok([0u8; 32])
    }
}

#[cfg(test)]
mod tests {
    use crate::transaction::{Transaction, TxOut, SigHash};
    use crate::script::operator;

    #[test]
    fn witness_p2pkh_z_provider() {
        let bytes = hex::decode("0100000002fff7f7881a8099afa6940d42d1e7f6362bec38171ea3edf433541db4e4ad969f0000000000eeffffffef51e1b804cc89d182d279655c3aa89e815b1b309fe287d9b2b55d57b90ec68a0100000000ffffffff02202cb206000000001976a9148280b37df378db99f66f85c95a783a76ac7a6d5988ac9093510d000000001976a9143bde42dbee7e4dbe6a21b2d50ce2f0167faa815988ac11000000").unwrap();
        let tx = Transaction::parse(&bytes).unwrap();

        let index = 1;
        let prevout_bytes = hex::decode("0046c323000000001600141d0f172a0ecb48aee1be1f2687d2963ae33f71a1").unwrap();
        let prevout = TxOut::parse(&prevout_bytes).unwrap();

        let (_script_pubkey, _script_sig, provider) = operator::convert_script(&tx, index, Some(prevout)).unwrap();
        let z = provider.z(index, SigHash::All, None, None).unwrap();
        assert_eq!("c37af31116d1b27caf68aae9e3ac82f1477929014d5b917657d0eb49478cb670", hex::encode(z));
    }
}
