pub use error::Error;
pub use version::Version;
pub use tx_in::TxIn;
pub use tx_out::TxOut;

pub type LockTime = Version;

mod error;
mod version;
mod tx_in;
mod tx_out;
mod transaction;
