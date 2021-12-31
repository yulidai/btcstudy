use super::Error;
use std::fmt;

pub struct Stack(Vec<Vec<u8>>);

impl Stack {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, ele: Vec<u8>) {
        self.0.push(ele);
    }

    pub fn pop(&mut self) -> Result<Vec<u8>, Error> {
        self.0.pop().ok_or(Error::EmptyStack)
    }
}

impl fmt::Debug for Stack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut r = String::new();
        r.push('[');
        for bytes in &self.0 {
            r += &format!("{}, ", hex::encode(bytes));
        }
        r.push(']');

        f.debug_struct("Stack")
            .field("content", &r)
            .finish()
    }
}
