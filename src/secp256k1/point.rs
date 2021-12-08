use crate::field_ecc::{FieldEccPoint, FieldEccPointCreator, FieldPoint, FieldPointCreator};
use crate::util::{base58, hash};
use primitive_types::U256;
use std::ops::{Add, Mul};
use super::{S256Curve, S256FieldElementP, S256FieldElementPCreator};

#[derive(Debug, Clone, PartialEq)]
pub struct S256Point(FieldEccPoint);

impl S256Point {
    pub fn parse(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() < 33 {
            return Err("invalid sec".into());
        }
        let x = U256::from_big_endian(&bytes[1..33]);
        let x = S256FieldElementPCreator::from_u256(x);

        if bytes.len() == 33 {
            let alpha = x.inner().pow_i64(3) + S256FieldElementPCreator::from_u256(S256Curve::b()).into_inner();
            let alpha = S256FieldElementPCreator::from_field_element(alpha)?;
            let y = alpha.sqrt();
            let y = match bytes[0] {
                2 => if y.inner().num() % 2 == U256::zero() { y } else { S256FieldElementPCreator::from_u256(S256Curve::n()) - y },
                3 => if y.inner().num() % 2 == U256::one() { y } else { S256FieldElementPCreator::from_u256(S256Curve::n()) - y },
                _ => return Err("invalid bytes[0], must as 2, 3 or 4".into()),
            };

            return Self::from_s256_field_element(x, y);
        }

        if bytes[0] == 4 && bytes.len() == 65 {
            let y = U256::from_big_endian(&bytes[33..65]);
            let y = S256FieldElementPCreator::from_u256(y);

            return Self::from_s256_field_element(x, y);
        }

        return Err("invalid sec".into());
    }

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

    pub fn sec_uncompressed(&self) -> Option<Vec<u8>> {
        match self.0.field_point() {
            Some(point) => {
                let mut x_bytes = [0u8; 32];
                let mut y_bytes = [0u8; 32];
                point.x().num().to_big_endian(&mut x_bytes);
                point.y().num().to_big_endian(&mut y_bytes);

                let mut result = vec![4u8];
                result.append(&mut x_bytes.to_vec());
                result.append(&mut y_bytes.to_vec());

                Some(result)
            },
            None => None,
        }
    }

    pub fn sec_compressed(&self) -> Option<Vec<u8>> {
        match self.0.field_point() {
            None => None,
            Some(point) => {
                let mut x_bytes = [0u8; 32];
                point.x().num().to_big_endian(&mut x_bytes);

                let mut result = if (point.x().num() % 2).is_zero() { vec![2u8] } else { vec![3u8] };
                result.append(&mut x_bytes.to_vec());

                Some(result)
            }
        }
    }

    pub fn hash160(&self, compressed: bool) -> Vec<u8> {
        let sec_bytes = match compressed {
            true => self.sec_compressed(),
            false => self.sec_uncompressed(),
        }.unwrap();
        let result = hash::hash160(&sec_bytes);

        result
    }

    pub fn address(&self, compressed: bool, test_net: bool) -> String {
        let mut h160 = self.hash160(compressed);
        let prefix = if test_net { 0x6fu8 } else { 0x00u8 };

        let mut bytes = [prefix].to_vec();
        bytes.append(&mut h160);

        base58::ecode_bytes_checksum(&bytes)
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
    use super::super::{S256Curve, PrivateKey};
    use primitive_types::U256;

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

    #[test]
    fn s256_point_sec_uncompressed() {
        let g_sec = S256Point::g().sec_uncompressed().unwrap();
        assert_eq!(hex::encode(g_sec), "0479be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8");
    }

    #[test]
    fn s256_point_sec_compressed_1() {
        let g_sec = S256Point::g().sec_compressed().unwrap();
        assert_eq!(hex::encode(g_sec), "0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798");
    }

    #[test]
    fn s256_point_sec_compressed_2() {
        let g_sec = (S256Point::g() * 2.into()).sec_compressed().unwrap();
        assert_eq!(hex::encode(g_sec), "03c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5");
    }

    #[test]
    fn s256_point_parse_sec_uncompressed() {
        let g = S256Point::g();
        let g_sec = g.sec_uncompressed().unwrap();
        let g_parsed = S256Point::parse(&g_sec).unwrap();
        assert_eq!(g, g_parsed);
    }

    #[test]
    fn s256_point_parse_sec_compressed() {
        let g = S256Point::g();
        let g_sec = g.sec_compressed().unwrap();

        let g_parsed = S256Point::parse(&g_sec).unwrap();
        assert_eq!(g, g_parsed);
    }

    #[test]
    fn s256_point_parse_1() {
        let sk = PrivateKey::new(5001.into()).unwrap();
        let pk = sk.pk_point();
        let pk_sec = pk.sec_compressed().unwrap();
        assert_eq!(hex::encode(pk_sec), "0357a4f368868a8a6d572991e484e664810ff14c05c0fa023275251151fe0e53d1");
    }

    #[test]
    fn s256_point_address_1() {
        let sk = PrivateKey::new(5002.into()).unwrap();
        let pk = sk.pk_point();
        let address = pk.address(false, true);
        assert_eq!(address, "mmTPbXQFxboEtNRkwfh6K51jvdtHLxGeMA");
    }

    #[test]
    fn s256_point_address_2() {
        let sk = U256::from(2020);
        let sk = sk.pow(5.into());
        let sk = PrivateKey::new(sk).unwrap();
        let pk = sk.pk_point();
        let address = pk.address(true, true);
        assert_eq!(address, "mopVkxp8UhXqRYbCYJsbeE1h1fiF64jcoH");
    }

    #[test]
    fn s256_point_address_3() {
        let sk = U256::from_str_radix("12345deadbeef", 16).unwrap();
        let sk = PrivateKey::new(sk).unwrap();
        let pk = sk.pk_point();
        let address = pk.address(true, false);
        assert_eq!(address, "1F1Pn2y6pDb68E5nYJJeba4TLg2U7B6KF1");
    }
}
