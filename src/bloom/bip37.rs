use crate::util::converter;

pub const BIP37_CONSTANT: u32 = 0xfba4c795;

pub struct BloomBip37 {
    bit_field: Vec<bool>,
    hash_count: u32,
    tweak: u32,
}

impl BloomBip37 {
    pub fn new(bit_field: usize, hash_count: u32, tweak: u32) -> Self {
        let bit_field = vec![false; bit_field];
        Self { bit_field, hash_count, tweak }
    }

    pub fn add(&mut self, bytes_vec: &Vec<Vec<u8>>) {
        for bytes in bytes_vec {
            for i in 0..self.hash_count {
                let seed = u32_mod_mul(i, BIP37_CONSTANT);
                let seed = u32_mod_add(seed, self.tweak);
                let h = mur3::murmurhash3_x86_32(bytes, seed);

                let h = converter::u32_into_usize(h).unwrap();
                let bit = h % self.bit_field.len();
                self.bit_field[bit] = true;
            }
        }
    }

    pub fn bit_field(&self) -> &Vec<bool> {
        &self.bit_field
    }
}

fn u32_mod_mul(l: u32, r: u32) -> u32 {
    let result = ( u64::from(l) * u64::from(r) ) % u64::from(u32::MAX);
    result as u32
}

fn u32_mod_add(l: u32, r: u32) -> u32 {
    let result = ( u64::from(l) + u64::from(r) ) % u64::from(u32::MAX);
    result as u32
}

#[cfg(test)]
mod tests {
    use super::BloomBip37;

    #[test]
    fn bloom_bip37_add() {
        let mut filter = BloomBip37::new(16, 2, 42);
        filter.add(&vec![b"hello world".to_vec(), b"goodbye".to_vec()]);

        let mut expect = vec![false; 16];
        for i in vec![5, 6, 9, 10] {
            expect[i] = true;
        }

        assert_eq!(filter.bit_field(), &expect);
    }
}
