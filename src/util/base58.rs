use primitive_types::U256;
use crate::util;

const BASE256: usize = 256;
const BASE58: usize = 58;
const BASE58_ALPHABET: [u8; 58] = *b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

pub fn encode(num: U256) -> String {
    let mut bytes = [0u8; 32];
    num.to_big_endian(&mut bytes);

    encode_bytes(&bytes)
}

pub fn encode_bytes(bytes: &[u8]) -> String {
    let mut result = Vec::<u8>::new();

    // u8 to base58
    for byte in bytes {
        let mut carry = *byte as usize;
        // recalculate with new byte
        for byte in &mut result {
            carry += (*byte as usize) << 8;
            *byte = (carry % BASE58) as u8;
            carry = carry / BASE58;
        }
        // extend result.length
        while carry > 0 {
            result.push( (carry % BASE58) as u8);
            carry /= BASE58;
        }
    }
    // add leading zeros
    for byte in bytes {
        if *byte != 0u8 {
            break;
        }
        result.push(0);
    }
    // convert into char of alphabet
    for byte in &mut result {
        *byte = BASE58_ALPHABET[*byte as usize];
    }

    result.reverse();
    String::from_utf8(result).unwrap()
}

pub fn encode_bytes_checksum(bytes: &[u8]) -> String {
    let mut checksum = make_checksum(bytes);
    let mut bytes = bytes.to_vec();
    bytes.append(&mut checksum);

    encode_bytes(&bytes)
}

fn make_checksum(bytes: &[u8]) -> Vec<u8> {
    util::hash::hash256(bytes)[..4].to_vec()
}

// TODO move to wallet module in the future
pub fn decode_btc_addr(characters: &str) -> Result<Vec<u8>, &'static str> {
    let result = decode(characters)?;
    let len = result.len();
    if len < 5 {
        return Err("invalid str, at least 5 char for network and checksum");
    }
    let checksum_expect = &result[(len-4)..];
    let checksum_real = make_checksum(&result[..(len-4)]);
    if checksum_expect != checksum_real {
        return Err("Invalid checksum");
    }

    Ok(result)
}

pub fn decode(characters: &str) -> Result<Vec<u8>, &'static str> {
    let mut result = Vec::new();
    for character in characters.trim_start_matches("0").chars() {
        let mut carry = match BASE58_ALPHABET.iter().position(|&b| character == b as char) {
            None => return Err("invalid char of base58"),
            Some(p) => p,
        };
        for byte in &mut result {
            carry += *byte as usize * BASE58;
            *byte = (carry % BASE256) as u8;
            carry /=  BASE256;
        }
        while carry > 0 {
            result.push((carry % BASE256) as u8);
            carry /= BASE256;
        }
    }
    result.reverse();

    Ok(result)
}


#[cfg(test)]
mod tests {
    use primitive_types::U256;

    #[test]
    fn base58_encode_1() {
        let num = hex::decode("7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d").unwrap();
        let num = U256::from_big_endian(&num);
        assert_eq!(super::encode(num), "9MA8fRQrT4u8Zj8ZRd6MAiiyaxb2Y1CMpvVkHQu5hVM6");
    }

    #[test]
    fn base58_encode_2() {
        let num = hex::decode("eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c").unwrap();
        let num = U256::from_big_endian(&num);
        assert_eq!(super::encode(num), "14fE3H2E6XMp4SsxtwinF7w9a34ooUrwWe4WsW1458Pd"); // encode u256 so will have "1" as prefix
    }

    #[test]
    fn base58_encode_bytes_1() {
        let bytes = hex::decode("7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d").unwrap();
        let base58 = "9MA8fRQrT4u8Zj8ZRd6MAiiyaxb2Y1CMpvVkHQu5hVM6";

        assert_eq!(super::encode_bytes(&bytes), base58);
        assert_eq!(super::decode(base58).unwrap(), bytes);
    }

    #[test]
    fn base58_encode_bytes_2() {
        let bytes = hex::decode("eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c").unwrap();
        let base58 = "4fE3H2E6XMp4SsxtwinF7w9a34ooUrwWe4WsW1458Pd";

        assert_eq!(super::encode_bytes(&bytes), base58);
        assert_eq!(super::decode(base58).unwrap(), bytes);
    }

    #[test]
    fn base58_encode_bytes_3() {
        let bytes = hex::decode("c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6").unwrap();
        let base58 = "EQJsjkd6JaGwxrjEhfeqPenqHwrBmPQZjJGNSCHBkcF7";

        assert_eq!(super::encode_bytes(&bytes), base58);
        assert_eq!(super::decode(base58).unwrap(), bytes);
    }
}
