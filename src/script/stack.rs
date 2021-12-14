use super::Error;

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
