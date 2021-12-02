use super::{S256Curve, S256FieldElementN, S256FieldElementNCreator, S256Point, Signature};
use primitive_types::U256;

pub struct PrivateKey {
    secret: S256FieldElementN,
    point: S256Point,
}

impl PrivateKey {
    pub fn new(secret: U256) -> Result<Self, &'static str> {
        let secret = S256FieldElementNCreator::from_u256(secret);
        let g = S256Point::g();
        let point = g * secret.num();
        if point.inner().field_point().is_none() {
            return Err("invalid key because of s*G=infinity");
        }

        Ok(Self { secret, point })
    }

    pub fn pk_point(&self) -> &S256Point {
        &self.point
    }

    // sig = (z + r*e)/k
    pub fn sign(&self, z: U256, k: U256) -> Result<Signature, &'static str> {
        let g = S256Point::g();
        let k = S256FieldElementNCreator::from_u256(k);
        let k_pk = g * k.num();
        let r = match k_pk.into_inner().into_field_point() {
            Some(p) => p.x(),
            None => return Err("failed to sign because of k*G=infinity"),
        };
        let r = S256FieldElementNCreator::from_u256(r.num());
        let z = S256FieldElementNCreator::from_u256(z);
        
        let signature = (z + r * self.secret) / k;
        if signature.num() > S256Curve::n()/2 {
            return Err("invalid signature because of the signature of BTC should less than N/2"); // the feature is not for secp256k1, just for Bitcoin
        }

        Ok(Signature::new(r.num(), signature.num()))
    }
}

#[cfg(test)]
mod tests {
    use super::PrivateKey;
    use super::super::S256Curve;
    use primitive_types::U256;

    #[test]
    fn create_priv_key_success() {
        let _ = PrivateKey::new(100u32.into()).unwrap();
    }

    #[test]
    #[should_panic]
    fn create_priv_key_failed() {
        let _ = PrivateKey::new(S256Curve::n()).unwrap();
    }
    
    #[test]
    fn priv_key_sign_success() {
        let priv_key = PrivateKey::new(100u32.into()).unwrap();

        let msg_hash = U256::from(200);
        let k = U256::from(800);
        let signature = priv_key.sign(msg_hash, k).unwrap();

        let result = signature.verify(msg_hash, priv_key.pk_point().clone());
        assert_eq!(result, true);
    }

    #[test]
    fn priv_key_sign_failed_1() {
        let priv_key = PrivateKey::new(100u32.into()).unwrap();

        let msg_hash = U256::from(200);
        let k = U256::from(S256Curve::n());
        let signature = priv_key.sign(msg_hash, k);
        assert_eq!(signature.err(), Some("failed to sign because of k*G=infinity"));
    }

    #[test]
    fn priv_key_sign_failed_2() {
        let priv_key = PrivateKey::new(100u32.into()).unwrap();

        let msg_hash = U256::from(200);
        let k = U256::from(300);
        let signature = priv_key.sign(msg_hash, k);
        assert_eq!(signature.err(), Some("invalid signature because of the signature of BTC should less than N/2"));
    }
}
