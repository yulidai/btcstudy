use std::convert::From;
use super::Error;
use std::ops::Add;

pub struct Num(i64);

impl From<i64> for Num {
    fn from(i: i64) -> Self {
        Self(i)
    }
}

impl Num {
    pub fn encode(&self) -> Vec<u8> {
        let mut result = Vec::new();
        if self.0 == 0 {
            return result;
        }
        // value
        let mut abs = self.0.abs();
        while abs > 0 {
            result.push(abs as u8 & 0xff);
            abs >>= 8;
        }
        // symbol
        let fst_i = result.len() - 1; // little endian
        if result[fst_i] >= 0x80 {
            if self.0 < 0 {
                result.push(0x80);
            } else {
                result.push(0);
            }
        } else if self.0 < 0 {
            result[fst_i] |= 0x80;
        }

        result
    }

    pub fn decode(mut bytes: Vec<u8>) -> Result<Self, Error> {
        if bytes.len() == 0 {
            return Ok(Self(0));
        }
        // value
        bytes.reverse();
        let negative = bytes[0] & 0x80 > 0;
        let mut result: i64 = (if negative {
            bytes[0] & 0x7f
        } else {
            bytes[0]
        }).into();
        if bytes.len() > 1 {
            for byte in &bytes[1..] {
                let (new, overflow) = result.overflowing_shl(8);
                if overflow {
                    return Err(Error::NumDecodeOverflow);
                }
                result  = new + *byte as i64;
            }
        }
        // symbol
        if negative {
            result = -result;
        }

        Ok(Self(result))
    }

    pub fn value(&self) -> i64 {
        self.0
    }
}

impl Add for Num {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Num(self.0 + other.0)
    }
}

#[cfg(test)]
mod tests {
    use super::Num;

    #[test]
    fn script_num_encode_0() {
        let bytes = vec![];
        let num = Num::decode(bytes.clone()).unwrap();
        assert_eq!(num.value(), 0);
        assert_eq!(num.encode(), bytes);
    }

    #[test]
    fn script_num_encode_1() {
        let bytes = vec![1u8];
        let num = Num::decode(bytes.clone()).unwrap();
        assert_eq!(num.value(), 1);
        assert_eq!(num.encode(), bytes);
    }

    #[test]
    fn script_num_encode_2() {
        let num = Num::from(999i64);
        let bytes = vec![0xe7, 0x3];
        assert_eq!(num.value(), 999);
        assert_eq!(num.encode(), bytes);
    }
}
