#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Opcode {
    OpChecksig = 0xac,
}

impl Opcode {
    pub fn from_u8(u: u8) -> Option<Self> {
        match u {
            0xac => Some(Opcode::OpChecksig),
            _ => None
        }
    }

    pub fn value(&self) -> u8 {
        *self as u8
    }
}

#[cfg(test)]
mod tests {
    use super::Opcode;

    #[test]
    fn opcode_from_u8_1() {
        assert_eq!(Opcode::from_u8(0xac), Some(Opcode::OpChecksig));
    }
}
