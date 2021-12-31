use std::fmt;

#[derive(Copy, Clone, PartialEq)]
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
    OpVerify = 0x69,
    OpDup = 0x76,
    OpEqual = 0x87,
    OpEqualverify = 0x88,
    OpAdd = 0x93,
    OpHash160 = 0xa9,
    OpCodeseparator = 0xab,
    OpChecksig = 0xac,
    OpChecksigverify = 0xad,
    OpCheckmultisig = 0xae,
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
            0x69 => Some(Self::OpVerify),
            0x76 => Some(Self::OpDup),
            0x87 => Some(Self::OpEqual),
            0x88 => Some(Self::OpEqualverify),
            0x93 => Some(Self::OpAdd),
            0xa9 => Some(Self::OpHash160),
            0xab => Some(Self::OpCodeseparator),
            0xac => Some(Self::OpChecksig),
            0xad => Some(Self::OpChecksigverify),
            0xae => Some(Self::OpCheckmultisig),
            _ => None
        }
    }

    pub fn value(&self) -> u8 {
        *self as u8
    }
}

impl fmt::Debug for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result = match self {
            Self::Op0 => "Op0",
            Self::Op1 => "Op1",
            Self::Op2 => "Op2",
            Self::Op3 => "Op3",
            Self::Op4 => "Op4",
            Self::Op5 => "Op5",
            Self::Op6 => "Op6",
            Self::Op7 => "Op7",
            Self::Op8 => "Op8",
            Self::Op9 => "Op9",
            Self::Op10 => "Op10",
            Self::Op11 => "Op11",
            Self::Op12 => "Op12",
            Self::Op13 => "Op13",
            Self::Op14 => "Op14",
            Self::Op15 => "Op15",
            Self::Op16 => "Op16",
            Self::OpVerify => "OpVerify",
            Self::OpDup => "OpDup",
            Self::OpEqual => "OpEqual",
            Self::OpEqualverify => "OpEqualverify",
            Self::OpAdd => "OpAdd",
            Self::OpHash160 => "OpHash160",
            Self::OpCodeseparator => "OpCodeseparator",
            Self::OpChecksig => "OpChecksig",
            Self::OpChecksigverify => "OpChecksigverify",
            Self::OpCheckmultisig => "OpCheckmultisig",
        };
        write!(f, "{}", result)
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
