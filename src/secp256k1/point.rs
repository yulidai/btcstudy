use crate::field_ecc::{FieldEccPoint, FieldEccPointCreator, FieldPoint, FieldPointCreator};
use primitive_types::U256;
use std::ops::{Add, Mul};
use super::{S256Curve, S256FieldElementP, S256FieldElementPCreator};

#[derive(Clone)]
pub struct S256Point(FieldEccPoint);

impl S256Point {
    pub fn from_s256_field_element(x: S256FieldElementP, y: S256FieldElementP) -> Result<Self, String> {
        let field_point = FieldPointCreator::from_field_element(x.into_inner(), y.into_inner()).expect("prime of s256_field_element is different");
        Self::from_field_point(field_point)
    }

    pub fn from_field_point(point: FieldPoint) -> Result<Self, String> {
        let p = S256Curve::prime();
        let field_ecc_point_creator = FieldEccPointCreator::new(p, S256Curve::a(), S256Curve::b());
        let ecc_point = field_ecc_point_creator.with_field_point(point)?;

        let n = S256Curve::n();
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

    pub fn g() -> Self {
        let gx = hex::decode("79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798").expect("invalid gx");
        let gy = hex::decode("483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8").expect("invalid gy");

        let gx = S256FieldElementPCreator::from_u256(U256::from_big_endian(&gx));
        let gy = S256FieldElementPCreator::from_u256(U256::from_big_endian(&gy));
        let field_point = FieldPointCreator::from_field_element(gx.into_inner(), gy.into_inner()).unwrap();

        Self::from_field_point(field_point).expect("invalid G of secp256k1")
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
        let n = S256Curve::n();
        let coefficient = coefficient % n;
        let ecc_point_result = self.0 * coefficient;
        
        Self(ecc_point_result)
    }
}

#[cfg(test)]
mod tests {
    use super::S256Point;
    use super::super::S256Curve;

    #[test]
    fn g_is_not_infinity() {
        let g = S256Point::g();
        assert!(!g.inner().is_infinity());
    }

    #[test]
    fn g_mul_n_is_infinity() {
        let g = S256Point::g();
        let n = S256Curve::n();

        let result = g * n;
        assert!(result.inner().is_infinity());
    }
}

