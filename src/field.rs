use std::cmp::PartialEq;

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
        let field1 = FieldElement::new(5, 11).unwrap();
        let field2 = FieldElement::new(5, 11).unwrap();
        assert!(field1 == field2);
    }

    #[test]
    fn ne_success() {
        let field1 = FieldElement::new(5, 11).unwrap();
        let field2 = FieldElement::new(8, 11).unwrap();
        assert!(field1 != field2);
    }
}
