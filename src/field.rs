use std::cmp::PartialEq;
use std::convert::TryFrom;
use std::ops::{Add, Sub, Mul, Div};
use std::fmt::{self, Display};
use primitive_types::{U256, U512};
use super::prime::Prime;

pub struct FieldElementCreator(pub Prime);

impl FieldElementCreator {
    pub fn from_u256(&self, num: U256) -> FieldElement {
        let prime = self.0;
        let num = prime.create_element_from_u256(num);
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
    num: U256,
    prime: Prime,
}

impl FieldElement {
    pub fn is_zero(&self) -> bool {
        self.num.is_zero()
    }

    pub fn prime(&self) -> Prime {
        self.prime
    }

    pub fn pow_u256(&self, exponent: U256) -> Self {
        let exponent = self.prime.create_exponent_from_u256(exponent);
        self.pow(exponent)
    }

    pub fn pow_i64(&self, exponent: i64) -> Self {
        let exponent = self.prime.create_exponent_from_i64(exponent);
        self.pow(exponent)
    }

    fn pow(&self, mut exponent: U256) -> Self {
        let prime = self.prime.0;
        let zero = U256::zero();
        let one = U256::one();

        // pow(num, exponent, prime)
        let mut current = self.num;
        let mut result = one;
        while exponent > zero {
            if exponent % 2 == one {
                let result_u512 = ( U512::from(result) * U512::from(current) )% U512::from(prime);
                result = U256::try_from(result_u512).expect("overflow when pow FieldElement");
            }
            let current_u512 = U512::from(current).pow(2.into()) % U512::from(prime);
            current = U256::try_from(current_u512).expect("overflow when pow FieldElement");
            exponent = exponent / 2;
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
        let num = ( U512::from(self.num) + U512::from(other.num) ) % U512::from(self.prime.0);
        Self {
            num: U256::try_from(num).expect("overflow when add FieldElement"),
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
        let num = if self.num >= other.num {
            self.num - other.num
        } else {
            self.prime.0 - other.num + self.num
        };
        Self {
            num: num % self.prime.0,
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
        let num = ( U512::from(self.num) * U512::from(other.num) ) % U512::from(self.prime.0);
        Self {
            num: U256::try_from(num).expect("overflow when mul FieldElement"),
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
    use primitive_types::U256;
    use super::FieldElementCreator;
    use crate::prime::Prime;

    // creator

    #[test]
    fn create_field_element_with_u32_1() {
        let creator11 = FieldElementCreator(Prime(U256::from(11)));
        let element1 = creator11.from_u256(10u32.into());
        let element2 = creator11.from_u256(21u32.into());
        assert_eq!(element1, element2);
    }

    #[test]
    fn create_field_element_with_u32_2() {
        let creator11 = FieldElementCreator(Prime(U256::from(11)));
        let element1 = creator11.from_u256(0u32.into());
        let element2 = creator11.from_u256(11u32.into());
        assert_eq!(element1, element2);
    }

    #[test]
    fn create_field_element_with_i64_1() {
        let creator11 = FieldElementCreator(Prime(U256::from(11)));
        let element1 = creator11.from_i64(10i64);
        let element2 = creator11.from_i64(21i64);
        assert_eq!(element1, element2);
    }

    #[test]
    fn create_field_element_with_i64_2() {
        let creator11 = FieldElementCreator(Prime(U256::from(11)));
        let element1 = creator11.from_i64(0i64);
        let element2 = creator11.from_i64(11i64);
        assert_eq!(element1, element2);
    }

    #[test]
    fn create_field_element_with_i64_3() {
        let creator11 = FieldElementCreator(Prime(U256::from(11)));
        let element1 = creator11.from_i64(-10i64);
        let element2 = creator11.from_i64(-21i64);
        assert_eq!(element1, element2);
    }

    #[test]
    fn create_field_element_with_i64_4() {
        let creator11 = FieldElementCreator(Prime(U256::from(11)));
        let element1 = creator11.from_i64(-10i64);
        let element2 = creator11.from_i64(1i64);
        assert_eq!(element1, element2);
    }

    // element

    #[test]
    fn eq_success() {
        let creator11 = FieldElementCreator(Prime(U256::from(11)));
        let element1 = creator11.from_u256(5.into());
        let element2 = creator11.from_u256(5.into());
        assert!(element1 == element2);
    }

    #[test]
    fn ne_success() {
        let creator11 = FieldElementCreator(Prime(U256::from(11)));
        let element1 = creator11.from_u256(5.into());
        let element2 = creator11.from_u256(8.into());
        assert!(element1 != element2);
    }

    #[test]
    fn add_success() {
        let creator13 = FieldElementCreator(Prime(U256::from(13)));
        let element1 = creator13.from_u256(7.into());
        let element2 = creator13.from_u256(12.into());
        let element3 = creator13.from_u256(6.into());

        assert_eq!(element1 + element2, element3);
    }

    #[test]
    fn add_overflow_success() {
        let creator_max = FieldElementCreator(Prime(U256::MAX));
        let element1 = creator_max.from_u256(U256::MAX - 1);
        let element2 = creator_max.from_u256(U256::MAX - 2);
        let element3 = creator_max.from_u256(U256::MAX - 3);

        assert_eq!(element1 + element2, element3);
    }

    #[test]
    #[should_panic]
    fn add_failed() {
        let creator11 = FieldElementCreator(Prime(U256::from(11)));
        let creator13 = FieldElementCreator(Prime(U256::from(13)));
        let element1 = creator11.from_u256(7.into());
        let element2 = creator13.from_u256(12.into());
        let _ = element1 + element2;
    }

    #[test]
    fn sub_success_1() {
        let creator13 = FieldElementCreator(Prime(U256::from(13)));
        let element1 = creator13.from_u256(7.into());
        let element2 = creator13.from_u256(12.into());
        let element3 = creator13.from_u256(5.into());

        assert_eq!(element2 - element1, element3);
    }

    #[test]
    fn sub_success_2() {
        let creator13 = FieldElementCreator(Prime(U256::from(13)));
        let element1 = creator13.from_u256(12.into());
        let element2 = creator13.from_u256(7.into());
        let element3 = creator13.from_u256(8.into());

        assert_eq!(element2 - element1, element3);
    }

    #[test]
    #[should_panic]
    fn sub_failed() {
        let creator11 = FieldElementCreator(Prime(U256::from(11)));
        let creator13 = FieldElementCreator(Prime(U256::from(13)));
        let element1 = creator11.from_u256(7.into());
        let element2 = creator13.from_u256(12.into());
        let _ = element2 - element1;
    }

    #[test]
    fn mul_success() {
        let creator13 = FieldElementCreator(Prime(U256::from(13)));

        let element1 = creator13.from_u256(3.into());
        let element2 = creator13.from_u256(12.into());

        let element3 = creator13.from_u256(10.into());

        assert_eq!(element2 * element1, element3);
    }

    #[test]
    fn mul_overflow_success() {
        let creator_max = FieldElementCreator(Prime(U256::MAX));

        let element1 = creator_max.from_u256(U256::MAX - 1);
        let element2 = creator_max.from_u256(2.into());
        let element3 = creator_max.from_u256(U256::MAX - 2);

        assert_eq!(element1 * element2, element3);
    }

    #[test]
    #[should_panic]
    fn mul_failed() {
        let creator11 = FieldElementCreator(Prime(U256::from(11)));
        let creator13 = FieldElementCreator(Prime(U256::from(13)));

        let element1 = creator11.from_u256(3.into());
        let element2 = creator13.from_u256(12.into());
        let _ = element1 * element2;
    }

    #[test]
    fn pow_success_1() {
        let creator13 = FieldElementCreator(Prime(U256::from(13)));
        let element1 = creator13.from_u256(7.into());
        let element2 = creator13.from_u256(8.into());

        assert_eq!(element1.pow_u256(9.into()), element2);
    }

    #[test]
    fn pow_success_2() {
        let creator13 = FieldElementCreator(Prime(U256::from(13)));
        let element1 = creator13.from_u256(7.into());
        let element2 = creator13.from_u256(8.into());

        assert_eq!(element1.pow_i64(-3), element2);
    }

    #[test]
    fn pow_success_3() {
        let creator13 = FieldElementCreator(Prime(U256::from(13)));
        let element1 = creator13.from_u256(7.into());
        let element2 = creator13.from_u256(8.into());

        assert_eq!(element1.pow_i64(-15), element2);
    }

    #[test]
    fn pow_overflow_success() {
        let creator_max = FieldElementCreator(Prime(U256::MAX));
        let element1 = creator_max.from_u256(U256::MAX - 1);
        let element2 = creator_max.from_u256(1.into());

        assert_eq!(element1.pow_u256(2.into()), element2);
    }

    #[test]
    fn div_success() {
        let creator19 = FieldElementCreator(Prime(U256::from(19)));
        let element1 = creator19.from_u256(7.into());
        let element2 = creator19.from_u256(5.into());
        let element3 = creator19.from_u256(9.into());

        assert_eq!(element1/element2, element3);
    }
}
