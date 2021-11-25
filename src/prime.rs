#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Prime(pub u32);

impl Prime {
    pub fn new(prime: u32) -> Self {
        //TODO check prime is a prime number
        Self(prime)
    }

    pub fn create_element_from_u32(&self, num: u32) -> u32 {
        num % self.0
    }

    pub fn create_element_from_i64(&self, num: i64) -> u32 {
        let prime = self.0 as i64;
        num.rem_euclid(prime) as u32
    }

    // a^(p-1)%p=1, so exponent in prime(13): 9, -3=-3+12=9, -15=-15+12*2=9
    pub fn create_exponent_from_u32(&self, exponent: u32) -> u32 {
        exponent % (self.0 - 1)
    }

    pub fn create_exponent_from_i64(&self, exponent: i64) -> u32 {
        let prime = self.0 as i64;
        exponent.rem_euclid(prime - 1) as u32
    }
}

#[cfg(test)]
mod tests {
    use super::Prime;

    const PRIME13: Prime = Prime(13);

    #[test]
    fn create_element_from_u32_1() {
        assert_eq!(PRIME13.create_element_from_u32(10), 10);
    }

    #[test]
    fn create_element_from_u32_2() {
        assert_eq!(PRIME13.create_element_from_u32(23), 10);
    }

    #[test]
    fn create_element_from_i64_1() {
        assert_eq!(PRIME13.create_element_from_i64(-3), 10);
    }

    #[test]
    fn create_element_from_i64_2() {
        assert_eq!(PRIME13.create_element_from_i64(-16), 10);
    }

    #[test]
    fn create_element_from_i64_3() {
        assert_eq!(PRIME13.create_element_from_i64(10), 10);
    }

    #[test]
    fn create_element_from_i64_4() {
        assert_eq!(PRIME13.create_element_from_i64(23), 10);
    }

    #[test]
    fn create_exponent_from_u32_1() {
        assert_eq!(PRIME13.create_exponent_from_u32(9), 9);
    }

    #[test]
    fn create_exponent_from_u32_2() {
        assert_eq!(PRIME13.create_exponent_from_u32(21), 9);
    }

    #[test]
    fn prime_create_exponent_from_i64_1() {
        assert_eq!(PRIME13.create_exponent_from_i64(-3), 9);
    }

    #[test]
    fn prime_create_exponent_from_i64_2() {
        assert_eq!(PRIME13.create_exponent_from_i64(-15), 9);
    }

    #[test]
    fn prime_create_exponent_from_i64_3() {
        assert_eq!(PRIME13.create_exponent_from_i64(9), 9);
    }

    #[test]
    fn prime_create_exponent_from_i64_4() {
        assert_eq!(PRIME13.create_exponent_from_i64(21), 9);
    }
}
