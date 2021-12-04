use primitive_types::U256;
use std::convert::TryFrom;

const BASE58_ALPHABET: &'static str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

pub fn encode(mut num: U256) -> String {
    let leading_zeros = usize::try_from(num.leading_zeros()).expect("failed to convert u32 into usize within base58") / 8;
    let base = U256::from(58);
    let base58_alphabet = BASE58_ALPHABET.as_bytes();

    let mut result_tmp = String::new();
    while !num.is_zero() {
        let (divisor, remainder) = num.div_mod(base);
        let index = remainder.as_usize();
        if index >= base58_alphabet.len() {
            panic!("invalid panic when encode into base58");
        }
        result_tmp.push(base58_alphabet[index].into());
        num = divisor;
    }
    let result_tmp = result_tmp.chars().rev().collect::<String>();

    let mut result = String::from("1").repeat(leading_zeros);
    result.push_str(result_tmp.as_str());

    result
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
    fn base58_encode_3() {
        let num = hex::decode("c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6").unwrap();
        let num = U256::from_big_endian(&num);
        assert_eq!(super::encode(num), "EQJsjkd6JaGwxrjEhfeqPenqHwrBmPQZjJGNSCHBkcF7");
    }
}