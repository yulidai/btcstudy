use crate::field::{Prime, FieldElement, FieldElementCreator};
use primitive_types::U256;

pub struct FieldPointCreator(FieldElementCreator);

impl FieldPointCreator {
    pub fn new(prime: Prime) -> Self {
        Self(FieldElementCreator(prime))
    }

    pub fn from_u256(&self, x: U256, y: U256) -> FieldPoint {
        let x = self.0.from_u256(x);
        let y = self.0.from_u256(y);
        FieldPoint { x, y }
    }

    pub fn from_i64(&self, x: i64, y: i64) -> FieldPoint {
        let x = self.0.from_i64(x);
        let y = self.0.from_i64(y);
        FieldPoint { x, y }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FieldPoint {
    pub x: FieldElement,
    pub y: FieldElement,
}
