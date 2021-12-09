use primitive_types::U256;
use crate::secp256k1::point::S256Point;
use super::{S256FieldElementN, S256FieldElementNCreator};
use crate::util::math;

#[derive(Debug, PartialEq)]
pub struct Signature {
    r: S256FieldElementN,
    s: S256FieldElementN,
}

impl Signature {
    pub fn new(r: U256, s: U256) -> Self {
        let r = S256FieldElementNCreator::from_u256(r);
        let s = S256FieldElementNCreator::from_u256(s);

        Self { r, s }
    }

    // sig = (z + r*e)/k
    pub fn verify(&self, z: U256, s256_pk_point: S256Point) -> bool {
        let z = S256FieldElementNCreator::from_u256(z);
        let g = S256Point::g();
        let cal_pk_point = g * (z/self.s).num() + s256_pk_point * (self.r/self.s).num();
        let cal_x = match cal_pk_point.into_inner().into_field_point() {
            None => return false,
            Some(p) => p.x(),
        };

        cal_x.num() == self.r.num()
    }

    pub fn der(&self) -> String {
        let r_der = Self::der_parse_u256(&self.r.num());
        let s_der = Self::der_parse_u256(&self.s.num());
        let sig = format!("02{:02x}{}02{:02x}{}", r_der.len()/2, r_der, s_der.len()/2, s_der);

        format!("30{:x}{}", sig.len()/2, sig)
    }

    pub fn der_parse_u256(num: &U256) -> String {
        let mut prefix = "";
        if num.byte(31) >= 0x80 { // small order
            prefix = "00";
        }
        format!("{}{:x}", prefix, num)
    }

    // @return (Self, bytes_used)
    pub fn parse_der(bytes: &[u8]) -> Result<(Self, usize), String> {
        if bytes.is_empty() {
            return Err("cannot convert empty bytes into Signature in Signature::parse_der".into());
        }
        if bytes[0] != 0x30 {
            return Err("bytes[0] != 0x30 for Signature::parse_der".into());
        }
        let element_len = bytes[1] as usize;
        if element_len + 2 > bytes.len() {
            return Err("bytes.len() is too short in Signature::parse_der".into());
        }
        let (r, byte_used_1) = Self::parse_der_element(&bytes[2..])?;
        let (s, byte_used_2) = Self::parse_der_element(&bytes[(2+byte_used_1)..])?;
        if element_len != byte_used_1 + byte_used_2 {
            return Err("invalid len in der[1]".into());
        }

        Ok( (Self::new(r, s), element_len + 2) )
    }

    // @return (U256, byte_used)
    fn parse_der_element(bytes: &[u8]) -> Result<(U256, usize), String> {
        if bytes.is_empty() {
            return Err("cannot convert empty bytes into U256 in Signature::parse_element".into());
        }

        let mut i = 0;
        let len = bytes.len();
        if bytes[i] != 0x02 {
            return Err("invalid mark byte in Signature::parse_element".into());
        }

        i = math::check_range_add(i, 1, len)?;
        let num_bytes = bytes[i];
        let result = match num_bytes {
            0x21 => {
                i = math::check_range_add(i, 1, len)?;
                if bytes[i] != 0x00 {
                    return Err("invalid prefix-00 in Signature::parse_element".into());
                }
                let fst = math::check_range_add(i, 1, len)?; // skip 0x00 byte
                let lst = math::check_range_add(i, 0x20, len)?;
                (U256::from_big_endian(&bytes[fst..(lst+1)]), 0x23)
            },
            0x20 => {
                let fst = math::check_range_add(i, 1, len)?;
                let lst = math::check_range_add(i, 0x20, len)?;
                (U256::from_big_endian(&bytes[fst..(lst+1)]), 0x22)
            },
            _ => return Err("invalid len of bytes in Signature::parse_element".into()),
        };

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::secp256k1::{point::S256Point, S256FieldElementPCreator};
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
        let px = S256FieldElementPCreator::from_u256(px);
        let py = S256FieldElementPCreator::from_u256(py);
        let s256_pk_point = S256Point::from_s256_field_element(px, py).unwrap();

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
        let px = S256FieldElementPCreator::from_u256(px);
        let py = S256FieldElementPCreator::from_u256(py);
        let s256_pk_point = S256Point::from_s256_field_element(px, py).unwrap();

        assert_eq!(signature.verify(z, s256_pk_point), false);
    }

    #[test]
    fn signature_der() {
        let r = hex::decode("37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6").unwrap();
        let s = hex::decode("8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec").unwrap();
        let r = U256::from_big_endian(&r);
        let s = U256::from_big_endian(&s);
        let signature = Signature::new(r, s);

        let der = signature.der();
        assert_eq!(der, "3045022037206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c60221008ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec");
    }

    #[test]
    fn signature_parse_der() {
        let r = hex::decode("37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6").unwrap();
        let s = hex::decode("8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec").unwrap();
        let r = U256::from_big_endian(&r);
        let s = U256::from_big_endian(&s);
        let signature = Signature::new(r, s);

        let der = signature.der();
        let der = hex::decode(der).unwrap();
        let (signature_2, bytes_used) = Signature::parse_der(&der).unwrap();
        assert_eq!(signature, signature_2);
        assert_eq!(bytes_used, 71);
    }
}