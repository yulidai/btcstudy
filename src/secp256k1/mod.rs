use primitive_types::{U256, U512};
use std::convert::TryFrom;
use crate::field::Prime;

pub use element_n::{S256FieldElementN, S256FieldElementNCreator};
pub use element_p::{S256FieldElementP, S256FieldElementPCreator};
pub use point::S256Point;
pub use private_key::PrivateKey;
pub use signature::Signature;

mod element_n;
mod element_p;
mod point;
mod signature;
mod private_key;

pub struct S256Curve;

impl S256Curve {
    pub fn a() -> U256 {
        U256::zero()
    }

    pub fn b() -> U256 {
        U256::from(7)
    }

    pub fn n() -> U256 {
        let big_endian = hex::decode("fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141").expect("invalid n of secp256k1");
        U256::from_big_endian(&big_endian)
    }

    pub fn prime() -> Prime {
        let p = U512::from(2).pow(256.into()) - U512::from(2).pow(32.into()) - 977;
        let p = U256::try_from(p).expect("invalid prime of secp256k1");
        Prime(p)
    }
}
