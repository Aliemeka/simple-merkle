/// Merkle tree implementation in Rust
///
/// This module provides a simple implementation of a Merkle tree, which is a data structure used in cryptography and blockchain applications to efficiently verify the integrity of data. The Merkle tree is built using a hash function, and it allows for efficient verification of data by comparing the root hash with the hashes of the individual data blocks.
///
///
use crate::hash_func::{hash, hash_pair};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MerkleNode {
    pub hash: String,
    pub left: Option<Box<MerkleNode>>,
    pub right: Option<Box<MerkleNode>>,
}

#[derive(Debug, Clone)]
pub struct MerkleTree {
    pub root: Option<Box<MerkleNode>>,
    pub leaf_nodes: Vec<String>,
    pub layers: Vec<Vec<String>>,
    pub blocks: Vec<(String, u64)>,
}

impl MerkleTree {
    pub fn build(blocks: Vec<(String, u64)>) -> Self {
        let leaf_nodes: Vec<String> = blocks
            .iter()
            .map(|(address, amount)| hash(&format!("{}:{}", address, amount)))
            .collect();

        let leaf_ref = Box::new(leaf_nodes.clone());
        let mut layers = vec![leaf_nodes.clone()];
        let mut current_layer = leaf_nodes;

        while current_layer.len() > 1 {
            let mut next_layer = Vec::new();
            for i in (0..current_layer.len()).step_by(2) {
                let left = &current_layer[i];
                let right = if i + 1 < current_layer.len() {
                    &current_layer[i + 1]
                } else {
                    left // If odd number of nodes, duplicate the last one
                };
                next_layer.push(hash_pair(left, right));
            }
            layers.push(next_layer.clone());
            current_layer = next_layer;
        }

        let root = if !current_layer.is_empty() {
            Some(Box::new(MerkleNode {
                hash: current_layer[0].clone(),
                left: None,
                right: None,
            }))
        } else {
            None
        };

        Self {
            root,
            leaf_nodes: *leaf_ref,
            layers,
            blocks,
        }
    }

    pub fn get_root(&self) -> Option<String> {
        self.root.as_ref().map(|node| node.hash.clone())
    }

    pub fn proof_block(&self, address: &str, amount: u64) -> Option<Vec<String>> {
        let target_hash = hash(&format!("{}:{}", address, amount));
        let mut proof = Vec::new();

        if let Some(index) = self.leaf_nodes.iter().position(|h| h == &target_hash) {
            for layer in &self.layers {
                let sibling_index = if index % 2 == 0 { index + 1 } else { index - 1 };
                if sibling_index < layer.len() {
                    proof.push(layer[sibling_index].clone());
                }
            }
            Some(proof)
        } else {
            None
        }
    }

    pub fn get_depth(&self) -> usize {
        self.layers.len()
    }

    pub fn verify_proof(&self, address: &str, amount: u64, proof: Vec<String>) -> bool {
        let mut computed_hash = hash(&format!("{}:{}", address, amount));

        for sibling_hash in proof {
            computed_hash = hash_pair(&computed_hash, &sibling_hash);
        }

        self.get_root() == Some(computed_hash)
    }

    pub fn print_tree(&self) {
        for (i, layer) in self.layers.iter().enumerate() {
            let index = i + 1;
            println!("Layer {index}: {layer:?}");
        }
    }
}
