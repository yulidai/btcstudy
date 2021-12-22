use primitive_types::U256;
use super::{BlockHeader, Bits, Error};

const TWO_WEEKS: u32 = 60 * 60 * 24 * 14;
const TIME_DIFF_MAX: u32 = TWO_WEEKS * 4;
const TIME_DIFF_MIN: u32 = TWO_WEEKS / 4;

pub fn calculate_new_bits(fst_block: &BlockHeader, lst_block: &BlockHeader) -> Result<Bits, Error> {
    if lst_block.timestamp.value() < fst_block.timestamp.value() {
        return Err(Error::TimeDiffIsNegativeNumber);
    }
    let mut time_diff = lst_block.timestamp.value() - fst_block.timestamp.value();
    if time_diff > TIME_DIFF_MAX {
        time_diff = TIME_DIFF_MAX;
    }
    if time_diff < TIME_DIFF_MIN {
        time_diff = TIME_DIFF_MIN;
    }
    let new_target = lst_block.bits.to_target()? * U256::from(time_diff) / U256::from(TWO_WEEKS);
    
    Bits::from_target(new_target)
}

#[cfg(test)]
mod tests {
    use crate::block::BlockHeader;

    #[test]
    fn block_calculate_new_target() {
        let fst_bytes = hex::decode("00000020fdf740b0e49cf75bb3d5168fb3586f7613dcc5cd89675b0100000000000000002e37b144c0baced07eb7e7b64da916cd3121f2427005551aeb0ec6a6402ac7d7f0e4235954d801187f5da9f5").unwrap();
        let fst_block = BlockHeader::parse(&fst_bytes).unwrap();

        let lst_bytes = hex::decode("000000201ecd89664fd205a37566e694269ed76e425803003628ab010000000000000000bfcade29d080d9aae8fd461254b041805ae442749f2a40100440fc0e3d5868e55019345954d80118a1721b2e").unwrap();
        let lst_block = BlockHeader::parse(&lst_bytes).unwrap();

        // bits.to_target() == "19eaf000000000000000000000000000000000000000000", not same with the demo of book, check in the future
        let bits = super::calculate_new_bits(&fst_block, &lst_block).unwrap();
        assert_eq!(hex::encode(bits.serialize()), "af9e0118");
    }
}
