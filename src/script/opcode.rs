#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Opcode {
    Op0 = 0x00,
    Op1 = 0x51,
    Op2 = 0x52,
    Op3 = 0x53,
    Op4 = 0x54,
    Op5 = 0x55,
    Op6 = 0x56,
    Op7 = 0x57,
    Op8 = 0x58,
    Op9 = 0x59,
    Op10 = 0x5a,
    Op11 = 0x5b,
    Op12 = 0x5c,
    Op13 = 0x5d,
    Op14 = 0x5e,
    Op15 = 0x5f,
    Op16 = 0x60,
    OpDup = 0x76,
    OpEqual = 0x87,
    OpEqualverify = 0x88,
    OpAdd = 0x93,
    OpHash160 = 0xa9,
    OpChecksig = 0xac,
}

impl Opcode {
    pub fn from_u8(u: u8) -> Option<Self> {
        match u {
            0x00 => Some(Self::Op0),
            0x51 => Some(Self::Op1),
            0x52 => Some(Self::Op2),
            0x53 => Some(Self::Op3),
            0x54 => Some(Self::Op4),
            0x55 => Some(Self::Op5),
            0x56 => Some(Self::Op6),
            0x57 => Some(Self::Op7),
            0x58 => Some(Self::Op8),
            0x59 => Some(Self::Op9),
            0x5a => Some(Self::Op10),
            0x5b => Some(Self::Op11),
            0x5c => Some(Self::Op12),
            0x5d => Some(Self::Op13),
            0x5e => Some(Self::Op14),
            0x5f => Some(Self::Op15),
            0x60 => Some(Self::Op16),
            0x76 => Some(Self::OpDup),
            0x87 => Some(Self::OpEqual),
            0x88 => Some(Self::OpEqualverify),
            0x93 => Some(Self::OpAdd),
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
