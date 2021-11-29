use primitive_types::{U256, U512};
use std::convert::TryFrom;
use crate::field::FieldElementCreator;
use crate::point::{FieldPoint, EccPoint};
use crate::prime::Prime;

pub fn a() -> U256 {
    U256::zero()
}

pub fn b() -> U256 {
    U256::from(7)
}

pub fn prime() -> Prime {
    let p = U512::from(2).pow(256.into()) - U512::from(2).pow(32.into()) - 977;
    println!("p: {:?}, {:x}", p, p);
    let p = U256::try_from(p).expect("invalid prime of secp256k1");
    Prime(p)
}

pub fn n() -> U256 {
    let big_endian = hex::decode("fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141").expect("invalid n of secp256k1");
    U256::from_big_endian(&big_endian)
}

pub fn g() -> EccPoint {
    let p = prime();
    let field_creator = FieldElementCreator(p);

    let gx = hex::decode("79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798").expect("invalid gx");
    let gx = p.create_element_from_u256(U256::from_big_endian(&gx));
    let gx = field_creator.from_u256(gx);

    let gy = hex::decode("483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8").expect("invalid gy");
    let gy = p.create_element_from_u256(U256::from_big_endian(&gy));
    let gy = field_creator.from_u256(gy);

    let field_point = FieldPoint{ x: gx, y: gy };

    let a = a();
    let a = field_creator.from_u256(a);
    let b = b();
    let b = field_creator.from_u256(b);
    EccPoint::new(Some(field_point), a, b).expect("invalid G of secp256k1")
}

#[cfg(test)]
mod tests {
    use crate::point::EccMulCoefficient;

    #[test]
    fn create_secp256k1_point() {
        let _ = super::g();
    }

    #[test]
    fn g_is_not_infinity() {
        let g = super::g();
        assert!(!g.is_infinity());
    }

    #[test]
    fn g_mul_n_is_infinity() {
        let g = super::g();
        let n = super::n();
        let n = EccMulCoefficient(n);

        let result = n * g;
        assert!(result.is_infinity());
    }
}
