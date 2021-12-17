use super::{Transaction, Error};
use crate::util::hash::Hash256Value;
use std::collections::HashMap;

pub struct TxFetcher {
    cache: HashMap<Hash256Value, Transaction>,
}

impl TxFetcher {
    pub fn get_url(testnet: bool) -> &'static str {
        if testnet {
            "https://blockstream.info/testnet/api"
        } else {
            "https://blockstream.info/api"
        }
    }

    pub fn new() -> Self {
        Self { cache: HashMap::new() }
    }

    pub fn fetch(&mut self, tx_hash: &Hash256Value, testnet: bool, fresh: bool) -> Result<Transaction, Error> {
        if fresh {
            self.cache.remove(tx_hash);
        }
        if let Some(tx) = self.cache.get(tx_hash) {
            return Ok((*tx).clone());
        }
        let tx = Self::fetch_without_cache(tx_hash, testnet)?;
        self.cache.insert(tx_hash.clone(), tx.clone());

        Ok(tx)
    }

    pub fn fetch_without_cache(tx_hash: &Hash256Value, testnet: bool) -> Result<Transaction, Error> {
        let url = format!("{}/tx/{}/hex", Self::get_url(testnet), hex::encode(tx_hash));
        let body = reqwest::blocking::get(&url)?.text()?;
        let body = hex::decode(body)?;
        let (tx, _) = Transaction::parse(&body)?;

        Ok(tx)
    }
}

#[cfg(test)]
mod tests {
    use super::TxFetcher;

    #[test]
    fn tx_fetcher_get() {
        let mut tx_hash = [0u8; 32];
        tx_hash.copy_from_slice(&hex::decode("fe28050b93faea61fa88c4c630f0e1f0a1c24d0082dd0e10d369e13212128f33").unwrap());

        let mut fetcher = TxFetcher::new();        
        let tx = fetcher.fetch(&tx_hash, false, false).unwrap();
        assert_eq!(tx.version.value(), 1);
    }
}
