use crate::util::hash::Hash256Value;
use super::MerkleTreeFull;

pub struct MerkleTreeCursor {
    inner: Vec<Vec<Option<Hash256Value>>>,
    depth: usize,
    index: usize,
}

impl MerkleTreeCursor {
    pub fn new(total: u32) -> Self {
        Self {
            inner: MerkleTreeFull::new(total).inner,
            depth: 0,
            index: 0,
        }
    }

    pub fn up(&mut self) {
        if self.depth == 0 {
            return;
        }
        self.depth -= 1;
        self.index = self.index / 2;
    }

    pub fn left(&mut self) {
        self.depth += 1;
        self.index = self.index * 2;
    }

    pub fn right(&mut self) {
        self.depth += 1;
        self.index = self.index * 2 + 1;
    }

    pub fn root(&self) -> &Option<Hash256Value> {
        &self.inner[0][0]
    }

    pub fn set_current_node(&mut self, node: Option<Hash256Value>) {
        self.inner[self.depth][self.index] = node;
    }

    pub fn get_current_node(&self) -> &Option<Hash256Value> {
        &self.inner[self.depth][self.index]
    }

    pub fn get_left_node(&self) -> &Option<Hash256Value> {
        &self.inner[self.depth+1][self.index*2]
    }

    pub fn get_right_node(&self) -> &Option<Hash256Value> {
        &self.inner[self.depth+1][self.index*2 + 1]
    }

    pub fn is_leaf(&self) -> bool {
        self.inner.len() == self.depth + 1
    }

    pub fn height(&self) -> usize {
        self.inner.len()
    }
}
