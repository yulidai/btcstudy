use std::cmp::PartialEq;
use std::convert::TryFrom;
use std::ops::{Add, Sub, Mul, Div};
use std::fmt::{self, Display};
use super::prime::Prime;

pub struct FieldElementCreator(pub Prime);

impl FieldElementCreator {
    pub fn from_u32(&self, num: u32) -> FieldElement {
        let prime = self.0;
        let num = prime.create_element_from_u32(num);
        FieldElement { num, prime }
    }

    pub fn from_i64(&self, num: i64) -> FieldElement {
        let prime = self.0;
        let num = prime.create_element_from_i64(num);
        FieldElement { num, prime }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct FieldElement {
    num: u32,
    prime: Prime,
}

impl FieldElement {
    pub fn is_zero(&self) -> bool {
        self.num == 0
    }

    pub fn prime(&self) -> Prime {
        self.prime
    }

    pub fn pow_u32(&self, exponent: u32) -> Self {
        let exponent = self.prime.create_exponent_from_u32(exponent);
        self.pow(exponent)
    }

    pub fn pow_i64(&self, exponent: i64) -> Self {
        let exponent = self.prime.create_exponent_from_i64(exponent);
        self.pow(exponent)
    }

    fn pow(&self, mut exponent: u32) -> Self {
        let prime = self.prime.0;

        // pow(num, exponent, prime)
        let mut current = self.num;
        let mut result = 1;
        while exponent > 0 {
            if exponent % 2 == 1 {
                let result_u64 = (result as u64 * current as u64) % (prime as u64);
                result = u32::try_from(result_u64).expect("overflow when pow FieldElement");
            }
            let current_u64 = (current as u64 * current as u64) % (prime as u64);
            current = u32::try_from(current_u64).expect("overflow when pow FieldElement");
            exponent = exponent >> 1;
        }

        Self {
            num: result,
            prime: self.prime,
        }
    }
}

impl Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.num)
    }
}

impl PartialEq for FieldElement {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.prime == other.prime
    }

    fn ne(&self, other: &Self) -> bool {
        !(self == other)
    }
}

impl Add for FieldElement {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("cannot add two numbers in different Fields");
        }
        let num = (self.num as u64 + other.num as u64) % (self.prime.0 as u64);
        Self {
            num: u32::try_from(num).expect("overflow when add FieldElement"),
            prime: self.prime
        }
    }
}

impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("cannot sub two numbers in different Fields");
        }
        let num = ((self.num as i64) - (other.num as i64)).rem_euclid(self.prime.0 as i64) as u32;
        Self {
            num: u32::try_from(num).expect("overflow when sub FieldElement"),
            prime: self.prime
        }
    }
}

impl Mul for FieldElement {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("cannot mul two numbers in different Fields");
        }
        let num = (self.num as u64) * (other.num as u64) % (self.prime.0 as u64);
        Self {
            num: u32::try_from(num).expect("overflow when mul FieldElement"),
            prime: self.prime
        }
    }
}

impl Div for FieldElement {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("cannot div two numbers in different Fields");
        }

        // b^-1 = b^(-1+(prime-1)) = b^(prime-2)
        // a/b = a*b^-1 = a*b^(prime-2)
        let divisor = other.pow(other.prime.0 - 2);
        self.mul(divisor)
    }
}

#[cfg(test)]
mod tests {
    use super::FieldElementCreator;
    use crate::prime::Prime;

    const CREATOR11: FieldElementCreator = FieldElementCreator(Prime(11));
    const CREATOR13: FieldElementCreator = FieldElementCreator(Prime(13));
    const CREATOR19: FieldElementCreator = FieldElementCreator(Prime(19));
    const CREATOR_MAX: FieldElementCreator = FieldElementCreator(Prime(u32::MAX));

    // creator

    #[test]
    fn create_field_element_with_u32_1() {
        let element1 = CREATOR11.from_u32(10u32);
        let element2 = CREATOR11.from_u32(21u32);
        assert_eq!(element1, element2);
    }

    #[test]
    fn create_field_element_with_u32_2() {
        let element1 = CREATOR11.from_u32(0u32);
        let element2 = CREATOR11.from_u32(11u32);
        assert_eq!(element1, element2);
    }

    #[test]
    fn create_field_element_with_i64_1() {
        let element1 = CREATOR11.from_i64(10i64);
        let element2 = CREATOR11.from_i64(21i64);
        assert_eq!(element1, element2);
    }

