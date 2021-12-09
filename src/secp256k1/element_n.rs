use crate::field::{FieldElement, FieldElementCreator, Prime};
use super::S256Curve;
use primitive_types::U256;
use std::ops::{Add, Sub, Mul, Div};

// creator

pub struct S256FieldElementNCreator;

impl S256FieldElementNCreator {
    pub fn from_u256(num: U256) -> S256FieldElementN {
        let prime = Prime(S256Curve::n());
        let element = FieldElementCreator(prime).from_u256(num);
        S256FieldElementN(element)
    }

    pub fn from_i64(num: i64) -> S256FieldElementN {
        let prime = Prime(S256Curve::n());
        let element = FieldElementCreator(prime).from_i64(num);
        S256FieldElementN(element)
    }
}

// element
#[derive(Copy, Clone, Debug)]
pub struct S256FieldElementN(FieldElement);

impl S256FieldElementN {
    pub fn num(&self) -> U256 {
        self.0.num()
    }

    pub fn inner(&self) -> &FieldElement {
        &self.0
    }

    pub fn into_inner(self) -> FieldElement {
        self.0
    }
}

impl PartialEq for S256FieldElementN {
    fn eq(&self, other: &Self) -> bool {
        self.inner() == other.inner()
    }

    fn ne(&self, other: &Self) -> bool {
        !(self == other)
    }
}

impl Add for S256FieldElementN {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let element = self.into_inner() + other.into_inner();
        Self(element)
    }
}

impl Sub for S256FieldElementN {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let element = self.into_inner() - other.into_inner();
        Self(element)
    }
}

impl Mul for S256FieldElementN {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let element = self.into_inner() * other.into_inner();
        Self(element)
    }
}

impl Div for S256FieldElementN {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let element = self.into_inner() / other.into_inner();
        Self(element)
    }
}
