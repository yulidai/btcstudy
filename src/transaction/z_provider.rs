use crate::util::hash::Hash256Value;
use super::{SigHash, Error};
use primitive_types::U256;

pub trait ZProvider {
    fn z(&self, index: usize, sighash: SigHash) -> Result<Hash256Value, Error>;
    
    fn z_u256(&self, index: usize, sighash: SigHash) -> Result<U256, Error> {
        let z = self.z(index, sighash)?;
        Ok(U256::from_big_endian(&z))
    }

    // for test
    fn z_without_replace_script(&self, index: usize, sighash: SigHash) -> Result<Hash256Value, Error>;
}

// mock
pub struct ZProviderMocker(pub U256);

impl ZProvider for ZProviderMocker {
    fn z(&self, _: usize, _: SigHash) -> Result<Hash256Value, Error> {
        let mut result = [0u8; 32];
        self.0.to_big_endian(&mut result);

        Ok(result)
    }

    fn z_without_replace_script(&self, index: usize, sighash: SigHash) -> Result<Hash256Value, Error> {
        self.z(index, sighash)
    }
}
