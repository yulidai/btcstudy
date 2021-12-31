use crate::util::hash::Hash256Value;
use crate::script::Script;
use super::Error;
use crate::transaction::SigHash;
use primitive_types::U256;

pub use legacy::TransactionLegacyZProvider;
pub use witness::TransactionWitnessP2pkhZProvider;

mod legacy;
mod witness;

pub trait ZProvider {
    fn z(&mut self, index: usize, sighash: SigHash, redeem_script: Option<Script>, witness_script: Option<Script>) -> Result<Hash256Value, Error>;
    
    fn z_u256(&mut self, index: usize, sighash: SigHash, redeem_script: Option<Script>, witness_script: Option<Script>) -> Result<U256, Error> {
        let z = self.z(index, sighash, redeem_script, witness_script)?;
        Ok(U256::from_big_endian(&z))
    }

    // for test
    fn z_without_replace_script(&self, index: usize, sighash: SigHash, redeem_script: Option<Script>, witness_script: Option<Script>) -> Result<Hash256Value, Error>;
}

// mock
pub struct ZProviderMocker(pub U256);

impl ZProvider for ZProviderMocker {
    fn z(&mut self, _: usize,  _: SigHash, _: Option<Script>, _: Option<Script>) -> Result<Hash256Value, Error> {
        let mut result = [0u8; 32];
        self.0.to_big_endian(&mut result);

        Ok(result)
    }

    fn z_without_replace_script(&self, _: usize, _: SigHash, _: Option<Script>, _: Option<Script>) -> Result<Hash256Value, Error> {
        let mut result = [0u8; 32];
        self.0.to_big_endian(&mut result);

        Ok(result)
    }
}
