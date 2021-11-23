use std::cmp::PartialEq;
use std::ops::{Add, Sub, Mul};
use num::checked_pow;

pub struct FieldElement {
    num: i32,
    prime: i32,
}

impl FieldElement {
    pub fn new(num: i32, prime: i32) -> Result<Self, String> {
        if num < 0 || num >= prime { // also means prime > 0
            let err = format!("Num {} not in field range 0 to {}", num, prime);
            return Err(err)
        }
        Ok(Self { num, prime })
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
        Self {
            num: (self.num + other.num) % self.prime,
            prime: self.prime
        }
    }
}

impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("cannot add two numbers in different Fields");
        }
        Self {
            num: (self.num - other.num) % self.prime,
            prime: self.prime
        }
    }
}

impl Mul for FieldElement {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("cannot add two numbers in different Fields");
        }
        Self {
            num: (self.num * other.num) % self.prime,
            prime: self.prime
        }
    }
}

#[cfg(test)]
mod tests {
    pub use super::FieldElement;

    #[test]
    fn new_success() {
        FieldElement::new(5, 11).unwrap();
    }

    #[test]
    #[should_panic]
    fn new_failed_num_too_big() {
        FieldElement::new(11, 11).unwrap();
    }

    #[test]
    #[should_panic]
    fn new_failed_num_is_nagetive() {
        FieldElement::new(-1, 11).unwrap();
    }

    #[test]
    fn eq_success() {
        let element1 = FieldElement::new(5, 11).unwrap();
        let element2 = FieldElement::new(5, 11).unwrap();
        assert!(element1 == element2);
    }

    #[test]
    fn ne_success() {
        let element1 = FieldElement::new(5, 11).unwrap();
        let element2 = FieldElement::new(8, 11).unwrap();
        assert!(element1 != element2);
    }

    #[test]
    fn add_success() {
        let element1 = FieldElement::new(7, 13).unwrap();
        let element2 = FieldElement::new(12, 13).unwrap();

        let element3 = FieldElement::new(6, 13).unwrap();

        assert!(element1 + element2 == element3);
    }

    #[test]
    #[should_panic]
    fn add_failed() {
        let element1 = FieldElement::new(7, 11).unwrap();
        let element2 = FieldElement::new(12, 13).unwrap();
        let _ = element1 + element2;
    }

    #[test]
    fn sub_success() {
        let element1 = FieldElement::new(7, 13).unwrap();
        let element2 = FieldElement::new(12, 13).unwrap();

        let element3 = FieldElement::new(5, 13).unwrap();

        assert!(element2 - element1 == element3);
    }

    #[test]
    #[should_panic]
    fn sub_failed() {
        let element1 = FieldElement::new(7, 11).unwrap();
        let element2 = FieldElement::new(12, 13).unwrap();
        let _ = element2 - element1;
    }

    #[test]
    fn mul_success() {
        let element1 = FieldElement::new(3, 13).unwrap();
        let element2 = FieldElement::new(12, 13).unwrap();

        let element3 = FieldElement::new(10, 13).unwrap();

        assert!(element2 * element1 == element3);
    }

    #[test]
    #[should_panic]
    fn mul_failed() {
        let element1 = FieldElement::new(3, 11).unwrap();
        let element2 = FieldElement::new(12, 13).unwrap();
        let _ = element1 * element2;
    }
}
