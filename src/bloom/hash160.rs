use crate::util::hash;
use primitive_types::U256;

pub struct BloomHash160 {
    bit_field: usize,
}

impl BloomHash160 {
    pub fn new(bit_field: usize) -> Self {
        Self { bit_field }
    }

    pub fn mark(&self, bytes_vec: &Vec<Vec<u8>>) -> Vec<bool> {
        let mut result = vec![false; self.bit_field];
        for bytes in bytes_vec {
            let i = hash::hash160(bytes);
            let i = U256::from_big_endian(&i) % self.bit_field;
            let i = i.as_usize(); // never panic, because of self.bit_field < usize::MAX
            result[i] = true;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::BloomHash160;

    #[test]
    fn bloom_hash160_mark() {
        let filter = BloomHash160::new(10);
        let result = filter.mark(&vec![b"hello world".to_vec(), b"goodbye".to_vec()]);

        let mut expect = vec![false; 10];
        expect[0] = true;
        expect[1] = true;

        assert_eq!(result, expect);
    }
}
