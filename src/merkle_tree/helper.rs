use crate::util::hash::{self, Hash256Value};

pub fn merkle_root(mut hashes: Vec<Hash256Value>) -> Option<Hash256Value> {
    if hashes.len() == 0 {
        return None;
    }
    while hashes.len() > 1 {
        hashes = merkle_parent_level(hashes);
    }
    Some(hashes[0])
}

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
    fn merkle_tree_merkle_root() {
        let hash_hex = [
            "c117ea8ec828342f4dfb0ad6bd140e03a50720ece40169ee38bdc15d9eb64cf5",
            "c131474164b412e3406696da1ee20ab0fc9bf41c8f05fa8ceea7a08d672d7cc5",
            "f391da6ecfeed1814efae39e7fcb3838ae0b02c02ae7d0a5848a66947c0727b0",
            "3d238a92a94532b946c90e19c49351c763696cff3db400485b813aecb8a13181",
            "10092f2633be5f3ce349bf9ddbde36caa3dd10dfa0ec8106bce23acbff637dae",
            "7d37b3d54fa6a64869084bfd2e831309118b9e833610e6228adacdbd1b4ba161",
            "8118a77e542892fe15ae3fc771a4abfd2f5d5d5997544c3487ac36b5c85170fc",
            "dff6879848c2c9b62fe652720b8df5272093acfaa45a43cdb3696fe2466a3877",
            "b825c0745f46ac58f7d3759e6dc535a1fec7820377f24d4c2c6ad2cc55c0cb59",
            "95513952a04bd8992721e9b7e2937f1c04ba31e0469fbe615a78197f68f52b7c",
            "2e6d722e5e4dbdf2447ddecc9f7dabb8e299bae921c99ad5b0184cd9eb8e5908",
            "b13a750047bc0bdceb2473e5fe488c2596d7a7124b4e716fdd29b046ef99bbf0",
        ];
        let hashes = hash_hex.iter()
            .map(|bytes| hex::decode(bytes).unwrap())
            .map(|bytes| hash::convert_slice_into_hash256(&bytes))
            .collect();
        let root = super::merkle_root(hashes).unwrap();
        assert_eq!(hex::encode(&root), "acbcab8bcc1af95d8d563b77d24c3d19b18f1486383d75a5085c4e86c86beed6");
    }

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
