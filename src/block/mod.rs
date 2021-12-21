pub use error::Error;
pub use block_hash::BlockHash;
pub use block_header::BlockHeader;
pub use version::Version;

mod error;
mod block_hash;
mod block_header;
mod version;

pub type Timestamp = Version;
pub type MerkleRoot = BlockHash;
