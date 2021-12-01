use primitive_types::U256;
use std::convert::TryFrom;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Prime(pub U256);

impl Prime {
    pub fn new(prime: U256) -> Self {
        Self(prime)
    }

    pub fn create_element_from_u256(&self, num: U256) -> U256 {
        num % self.0
    }

    pub fn create_element_from_i64(&self, num: i64) -> U256 {
        Self::i64_mod_u256(num, self.0)
    }

    // a^(p-1)%p=1, so exponent in prime(13): 9, -3=-3+12=9, -15=-15+12*2=9
    pub fn create_exponent_from_u256(&self, exponent: U256) -> U256 {
        exponent % (self.0 - 1)
    }

    pub fn create_exponent_from_i64(&self, exponent: i64) -> U256 {
        Self::i64_mod_u256(exponent, self.0-1)
    }

    fn i64_mod_u256(num: i64, denominator: U256) -> U256 {
        if num >= 0 {
            let num = u64::try_from(num).expect("failed to convert from i64(>0) into u64");
            return U256::from(num) % denominator
        }

        // num<0 && denominator>u64::MAX
        if denominator > U256::from(u64::MAX) {
            let num_abs = i128::from(num).abs(); // panic if num==i64::MIN when abs(), so convert into i128 before abs()
            let num_abs = u64::try_from(num_abs).expect("failed to convert i64.abs() into u64 when i64 is negative");
            return denominator - num_abs;
        }

        // num<0 && prime<=u64::MAX
        let denominator = i128::from(denominator.as_u64());
        let num = i128::from(num).rem_euclid(denominator);
        let num = u64::try_from(num).expect("failed to execute i64%u64");
        num.into()
    }
}

#[cfg(test)]
mod tests {
    use primitive_types::U256;
    use super::Prime;
    use std::convert::TryFrom;

    #[test]
    pub fn i64_min_to_u64() {
        let bf = i64::MIN as i128;
        let af = u64::try_from(bf.abs());

        println!("bf: {:?}, af: {:?}", bf, af);
    }

    #[test]
    pub fn i64_to_u32() {
        println!("{}, {}", 0i64.is_positive(), 0i64.is_negative());

        let bf = 100i64;
        let af = u32::try_from(bf);

        println!("bf: {:?}, af: {:?}", bf, af);
    }

    #[test]
    fn create_element_from_u256_0() {
        let prime13 = Prime(U256::from(13u32));
        assert_eq!(prime13.create_element_from_u256(0u32.into()), 0.into());
    }

    #[test]
    fn create_element_from_u256_1() {
        let prime13 = Prime(U256::from(13u32));
        assert_eq!(prime13.create_element_from_u256(10u32.into()), 10.into());
    }

    #[test]
    fn create_element_from_u256_2() {
        let prime13 = Prime(U256::from(13u32));
        assert_eq!(prime13.create_element_from_u256(23u32.into()), 10.into());
    }

    #[test]
    fn create_element_from_i64_1() {
        let prime13 = Prime(U256::from(13u32));
        assert_eq!(prime13.create_element_from_i64(-3), 10.into());
    }

    #[test]
    fn create_element_from_i64_2() {
        let prime13 = Prime(U256::from(13u32));
        assert_eq!(prime13.create_element_from_i64(-16), 10.into());
    }

    #[test]
    fn create_element_from_i64_3() {
        let prime13 = Prime(U256::from(13u32));
        assert_eq!(prime13.create_element_from_i64(10), 10.into());
    }

    #[test]
    fn create_element_from_i64_4() {
        let prime13 = Prime(U256::from(13u32));
        assert_eq!(prime13.create_element_from_i64(23), 10.into());
    }

    #[test]
    fn create_exponent_from_u256_1() {
        let prime13 = Prime(U256::from(13u32));
        assert_eq!(prime13.create_exponent_from_u256(9.into()), 9.into());
    }

    #[test]
    fn create_exponent_from_u256_2() {
        let prime13 = Prime(U256::from(13u32));
        assert_eq!(prime13.create_exponent_from_u256(21.into()), 9.into());
    }

    #[test]
    fn prime_create_exponent_from_i64_1() {
        let prime13 = Prime(U256::from(13u32));
        assert_eq!(prime13.create_exponent_from_i64(-3), 9.into());
    }

    #[test]
    fn prime_create_exponent_from_i64_2() {
        let prime13 = Prime(U256::from(13u32));
        assert_eq!(prime13.create_exponent_from_i64(-15), 9.into());
    }

    #[test]
    fn prime_create_exponent_from_i64_3() {
        let prime13 = Prime(U256::from(13u32));
        assert_eq!(prime13.create_exponent_from_i64(9), 9.into());
    }

    #[test]
    fn prime_create_exponent_from_i64_4() {
        let prime13 = Prime(U256::from(13u32));
        assert_eq!(prime13.create_exponent_from_i64(21), 9.into());
    }

    #[test]
    fn prime_i64_mod_u256_0() {
        assert_eq!(Prime::i64_mod_u256(23, 13.into()), 10.into());
    }

    #[test]
    fn prime_i64_mod_u256_1() {
        assert_eq!(Prime::i64_mod_u256(3, 13.into()), 3.into());
    }

    #[test]
    fn prime_i64_mod_u256_2() {
        assert_eq!(Prime::i64_mod_u256(0, 13.into()), 0.into());
    }

    #[test]
    fn prime_i64_mod_u256_3() {
        assert_eq!(Prime::i64_mod_u256(-3, 13.into()), 10.into());
    }

    #[test]
    fn prime_i64_mod_u256_4() {
        assert_eq!(Prime::i64_mod_u256(-23, 13.into()), 3.into());
    }

    #[test]
    fn prime_i64_mod_u256_5() {
        let denominator = (u64::MAX as u128) + 1;
        assert_eq!(Prime::i64_mod_u256(i64::MIN, denominator.into()), 9223372036854775808u128.into());
    }

    #[test]
    fn prime_i64_mod_u256_6() {
        let denominator = (u64::MAX as u128) + 1;
        assert_eq!(Prime::i64_mod_u256(i64::MAX, denominator.into()), u64::try_from(i64::MAX).unwrap().into());
    }
}
