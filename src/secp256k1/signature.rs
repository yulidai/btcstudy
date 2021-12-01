use primitive_types::U256;
use crate::field::FieldElement;
use crate::secp256k1::S256NFieldCreator;
use crate::secp256k1::point::S256Point;

pub struct Signature {
    r: FieldElement,
    s: FieldElement,
}

impl Signature {
    pub fn new(r: U256, s: U256) -> Self {
        let r = S256NFieldCreator::from_u256(r);
        let s = S256NFieldCreator::from_u256(s);

        Self { r, s }
    }

    // sig = (z + r*e)/k
    pub fn verify(&self, z: U256, s256_pk_point: S256Point) -> bool {
        let z = S256NFieldCreator::from_u256(z);
        let g = S256Point::g();
        let cal_pk_point = g * (z/self.s).num() + s256_pk_point * (self.r/self.s).num();
        let cal_x = match cal_pk_point.into_inner().into_field_point() {
            None => return false,
            Some(p) => p.x,
        };

        cal_x.num() == self.r.num()
    }
}

#[cfg(test)]
mod tests {
    use crate::point::FieldPoint;
    use crate::secp256k1::{point::S256Point, S256FieldCreator};
    use primitive_types::U256;
    use super::Signature;

    #[test]
    fn signature_verify_success() {
        let r = hex::decode("37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6").unwrap();
        let s = hex::decode("8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec").unwrap();
        let r = U256::from_big_endian(&r);
        let s = U256::from_big_endian(&s);
        let signature = Signature::new(r, s);

        let z = hex::decode("bc62d4b80d9e36da29c16c5d4d9f11731f36052c72401a76c23c0fb5a9b74423").unwrap();
        let z = U256::from_big_endian(&z);

        let px = hex::decode("04519fac3d910ca7e7138f7013706f619fa8f033e6ec6e09370ea38cee6a7574").unwrap();
        let py = hex::decode("82b51eab8c27c66e26c858a079bcdf4f1ada34cec420cafc7eac1a42216fb6c4").unwrap();        
        let px = U256::from_big_endian(&px);
        let py = U256::from_big_endian(&py);
        let px = S256FieldCreator::from_u256(px);
        let py = S256FieldCreator::from_u256(py);
        let field_point = FieldPoint { x: px, y: py };
        let s256_pk_point = S256Point::new(field_point).unwrap();

        assert_eq!(signature.verify(z, s256_pk_point), true);
    }

    #[test]
    fn signature_verify_failed() {
        let r = hex::decode("37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6").unwrap();
        let s = hex::decode("8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaed").unwrap();
        let r = U256::from_big_endian(&r);
        let s = U256::from_big_endian(&s);
        let signature = Signature::new(r, s);

        let z = hex::decode("bc62d4b80d9e36da29c16c5d4d9f11731f36052c72401a76c23c0fb5a9b74423").unwrap();
        let z = U256::from_big_endian(&z);

        let px = hex::decode("04519fac3d910ca7e7138f7013706f619fa8f033e6ec6e09370ea38cee6a7574").unwrap();
        let py = hex::decode("82b51eab8c27c66e26c858a079bcdf4f1ada34cec420cafc7eac1a42216fb6c4").unwrap();        
        let px = U256::from_big_endian(&px);
        let py = U256::from_big_endian(&py);
        let px = S256FieldCreator::from_u256(px);
        let py = S256FieldCreator::from_u256(py);
        let field_point = FieldPoint { x: px, y: py };
        let s256_pk_point = S256Point::new(field_point).unwrap();

        assert_eq!(signature.verify(z, s256_pk_point), false);
    }
}