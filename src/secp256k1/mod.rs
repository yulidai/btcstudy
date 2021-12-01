use primitive_types::{U256, U512};
use std::convert::TryFrom;
use crate::field::{FieldElement, FieldElementCreator};
use crate::prime::Prime;

pub mod point;
pub mod signature;
pub mod private_key;

pub struct S256FieldCreator;

impl S256FieldCreator {
    pub fn from_u256(num: U256) -> FieldElement {
        let prime = prime();
        FieldElementCreator(prime).from_u256(num)
    }

    pub fn from_i64(num: i64) -> FieldElement {
        let prime = prime();
        FieldElementCreator(prime).from_i64(num)
    }
}

pub fn prime() -> Prime {
    let p = U512::from(2).pow(256.into()) - U512::from(2).pow(32.into()) - 977;
    let p = U256::try_from(p).expect("invalid prime of secp256k1");
    Prime(p)
}

pub struct S256NFieldCreator;

impl S256NFieldCreator {
    pub fn from_u256(num: U256) -> FieldElement {
        let prime = Self::n_prime();
        FieldElementCreator(prime).from_u256(num)
    }

    pub fn from_i64(num: i64) -> FieldElement {
        let prime = Self::n_prime();
        FieldElementCreator(prime).from_i64(num)
    }

    fn n_prime() -> Prime {
        let n = n();
        let n_prime = U256::try_from(n).expect("invalid n_prime of secp256k1");
        Prime(n_prime)
    }
}

pub fn n() -> U256 {
    let big_endian = hex::decode("fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141").expect("invalid n of secp256k1");
    U256::from_big_endian(&big_endian)
}
