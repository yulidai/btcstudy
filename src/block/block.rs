use crate::transaction::Transaction;
use crate::merkle_tree::helper;
use crate::util::hash::{self, Hash256Value};
use super::{BlockHeader};

pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(header: BlockHeader, transactions: Vec<Transaction>) -> Self {
        Self { header, transactions }
    }

    pub fn make_merkle_root(tx_hashes: &Vec<Hash256Value>) -> Hash256Value {
        // reverse tx_hash
        let tx_hashes = tx_hashes.iter()
            .map(|tx_hash| {
                let mut tx_hash = tx_hash.to_vec();
                tx_hash.reverse();
                hash::convert_slice_into_hash256(&tx_hash)
            })
            .collect();

        // calculate and reverse result
        let mut root = helper::merkle_root(tx_hashes);
        root.reverse();
        root
    }
}

#[cfg(test)]
mod tests {
    use super::Block;
    use crate::util::hash;

    #[test]
    fn block_make_merkle_root() {
        let tx_hashes = [
            "42f6f52f17620653dcc909e58bb352e0bd4bd1381e2955d19c00959a22122b2e",
            "94c3af34b9667bf787e1c6a0a009201589755d01d02fe2877cc69b929d2418d4",
            "959428d7c48113cb9149d0566bde3d46e98cf028053c522b8fa8f735241aa953",
            "a9f27b99d5d108dede755710d4a1ffa2c74af70b4ca71726fa57d68454e609a2",
            "62af110031e29de1efcad103b3ad4bec7bdcf6cb9c9f4afdd586981795516577",
            "766900590ece194667e9da2984018057512887110bf54fe0aa800157aec796ba",
            "e8270fb475763bc8d855cfe45ed98060988c1bdcad2ffc8364f783c98999a208",
        ];
        let tx_hashes = tx_hashes.iter()
            .map(|tx_hash| {
                let tx_hash = hex::decode(tx_hash).unwrap();
                hash::convert_slice_into_hash256(&tx_hash)
            })
            .collect();
        let root = Block::make_merkle_root(&tx_hashes);
        
        assert_eq!(hex::encode(&root), "654d6181e18e4ac4368383fdc5eead11bf138f9b7ac1e15334e4411b3c4797d9");
    }
}
