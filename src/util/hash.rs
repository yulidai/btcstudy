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

#[cfg(test)]
mod tests {
    #[test]
    fn test_hash160_1() {
        let pk = hex::decode("0250863ad64a87ae8a2fe83c1af1a8403cb53f53e486d8511dad8a04887e5b2352").unwrap();
        let hash = super::hash160(&pk);
        assert_eq!(hex::encode(hash), "f54a5851e9372b87810a8e60cdd2e7cfd80b6e31");
    }
}
