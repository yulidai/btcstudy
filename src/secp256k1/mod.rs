use primitive_types::{U256, U512};
use std::convert::TryFrom;
use crate::field::{FieldElement, FieldElementCreator};
use crate::prime::Prime;

pub mod point;

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
