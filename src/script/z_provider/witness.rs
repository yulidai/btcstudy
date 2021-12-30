use crate::util::{
    hash::{self, Hash256Value},
};
use crate::script::{Script, ScriptBuilder, Error, ZProvider};
use crate::transaction::{Transaction, SigHash};

pub struct TransactionWitnessP2pkhZProvider(pub Transaction);

impl From<Transaction> for TransactionWitnessP2pkhZProvider {
    fn from(t: Transaction) -> Self {
        Self(t)
    }
}

impl ZProvider for TransactionWitnessP2pkhZProvider {
    fn z(&self, index: usize, sighash: SigHash, redeem_script: Option<Script>, witness_script: Option<Script>) -> Result<Hash256Value, Error> {
        let input = &self.0.inputs[index];
        match sighash {
            SigHash::All => {
                let mut result = Vec::new();
                result.append(&mut self.0.version.serialize().to_vec());
                result.append(&mut self.0.hash_prevouts(sighash)?.to_vec());
                result.append(&mut self.0.hash_sequence(sighash)?.to_vec());

                let mut input_prev_tx = input.prev_tx.to_vec();
                input_prev_tx.reverse();
                result.append(&mut input_prev_tx);
                result.append(&mut input.prev_index.serialize().to_vec());

                let mut script_code = if witness_script.is_some() {
                    witness_script.unwrap().serialize()?
                } else if redeem_script.is_some() {
                    // TODO
                    return Err(Error::NotImpl);
                } else {
                    let prevout = input.get_output_ref()?;
                    let script_pubkey = Script::parse(prevout.script())?; //TODO define ScriptGroup for P2PKH
                    let pk_hash = hash::convert_slice_into_hash160(&script_pubkey.get_bottom_as_data().unwrap());
                    let script_pubkey = ScriptBuilder::p2pkh(&pk_hash);
                    script_pubkey.serialize()?
                };
                result.append(&mut script_code);

                result.append(&mut input.value()?.to_le_bytes().to_vec());
                result.append(&mut input.sequence.serialize().to_vec());
                result.append(&mut self.0.hash_outputs(sighash)?.to_vec());
                result.append(&mut self.0.locktime.serialize().to_vec());
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
