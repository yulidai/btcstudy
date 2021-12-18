use crate::util::hash::Hash256Value;
use super::{SigHash, Error};
use primitive_types::U256;

pub trait ZProvider {
    fn z(&self, sighash: SigHash) -> Result<Hash256Value, Error>;
    
    fn z_u256(&self, sighash: SigHash) -> Result<U256, Error> {
        let z = self.z(sighash)?;
        Ok(U256::from_big_endian(&z))
    }
}

// mock
pub struct ZProviderMocker(pub U256);

impl ZProvider for ZProviderMocker {
    fn z(&self, _: SigHash) -> Result<Hash256Value, Error> {
        let mut result = [0u8; 32];
        self.0.to_big_endian(&mut result);

        Ok(result)
    }
}
