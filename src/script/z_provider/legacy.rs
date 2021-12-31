use crate::util::{
    hash::{self, Hash256Value},
};
use crate::script::{Script, Error, ZProvider};
use crate::transaction::{Transaction, SigHash};

pub struct TransactionLegacyZProvider(pub Transaction);

impl From<Transaction> for TransactionLegacyZProvider {
    fn from(t: Transaction) -> Self {
        Self(t)
    }
}

impl ZProvider for TransactionLegacyZProvider {
    fn z(&mut self, _input: usize, sighash: SigHash, _redeem_script: Option<Script>, _witness_script: Option<Script>) -> Result<Hash256Value, Error> {
        match sighash {
            SigHash::All => {
                let mut tx = self.0.clone();
                for input in &mut tx.inputs {
                    let output_ref = input.get_output_ref()?;
                    input.script = output_ref.script().clone();
                }
                let mut tx_bytes = tx.serialize()?;
                tx_bytes.append(&mut sighash.serialize().to_vec());

                Ok(hash::hash256(&tx_bytes))
            },
            _ => Err(Error::NotImpl),
        }
    }

    // for test
    fn z_without_replace_script(&self, _index: usize, sighash: SigHash, _redeem_script: Option<Script>, _witness_script: Option<Script>) -> Result<Hash256Value, Error> {
        match sighash {
            SigHash::All => {
                let mut tx_bytes = self.0.serialize()?;
                tx_bytes.append(&mut sighash.serialize().to_vec());

                Ok(hash::hash256(&tx_bytes))
            },
            _ => Err(Error::NotImpl),
        }
    }
}
