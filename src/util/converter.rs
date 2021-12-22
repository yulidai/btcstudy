use std::convert::TryFrom;

// TODO use macro
pub fn le_bytes_into_u32(bytes: &[u8]) -> Result<u32, &'static str> {
    if bytes.len() != 4 {
        return Err("u32 need slice with 4 byte");
    }
    let mut result = [0u8; 4];
    result.copy_from_slice(bytes);

    Ok(u32::from_le_bytes(result))
}

pub fn u32_into_usize(num: u32) -> Result<usize, &'static str> {
    usize::try_from(num).map_err(|_| "failed convert u32 into usize")
}

pub fn usize_into_u32(num: usize) -> Result<u32, &'static str> {
    u32::try_from(num).map_err(|_| "failed convert usize into u32")
}
