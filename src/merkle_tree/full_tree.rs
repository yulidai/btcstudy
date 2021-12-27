use crate::util::hash::{Hash256Value};

pub struct MerkleTreeFull {
    pub inner: Vec<Vec<Option<Hash256Value>>>,
}

impl MerkleTreeFull {
    // TODO use link to replace build all empty tree
    pub fn new(total: u32) -> Self {
        let height = Self::height(total);
        let mut inner = Vec::new();
        for i in 0..height {
            let mut nodes = Vec::new();
            for _ in 0..(2u32.pow(i)) {
                nodes.push(None);
            }
            inner.push(nodes);
        }

        Self { inner }
    }

    pub fn height(mut total: u32) -> u32 {
        if total == 0 {
            return 0;
        }

        let copy = total;
        let mut height = 0;
        while total > 0 {
            height += 1;
            total >>= 1;
        }
        if copy > 2u32.pow(height-1) {
            height += 1;
        }

        height
    }
}

#[cfg(test)]
mod tests {
    use super::MerkleTreeFull;

    #[test]
    fn merkle_tree_height_0() {
        assert_eq!(MerkleTreeFull::height(0), 0);
    }

    #[test]
    fn merkle_tree_height_1() {
        assert_eq!(MerkleTreeFull::height(1), 1);
    }

    #[test]
    fn merkle_tree_height_2() {
        assert_eq!(MerkleTreeFull::height(2), 2);
    }

    #[test]
    fn merkle_tree_height_3() {
        assert_eq!(MerkleTreeFull::height(3), 3);
    }

    #[test]
    fn merkle_tree_height_4() {
        assert_eq!(MerkleTreeFull::height(4), 3);
    }

    #[test]
    fn merkle_tree_height_5() {
        assert_eq!(MerkleTreeFull::height(27), 6);
    }

    #[test]
    fn merkle_tree_new_0() {
        let tree = MerkleTreeFull::new(1);
        assert_eq!(tree.inner.len(), 1);
        assert_eq!(tree.inner[0].len(), 1);
        assert_eq!(tree.inner[0][0], None);
    }

    #[test]
    fn merkle_tree_new_1() {
        let tree = MerkleTreeFull::new(27);
        assert_eq!(tree.inner.len(), 6);
        assert_eq!(tree.inner[5].len(), 32);
        assert_eq!(tree.inner[0][0], None);
    }
}
