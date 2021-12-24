use crate::util::hash::{self, Hash256Value};

pub fn merkle_parent_level(mut hashes: Vec<Hash256Value>) -> Vec<Hash256Value> {
    if hashes.len() % 2 == 1 {
        let last = hashes.last().unwrap().clone();
        hashes.push(last);
    }

    let result_len = hashes.len() / 2;
    let mut result: Vec<Hash256Value> = Vec::with_capacity(result_len);
    for i in 0..result_len {
        let parent_hash = merkle_parent(hashes[i*2], hashes[i*2+1]);
        result.push(parent_hash);
    }

    result
}

pub fn merkle_parent(l: Hash256Value, r: Hash256Value) -> Hash256Value {
    let p = [l, r].concat();
    hash::hash256(&p)
}

#[cfg(test)]
mod tests {
    use crate::util::hash;

    #[test]
    fn merkle_tree_merkle_parent_level_0() {
        let l = hash::convert_slice_into_hash256(&hex::decode("c117ea8ec828342f4dfb0ad6bd140e03a50720ece40169ee38bdc15d9eb64cf5").unwrap());
        let r = hash::convert_slice_into_hash256(&hex::decode("c131474164b412e3406696da1ee20ab0fc9bf41c8f05fa8ceea7a08d672d7cc5").unwrap());
        let result = super::merkle_parent_level(vec![l, r, l, r]);
        assert_eq!(result.len(), 2);
        assert_eq!(hex::encode(result[0]), "8b30c5ba100f6f2e5ad1e2a742e5020491240f8eb514fe97c713c31718ad7ecd");
        assert_eq!(result[0], result[1]);
    }

    #[test]
    fn merkle_tree_merkle_parent_level_1() {
        let l = hash::convert_slice_into_hash256(&hex::decode("c117ea8ec828342f4dfb0ad6bd140e03a50720ece40169ee38bdc15d9eb64cf5").unwrap());
        let r = hash::convert_slice_into_hash256(&hex::decode("c131474164b412e3406696da1ee20ab0fc9bf41c8f05fa8ceea7a08d672d7cc5").unwrap());
        let result = super::merkle_parent_level(vec![l, r, l]);
        assert_eq!(result.len(), 2);
        assert_eq!(hex::encode(result[0]), "8b30c5ba100f6f2e5ad1e2a742e5020491240f8eb514fe97c713c31718ad7ecd");
        assert_eq!(hex::encode(result[1]), "0f5dc42b1311f693cd458ee8433279e0e1144aae800f4da70c9bb99822354ccd");
    }

    #[test]
    fn merkle_tree_merkle_parent() {
        let l = hash::convert_slice_into_hash256(&hex::decode("c117ea8ec828342f4dfb0ad6bd140e03a50720ece40169ee38bdc15d9eb64cf5").unwrap());
        let r = hash::convert_slice_into_hash256(&hex::decode("c131474164b412e3406696da1ee20ab0fc9bf41c8f05fa8ceea7a08d672d7cc5").unwrap());

        let result = super::merkle_parent(l, r);
        assert_eq!(hex::encode(result), "8b30c5ba100f6f2e5ad1e2a742e5020491240f8eb514fe97c713c31718ad7ecd");
    }
}
