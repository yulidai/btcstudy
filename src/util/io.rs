use std::io::{self, Error, Read};

pub struct ReaderManager<'a> {
    reader: Box<&'a mut dyn Read>,
}

impl<'a> ReaderManager<'a> {
    pub fn new(reader: &'a mut dyn Read) -> Self {
        Self {
            reader: Box::new(reader)
        }
    }

    pub fn more(&mut self, len: usize) -> Result<Vec<u8>, Error> {
        let mut buff = Vec::with_capacity(len);
        for _ in 0..len {
            buff.push(0);
        }
        self.reader.read_exact(&mut buff)?;
    
        Ok(buff)
    }
}

pub struct BytesReader<'a> {
    index: usize,
    bytes: &'a [u8],
}

impl<'a> Read for BytesReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        buf[0] = self.bytes[self.index];
        self.index += 1;

        Ok(1)
    }
}

impl<'a> BytesReader<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { index: 0, bytes }
    }
}
