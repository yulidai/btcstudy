use crate::field::{FieldElement, FieldElementCreator};
use super::S256Curve;
use primitive_types::U256;

// creator

pub struct S256FieldElementPCreator;

impl S256FieldElementPCreator {
    pub fn from_u256(num: U256) -> S256FieldElementP {
        let prime = S256Curve::prime();
        let element = FieldElementCreator(prime).from_u256(num);
        S256FieldElementP(element)
    }

    pub fn from_i64(num: i64) -> S256FieldElementP {
        let prime = S256Curve::prime();
        let element = FieldElementCreator(prime).from_i64(num);
        S256FieldElementP(element)
    }
}

// element

pub struct S256FieldElementP(FieldElement);

impl S256FieldElementP {
    pub fn inner(&self) -> &FieldElement {
        &self.0
    }

    pub fn into_inner(self) -> FieldElement {
        self.0
    }
}
