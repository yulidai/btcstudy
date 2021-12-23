use super::{Version, Bits, BlockHash, MerkleRoot, Timestamp, Error};
use crate::util::{
    hash::{self, Hash256Value},
    io::{ReaderManager, BytesReader},
};
use std::fmt;
use primitive_types::U256;

pub struct BlockHeader {
    pub version: Version,
    pub prev_block: BlockHash,
    pub merkle_root: MerkleRoot,
    pub timestamp: Timestamp,
    pub bits: Bits,
    pub nonce: [u8; 4],
}

impl BlockHeader {
    pub fn parse(bytes: &[u8]) -> Result<Self, Error> {
        let mut reader = BytesReader::new(bytes);
        let mut reader = ReaderManager::new(&mut reader);
        Self::parse_reader(&mut reader)
    }

    pub fn parse_reader(reader: &mut ReaderManager) -> Result<Self, Error> {
        let version = Version::parse(&reader.more(4)?)?;
        let prev_block = BlockHash::parse(reader)?;
        let merkle_root = MerkleRoot::parse(reader)?;
        let timestamp = Timestamp::parse(&reader.more(4)?)?;
        let bits = Bits::parse_reader(reader)?;

        let mut nonce = [0u8; 4];
        nonce.copy_from_slice(&reader.more(4)?);

        Ok(Self { version, prev_block, merkle_root, timestamp, bits, nonce })
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.append(&mut self.version.serialize().to_vec());
        result.append(&mut self.prev_block.serialize().to_vec());
        result.append(&mut self.merkle_root.serialize().to_vec());
        result.append(&mut self.timestamp.serialize().to_vec());
        result.append(&mut self.bits.serialize().to_vec());
        result.append(&mut self.nonce.to_vec());

        result
    }

    pub fn id(&self) -> Hash256Value {
        let mut id = hash::hash256(&self.serialize()).to_vec();
        id.reverse(); // little endian
        hash::convert_slice_into_hash256(&id)
    }

    pub fn bip9(&self) -> bool {
        self.version.value() >> 29 == 1
    }

    pub fn bip91(&self) -> bool {
        self.version.value() >> 4 & 1 == 1
    }

    pub fn bip141(&self) -> bool {
        self.version.value() >> 1 & 1 == 1
    }

    pub fn check_pow(&self) -> Result<bool, Error> {
        let target = self.bits.to_target()?;
        let proof = U256::from_big_endian(&self.id());

        Ok(proof < target)
    }

    pub fn difficulty(&self) -> Result<U256, Error> {
        self.bits.to_diff()
    }
}

impl fmt::Debug for BlockHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BlockHeader")
            .field("version", &hex::encode(self.version.serialize()))
            .field("prev_block", &hex::encode(self.prev_block.serialize()))
            .field("merkle_root", &hex::encode(self.merkle_root.serialize()))
            .field("timestamp", &hex::encode(self.timestamp.serialize()))
            .field("bits", &hex::encode(self.bits.serialize()))
            .field("nonce", &hex::encode(&self.nonce))
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::BlockHeader;

    fn get_block_header() -> (BlockHeader, Vec<u8>) {
        let bytes = hex::decode("020000208ec39428b17323fa0ddec8e887b4a7c53b8c0a0a220cfd0000000000000000005b0750fce0a889502d40508d39576821155e9c9e3f5c3157f961db38fd8b25be1e77a759e93c0118a4ffd71d").unwrap();
        (BlockHeader::parse(&&bytes).unwrap(), bytes)
    }

    #[test]
    fn block_header_parse_reader() {
        let (header, _) = get_block_header();
        assert_eq!(hex::encode(header.version.serialize()), "02000020");
        assert_eq!(hex::encode(header.prev_block.serialize()), "8ec39428b17323fa0ddec8e887b4a7c53b8c0a0a220cfd000000000000000000");
        assert_eq!(hex::encode(header.merkle_root.serialize()), "5b0750fce0a889502d40508d39576821155e9c9e3f5c3157f961db38fd8b25be");
        assert_eq!(hex::encode(header.timestamp.serialize()), "1e77a759");
        assert_eq!(hex::encode(header.bits.serialize()), "e93c0118");
        assert_eq!(hex::encode(&header.nonce), "a4ffd71d");
    }

    #[test]
    fn block_header_serialize() {
        let (header, bytes) = get_block_header();
        assert_eq!(header.serialize(), bytes);
    }

    #[test]
    fn block_header_id() {
        let (header, _) = get_block_header();
        assert_eq!(hex::encode(header.id()), "0000000000000000007e9e4c586439b0cdbe13b1370bdd9435d76a644d047523");
    }

    #[test]
    fn block_header_bip9() {
        let (header, _) = get_block_header();
        assert_eq!(header.bip9(), true);
    }

    #[test]
    fn block_header_bip91() {
        let (header, _) = get_block_header();
        assert_eq!(header.bip91(), false);
    }

    #[test]
    fn block_header_bip141() {
        let (header, _) = get_block_header();
        assert_eq!(header.bip141(), true);
    }

    #[test]
    fn block_header_check_pow() {
        let (header, _) = get_block_header();
        assert_eq!(header.check_pow().unwrap(), true);
    }

    #[test]
    fn block_difficulty() {
        let (header, _) = get_block_header();
        assert_eq!(header.difficulty().unwrap(), 888171856257u64.into());
    }
}
