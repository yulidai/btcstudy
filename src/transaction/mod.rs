pub use error::Error;
pub use version::Version;
pub use tx_in::TxIn;
pub use tx_out::TxOut;
pub use tx_fetcher::TxFetcher;
pub use transaction::Transaction;

pub type LockTime = Version;

mod error;
mod version;
mod tx_in;
mod tx_out;
mod tx_fetcher;
mod transaction;
