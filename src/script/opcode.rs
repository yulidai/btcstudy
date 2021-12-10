#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Opcode {
    Op0 = 0x00,
    OpDup = 0x76,
    OpEqualverify = 0x88,
    OpHash160 = 0xa9,
    OpChecksig = 0xac,
}

impl Opcode {
    pub fn from_u8(u: u8) -> Option<Self> {
        match u {
            0x00 => Some(Self::Op0),
            0x76 => Some(Self::OpDup),
            0x88 => Some(Self::OpEqualverify),
            0xa9 => Some(Self::OpHash160),
            0xac => Some(Self::OpChecksig),
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
    fn opcode_from_u8_76() {
        assert_eq!(Opcode::from_u8(0x76), Some(Opcode::OpDup));
    }

    #[test]
    fn opcode_from_u8_88() {
        assert_eq!(Opcode::from_u8(0x88), Some(Opcode::OpEqualverify));
    }

    #[test]
    fn opcode_from_u8_a9() {
        assert_eq!(Opcode::from_u8(0xa9), Some(Opcode::OpHash160));
    }

    #[test]
    fn opcode_from_u8_ac() {
        assert_eq!(Opcode::from_u8(0xac), Some(Opcode::OpChecksig));
    }
}
