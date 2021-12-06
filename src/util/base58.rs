use primitive_types::U256;
use crate::util;

const BASE: usize = 58;
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
            *byte = (carry % BASE) as u8;
            carry = carry / BASE;
        }
        // extend result.length
        while carry > 0 {
            result.push( (carry % BASE) as u8);
            carry /= BASE;
        }
    }
    // add leading zeros
    for byte in bytes {
        if *byte == 0u8 {
            result.push(0);
        }
    }
    // convert into char of alphabet
    for byte in &mut result {
        *byte = BASE58_ALPHABET[*byte as usize];
    }

    result.reverse();
    String::from_utf8(result).unwrap()
}

pub fn ecode_bytes_checksum(bytes: &[u8]) -> String {
    let mut checksum = util::hash::hash256(bytes)[..4].to_vec();
    let mut bytes = bytes.to_vec();
    bytes.append(&mut checksum);

    encode_bytes(&bytes)
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
        let result = super::encode_bytes(&bytes);
        assert_eq!(result, "9MA8fRQrT4u8Zj8ZRd6MAiiyaxb2Y1CMpvVkHQu5hVM6");
    }

    #[test]
    fn base58_encode_bytes_2() {
        let bytes = hex::decode("eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c").unwrap();
        let result = super::encode_bytes(&bytes);
        assert_eq!(result, "4fE3H2E6XMp4SsxtwinF7w9a34ooUrwWe4WsW1458Pd");
    }

    #[test]
    fn base58_encode_bytes_3() {
        let bytes = hex::decode("c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6").unwrap();
        let result = super::encode_bytes(&bytes);
        assert_eq!(result, "EQJsjkd6JaGwxrjEhfeqPenqHwrBmPQZjJGNSCHBkcF7");
    }
}