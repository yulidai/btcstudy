use std::convert::{TryFrom, TryInto};
use std::io::Error as IoError;
use crate::util::{math, Reader, io::ReaderManager};

pub fn encode(num: u64) -> Vec<u8> {
    let mut result = Vec::new();
    if num < 253 {
        let num = u8::try_from(num).expect("num is out of range of u8 within Varint::encode");
        result.push(num);
    } else if num < u16::MAX.into() {
        let num = u16::try_from(num).expect("num is out of range of u16 within Varint::encode");
        result.push(0xfdu8);
        result.append(&mut num.to_le_bytes().to_vec());
    } else if num < u32::MAX.into() {
        let num = u32::try_from(num).expect("num is out of range of u32 within Varint::encode");
        result.push(0xfeu8);
        result.append(&mut num.to_le_bytes().to_vec());
    } else {
        let num = u64::try_from(num).expect("num is out of range of u64 within Varint::encode");
        result.push(0xffu8);
        result.append(&mut num.to_le_bytes().to_vec());
    }
    result
}

pub fn decode(bytes: &[u8]) -> Result<(usize, u8), &'static str> {
    let total_length = bytes.len();
    if total_length == 0 {
        return Err("cannot decode empty bytes into u64 within varint::decode");
    }

    let byte = bytes[0];
    let result = if byte == 0xfd {
        math::check_range_add(0, 2, total_length)?;
        let num = bytes[1..3].try_into().map_err(|_| "faield to convert slice into array within varint::decode")?;
        let num = u16::from_le_bytes(num);
        (num as usize, 2 + 1)
    } else if byte == 0xfe {
        math::check_range_add(0, 4, total_length)?;
        let num = bytes[1..5].try_into().map_err(|_| "faield to convert slice into array within varint::decode")?;
        let num = u32::from_le_bytes(num);
        (num as usize, 4 + 1)
    } else if byte == 0xff {
        math::check_range_add(0, 8, total_length)?;
        let num = bytes[1..9].try_into().map_err(|_| "faield to convert slice into array within varint::decode")?;
        let num = usize::from_le_bytes(num);
        (num, 8 + 1)
    } else {
        (byte as usize, 1)
    };

    Ok(result)
}

pub fn decode_with_reader(reader: &mut Reader) -> Result<usize, &'static str> {
    let byte = reader.more(1)?[0];
    Ok(match byte {
        0xfd => u16::from_le_bytes(reader.more(2)?.try_into().unwrap()) as usize,
        0xfe => u32::from_le_bytes(reader.more(4)?.try_into().unwrap()) as usize,
        0xff => usize::from_le_bytes(reader.more(8)?.try_into().unwrap()),
        _ => byte as usize
    })
}

pub fn decode_with_reader_manager(reader: &mut ReaderManager) -> Result<usize, IoError> {
    let byte = reader.more(1)?[0];
    Ok(match byte {
        0xfd => u16::from_le_bytes(reader.more(2)?.try_into().unwrap()) as usize,
        0xfe => u32::from_le_bytes(reader.more(4)?.try_into().unwrap()) as usize,
        0xff => usize::from_le_bytes(reader.more(8)?.try_into().unwrap()),
        _ => byte as usize
    })
}

#[cfg(test)]
mod tests {
    use crate::util::Reader;

    #[test]
    fn varint_encode_1() {
        assert_eq!(super::encode(252), vec![252u8]);
    }

    #[test]
    fn varint_encode_2() {
        assert_eq!(super::encode(255), vec![0xfdu8, 0xff, 0]);
    }

    #[test]
    fn varint_encode_3() {
        assert_eq!(super::encode(555), vec![0xfdu8, 0x2b, 0x02]);
    }

    #[test]
    fn varint_encode_4() {
        assert_eq!(super::encode(70015), vec![0xfeu8, 0x7f, 0x11, 0x01, 0x00]);
    }
    
    #[test]
    fn varint_encode_5() {
        assert_eq!(super::encode(18005558675309), vec![0xffu8, 0x6d, 0xc7, 0xed, 0x3e, 0x60, 0x10, 0, 0]);
    }

    #[test]
    fn varint_decode_1() {
        assert_eq!(super::decode(&[252u8]).unwrap(), (252, 1));
    }

    #[test]
    fn varint_decode_2() {
        assert_eq!(super::decode(&[0xfdu8, 0xff, 0]).unwrap(), (255, 3));
    }

    #[test]
    fn varint_decode_3() {
        assert_eq!(super::decode(&[0xfdu8, 0x2b, 0x02]).unwrap(), (555, 3));
    }

    #[test]
    fn varint_decode_4() {
        assert_eq!(super::decode(&[0xfeu8, 0x7f, 0x11, 0x01, 0x00]).unwrap(), (70015, 5));
    }
    
    #[test]
    fn varint_decode_5() {
        assert_eq!(super::decode(&[0xffu8, 0x6d, 0xc7, 0xed, 0x3e, 0x60, 0x10, 0, 0]).unwrap(), (18005558675309, 9));
    }

    #[test]
    fn varint_decode_with_reader_1() {
        let mut reader = Reader::new(&[252u8]);
        assert_eq!(super::decode_with_reader(&mut reader).unwrap(), 252);
        assert_eq!(reader.used(), 1);
    }

    #[test]
    fn varint_decode_with_reader_2() {
        let mut reader = Reader::new(&[0xfdu8, 0xff, 0]);
        assert_eq!(super::decode_with_reader(&mut reader).unwrap(), 255);
        assert_eq!(reader.used(), 3);
    }

    #[test]
    fn varint_decode_with_reader_3() {
        let mut reader = Reader::new(&[0xfdu8, 0x2b, 0x02]);
        assert_eq!(super::decode_with_reader(&mut reader).unwrap(), 555);
        assert_eq!(reader.used(), 3);
    }

    #[test]
    fn varint_decode_with_reader_4() {
        let mut reader = Reader::new(&[0xfeu8, 0x7f, 0x11, 0x01, 0x00]);
        assert_eq!(super::decode_with_reader(&mut reader).unwrap(), 70015);
        assert_eq!(reader.used(), 5);
    }

    #[test]
    fn varint_decode_with_reader_5() {
        let mut reader = Reader::new(&[0xffu8, 0x6d, 0xc7, 0xed, 0x3e, 0x60, 0x10, 0, 0]);
        assert_eq!(super::decode_with_reader(&mut reader).unwrap(), 18005558675309);
        assert_eq!(reader.used(), 9);
    }
}
