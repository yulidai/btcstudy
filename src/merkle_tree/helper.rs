use crate::util::hash::{self, Hash256Value};

pub fn merkle_parent(l: Hash256Value, r: Hash256Value) -> Hash256Value {
    let p = [l, r].concat();
    hash::hash256(&p)
}

#[cfg(test)]
mod tests {
    use crate::util::hash;

    #[test]
    fn merkle_tree_merkle_parent() {
        let l = hash::convert_slice_into_hash256(&hex::decode("c117ea8ec828342f4dfb0ad6bd140e03a50720ece40169ee38bdc15d9eb64cf5").unwrap());
        let r = hash::convert_slice_into_hash256(&hex::decode("c131474164b412e3406696da1ee20ab0fc9bf41c8f05fa8ceea7a08d672d7cc5").unwrap());

        let result = super::merkle_parent(l, r);
        assert_eq!(hex::encode(result), "8b30c5ba100f6f2e5ad1e2a742e5020491240f8eb514fe97c713c31718ad7ecd");
    }
}
