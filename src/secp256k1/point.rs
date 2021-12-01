use crate::field::FieldElementCreator;
use crate::field_ecc::{FieldEccPoint, FieldPoint};
use primitive_types::U256;
use std::ops::{Add, Mul};

#[derive(Clone)]
pub struct S256Point(FieldEccPoint);

impl S256Point {
    pub fn new(point: FieldPoint) -> Result<Self, String> {
        let p = super::prime();
        let field_creator = FieldElementCreator(p);

        let a = field_creator.from_u256(Self::a());
        let b = field_creator.from_u256(Self::b());
        let ecc_point = FieldEccPoint::new(Some(point), a, b)?;

        let n = super::n();
        if !( ecc_point.clone() * n ).is_infinity() {
            return Err(format!("invalid n({:x}) for ecc_point({:?})", n, ecc_point));
        }

        Ok(Self(ecc_point))
    }

    pub fn into_inner(self) -> FieldEccPoint {
        self.0
    }

    pub fn inner(&self) -> &FieldEccPoint {
        &self.0
    }

    pub fn a() -> U256 {
        U256::zero()
    }
    
    pub fn b() -> U256 {
        U256::from(7)
    }

    pub fn g() -> Self {
        let p = super::prime();
        let field_creator = FieldElementCreator(p);

        let gx = hex::decode("79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798").expect("invalid gx");
        let gx = p.create_element_from_u256(U256::from_big_endian(&gx));
        let gx = field_creator.from_u256(gx);

        let gy = hex::decode("483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8").expect("invalid gy");
        let gy = p.create_element_from_u256(U256::from_big_endian(&gy));
        let gy = field_creator.from_u256(gy);

        let field_point = FieldPoint{ x: gx, y: gy };

        let a = Self::a();
        let a = field_creator.from_u256(a);
        let b = Self::b();
        let b = field_creator.from_u256(b);
        let ecc_point = FieldEccPoint::new(Some(field_point), a, b).expect("invalid G of secp256k1");

        Self(ecc_point)
    }
}

impl Add<Self> for S256Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let ecc_point = self.inner().clone() + rhs.inner().clone();
        Self(ecc_point)
    }
}
    

impl Mul<U256> for S256Point {
    type Output = Self;

    fn mul(self, coefficient: U256) -> Self {
        let n = super::n();
        let coefficient = coefficient % n;
        let ecc_point_result = self.0 * coefficient;
        
        Self(ecc_point_result)
    }
}

#[cfg(test)]
mod tests {
    use super::S256Point;

    #[test]
    fn g_is_not_infinity() {
        let g = S256Point::g();
        assert!(!g.inner().is_infinity());
    }

    #[test]
    fn g_mul_n_is_infinity() {
        let g = S256Point::g();
        let n = super::super::n();

        let result = g * n;
        assert!(result.inner().is_infinity());
    }
}

