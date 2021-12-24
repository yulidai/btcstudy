pub use error::Error;
pub use bits::Bits;
pub use block::Block;
pub use block_hash::{BlockHash, GENESIS_BLOCK_HASH};
pub use block_header::BlockHeader;
pub use version::Version;

mod error;
mod bits;
mod block;
mod block_hash;
mod block_header;
mod version;
pub mod helper;

pub type Timestamp = Version;
pub type MerkleRoot = BlockHash;
