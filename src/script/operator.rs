use crate::secp256k1::{S256Point, Signature};
use primitive_types::U256;

pub fn check_signature(pk: Vec<u8>, sig: Vec<u8>, z: U256) -> bool {
    let pk = match S256Point::parse(&pk) {
        Ok(pk) => pk,
        Err(e) => {
            println!("invalid public bytes: {}", e);
            return false;
        },
    };
    let (sig, _) = match Signature::parse_der(&sig) {
        Ok(sig) => sig,
        Err(e) => {
            println!("invalid signature bytes: {}", e);
            return false;
        }
    };
    
    sig.verify(z, pk)
}
