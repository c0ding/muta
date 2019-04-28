use static_merkle_tree::Tree;

use core_types::{Hash, Receipt};

#[derive(Debug, Clone)]
pub struct ProofNode {
    pub is_right: bool,
    pub hash:     Hash,
}

pub struct Merkle {
    tree: Tree<Hash>,
}

impl Merkle {
    pub fn from_hashes(hashes: Vec<Hash>) -> Self {
        let tree = Tree::from_hashes(hashes, merge);
        Merkle { tree }
    }

    pub fn from_receipts(receipts: &[Receipt]) -> Self {
        let hashes = receipts.iter().map(Receipt::hash).collect();
        Merkle::from_hashes(hashes)
    }

    pub fn get_root_hash(&self) -> Hash {
        match self.tree.get_root_hash() {
            Some(hash) => hash.clone(),
            None => Merkle::empty_root(),
        }
    }

    pub fn get_proof_by_input_index(&self, input_index: usize) -> Option<Vec<ProofNode>> {
        self.tree
            .get_proof_by_input_index(input_index)
            .map(|proof| {
                proof
                    .0
                    .into_iter()
                    .map(|node| ProofNode {
                        is_right: node.is_right,
                        hash:     node.hash.clone(),
                    })
                    .collect()
            })
    }

    pub fn empty_root() -> Hash {
        Hash::digest(&rlp::NULL_RLP)
    }
}

fn merge(left: &Hash, right: &Hash) -> Hash {
    let left = left.as_bytes();
    let right = right.as_bytes();

    let mut root = Vec::with_capacity(left.len() + right.len());
    root.extend_from_slice(left);
    root.extend_from_slice(right);
    Hash::digest(&root)
}