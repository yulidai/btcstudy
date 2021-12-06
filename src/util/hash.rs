use sha2::{Sha256};
use ripemd160::{Ripemd160, Digest};

pub fn sha256(bytes: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher.finalize().to_vec()
}

pub fn ripemd160(bytes: &[u8]) -> Vec<u8> {
    let mut hasher = Ripemd160::new();
    hasher.update(bytes);
    hasher.finalize().to_vec()
}

pub fn hash160(bytes: &[u8]) -> Vec<u8> {
    ripemd160(&sha256(bytes))
}

pub fn hash256(bytes: &[u8]) -> Vec<u8> {
    sha256(&sha256(bytes))
}
