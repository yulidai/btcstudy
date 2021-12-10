use crate::secp256k1::{S256Point, Signature};
use crate::util::varint;
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

pub fn decode_num(bytes: &[u8]) -> Result<u64, &'static str> {
    let (num, used) = varint::decode(bytes)?;
    if (used as usize) != bytes.len() {
        Err("invalid num because all of bytes must be used")
    } else {
        Ok(num)
    }
}