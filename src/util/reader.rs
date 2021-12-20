pub struct Reader<'a> {
    index: usize,
    bytes: &'a [u8],
}

impl<'a> Reader<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { index: 0, bytes }
    }

    pub fn more(&mut self, count: usize) -> Result<&[u8], &'static str> {
        let new_index = self.index + count;
        if new_index > self.bytes.len() {
            return Err("not enough bytes for reading");
        }

        let result = &self.bytes[self.index..new_index];
        self.index = new_index;
        Ok(result)
    }

    pub fn used(&self) -> usize {
        self.index
    }
}

#[cfg(test)]
mod tests {
    use super::Reader;

    #[test]
    fn reader_content() {
        let mut reader = Reader::new(&[1u8; 5]);
        assert_eq!(reader.more(3).unwrap(), &[1u8; 3]);
    }

    #[test]
    fn reader_zero_with_max_index() {
        let mut reader = Reader::new(&[0u8; 5]);
        reader.more(5).unwrap();
        assert_eq!(reader.more(0).unwrap(), []);
    }
}
