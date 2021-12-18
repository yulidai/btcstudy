use crate::util::hash::Hash256Value;
use super::SigHash;
use primitive_types::U256;

pub trait ZProvider {
    fn z(&self, sighash: SigHash) -> Hash256Value;
    
    fn z_u256(&self, sighash: SigHash) -> U256 {
        let z = self.z(sighash);
        U256::from_big_endian(&z)
    }
}

// mock
pub struct ZProviderMocker(pub U256);

impl ZProvider for ZProviderMocker {
    fn z(&self, _: SigHash) -> Hash256Value {
        let mut result = [0u8; 32];
        self.0.to_big_endian(&mut result);

        result
    }
}
