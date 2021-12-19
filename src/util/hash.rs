use sha2::{Sha256, Digest as Sha256Digest};
use ripemd160::{Ripemd160, Digest as Rip160Digest};

// types

pub type Hash256Value = [u8; 32];
pub type Hash160Value = [u8; 20];

pub fn convert_slice_into_hash256(data: &[u8]) -> Hash256Value {
    let mut last = data.len();
    if last > 32 {
        last = 32;
    }
    let mut result = [0u8; 32];
    result.copy_from_slice(&data[..last]);

    result
}

pub fn convert_slice_into_hash160(data: &[u8]) -> Hash160Value {
    let mut last = data.len();
    if last > 20 {
        last = 20;
    }
    let mut result = [0u8; 20];
    result.copy_from_slice(&data[..last]);

    result
}

// helpers

pub fn sha256(bytes: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher.finalize().into()
}

pub fn ripemd160(bytes: &[u8]) -> [u8; 20] {
    let mut hasher = Ripemd160::new();
    hasher.update(bytes);
    hasher.finalize().into()
}

pub fn hash160(bytes: &[u8]) -> [u8; 20] {
    ripemd160(&sha256(bytes))
}

pub fn hash256(bytes: &[u8]) -> [u8; 32] {
    sha256(&sha256(bytes))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_hash160_1() {
        let pk = hex::decode("0250863ad64a87ae8a2fe83c1af1a8403cb53f53e486d8511dad8a04887e5b2352").unwrap();
        let hash = super::hash160(&pk);
        assert_eq!(hex::encode(hash), "f54a5851e9372b87810a8e60cdd2e7cfd80b6e31");
    }
}