    #[test]
    fn create_field_element_with_i64_2() {
        let element1 = CREATOR11.from_i64(0i64);
        let element2 = CREATOR11.from_i64(11i64);
        assert_eq!(element1, element2);
    }

    #[test]
    fn create_field_element_with_i64_3() {
        let element1 = CREATOR11.from_i64(-10i64);
        let element2 = CREATOR11.from_i64(-21i64);
        assert_eq!(element1, element2);
    }

    #[test]
    fn create_field_element_with_i64_4() {
        let element1 = CREATOR11.from_i64(-10i64);
        let element2 = CREATOR11.from_i64(1i64);
        assert_eq!(element1, element2);
    }

    // element

    #[test]
    fn eq_success() {
        let element1 = CREATOR11.from_u32(5);
        let element2 = CREATOR11.from_u32(5);
        assert!(element1 == element2);
    }

    #[test]
    fn ne_success() {
        let element1 = CREATOR11.from_u32(5);
        let element2 = CREATOR11.from_u32(8);
        assert!(element1 != element2);
    }

    #[test]
    fn add_success() {
        let element1 = CREATOR13.from_u32(7);
        let element2 = CREATOR13.from_u32(12);
        let element3 = CREATOR13.from_u32(6);

        assert_eq!(element1 + element2, element3);
    }

    #[test]
    fn add_overflow_success() {
        let element1 = CREATOR_MAX.from_u32(u32::MAX - 1);
        let element2 = CREATOR_MAX.from_u32(u32::MAX - 2);
        let element3 = CREATOR_MAX.from_u32(u32::MAX - 3);

        assert_eq!(element1 + element2, element3);
    }

    #[test]
    #[should_panic]
    fn add_failed() {
        let element1 = CREATOR11.from_u32(7);
        let element2 = CREATOR13.from_u32(12);
        let _ = element1 + element2;
    }

    #[test]
    fn sub_success_1() {
        let element1 = CREATOR13.from_u32(7);
        let element2 = CREATOR13.from_u32(12);
        let element3 = CREATOR13.from_u32(5);

        assert_eq!(element2 - element1, element3);
    }

    #[test]
    fn sub_success_2() {
        let element1 = CREATOR13.from_u32(12);
        let element2 = CREATOR13.from_u32(7);
        let element3 = CREATOR13.from_u32(8);

        assert_eq!(element2 - element1, element3);
    }

    #[test]
    #[should_panic]
    fn sub_failed() {
        let element1 = CREATOR11.from_u32(7);
        let element2 = CREATOR13.from_u32(12);
        let _ = element2 - element1;
    }

    #[test]
    fn mul_success() {
        let element1 = CREATOR13.from_u32(3);
        let element2 = CREATOR13.from_u32(12);

        let element3 = CREATOR13.from_u32(10);

        assert_eq!(element2 * element1, element3);
    }

    #[test]
    fn mul_overflow_success() {
        let element1 = CREATOR_MAX.from_u32(u32::MAX - 1);
        let element2 = CREATOR_MAX.from_u32(2);
        let element3 = CREATOR_MAX.from_u32(u32::MAX - 2);

        assert_eq!(element1 * element2, element3);
    }

    #[test]
    #[should_panic]
    fn mul_failed() {
        let element1 = CREATOR11.from_u32(3);
        let element2 = CREATOR13.from_u32(12);
        let _ = element1 * element2;
    }

    #[test]
    fn pow_success_1() {
        let element1 = CREATOR13.from_u32(7);
        let element2 = CREATOR13.from_u32(8);

        assert_eq!(element1.pow_u32(9), element2);
    }

    #[test]
    fn pow_success_2() {
        let element1 = CREATOR13.from_u32(7);
        let element2 = CREATOR13.from_u32(8);

        assert_eq!(element1.pow_i64(-3), element2);
    }

    #[test]
    fn pow_success_3() {
        let element1 = CREATOR13.from_u32(7);
        let element2 = CREATOR13.from_u32(8);

        assert_eq!(element1.pow_i64(-15), element2);
    }

    #[test]
    fn pow_overflow_success() {
        let element1 = CREATOR_MAX.from_u32(u32::MAX - 1);
        let element2 = CREATOR_MAX.from_u32(1);

        assert_eq!(element1.pow_u32(2), element2);
    }

    #[test]
    fn div_success() {
        let element1 = CREATOR19.from_u32(7);
        let element2 = CREATOR19.from_u32(5);
        let element3 = CREATOR19.from_u32(9);

        assert_eq!(element1/element2, element3);
    }
}
