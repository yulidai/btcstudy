use crate::field::{FieldElement, FieldElementCreator};
use super::S256Curve;
use primitive_types::U256;
use std::ops::Sub;

// creator

pub struct S256FieldElementPCreator;

impl S256FieldElementPCreator {
    pub fn from_field_element(element: FieldElement) -> Result<S256FieldElementP, &'static str> {
        if element.prime() != S256Curve::prime() {
            return Err("invalid prime of field_element for S256FieldElement");
        }
        Ok( S256FieldElementP(element) )
    }

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

    // only for (prime % 4 == 3): the prime of S256K1 is match
    // a.sqrt() = a ^ ((prime+1) / 4)
    pub fn sqrt(&self) -> Self {
        let exponent = (self.0.prime().0 + 1) / 4;
        let field_element = self.0.pow_u256(exponent);

        Self(field_element)
    }
}

impl Sub for S256FieldElementP {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let element = self.into_inner() - other.into_inner();
        Self(element)
    }
}
