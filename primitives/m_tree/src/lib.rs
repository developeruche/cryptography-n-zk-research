//! # Merkle Tree Library
//!
//! A professional Rust implementation of Merkle Trees, compatible with `rs_merkle`.
//! This library provides efficient tree construction, multi-proof generation,
//! and verification using custom hashers.

use std::marker::PhantomData;
pub use sha2::{Digest, Sha256};

/// Trait for hashers used in the Merkle Tree.
/// Implement this to define custom hashing logic.
pub trait Hasher {
    /// The type of the hash output.
    type Hash: AsRef<[u8]> + Clone + Eq + Ord + Copy + Default + Sync + Send;

    /// Hash a byte slice.
    fn hash(data: &[u8]) -> Self::Hash;
}

/// Merkle Tree implementation.
#[derive(Debug, Clone)]
pub struct MerkleTree<H: Hasher> {
    leaves: Vec<H::Hash>,
    layers: Vec<Vec<H::Hash>>,
    _phantom: PhantomData<H>,
}

impl<H: Hasher> MerkleTree<H> {
    /// Create a new Merkle Tree from a list of leaves.
    pub fn from_leaves(leaves: &[H::Hash]) -> Self {
        let layers = Self::build_layers(leaves);
        Self {
            leaves: leaves.to_vec(),
            layers,
            _phantom: PhantomData,
        }
    }

    /// Get the Merkle Root of the tree.
    pub fn root(&self) -> Option<H::Hash> {
        self.layers.last().and_then(|layer| layer.first().cloned())
    }

    /// Get the Merkle Root of the tree as a hex string.
    pub fn root_to_hex(&self) -> Option<String> {
        self.root().map(|r| hex::encode(r))
    }

    /// Generate a Merkle proof for the given leaf index.
    pub fn proof(&self, index: usize) -> Option<MerkleProof<H>> {
        if index >= self.leaves.len() {
            return None;
        }

        let mut proof_hashes = Vec::new();
        let mut current_index = index;

        // Iterate through layers up to the root
        for layer in &self.layers[0..self.layers.len() - 1] {
            let len = layer.len();
            let sibling_index = if current_index % 2 == 0 {
                current_index + 1
            } else {
                current_index - 1
            };

            if sibling_index < len {
                proof_hashes.push(layer[sibling_index]);
            }

            current_index /= 2;
        }

        Some(MerkleProof {
            proof_hashes,
            leaves_count: self.leaves.len(),
            _phantom: PhantomData,
        })
    }

    fn build_layers(leaves: &[H::Hash]) -> Vec<Vec<H::Hash>> {
        if leaves.is_empty() {
            return vec![];
        }

        let mut layers = vec![leaves.to_vec()];
        let mut current = leaves.to_vec();

        while current.len() > 1 {
            let mut next = Vec::new();
            for chunk in current.chunks(2) {
                if chunk.len() == 2 {
                    next.push(HasherHelper::<H>::hash_node(&chunk[0], &chunk[1]));
                } else {
                    // Odd node promotion strategy
                    next.push(chunk[0]);
                }
            }
            layers.push(next.clone());
            current = next;
        }

        layers
    }
}

struct HasherHelper<H: Hasher>(PhantomData<H>);

impl<H: Hasher> HasherHelper<H> {
    fn hash_node(left: &H::Hash, right: &H::Hash) -> H::Hash {
        let mut concat = Vec::new();
        concat.extend_from_slice(left.as_ref());
        concat.extend_from_slice(right.as_ref());
        H::hash(&concat)
    }
}

/// A Merkle Proof.
#[derive(Debug, Clone)]
pub struct MerkleProof<H: Hasher> {
    proof_hashes: Vec<H::Hash>,
    leaves_count: usize,
    _phantom: PhantomData<H>,
}

#[derive(Clone)]
pub struct DefaultHasher;

impl Hasher for DefaultHasher {
    type Hash = [u8; 32];

    fn hash(data: &[u8]) -> Self::Hash {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().into()
    }
}

pub type DefaultMerkleTree = MerkleTree<DefaultHasher>;

impl<H: Hasher> MerkleProof<H> {
    /// Calculate the root from the proof and a leaf.
    pub fn root(&self, index: usize, leaf_hash: H::Hash, total_leaves_count: usize) -> Option<H::Hash> {
        if index >= total_leaves_count || total_leaves_count != self.leaves_count {
            return None;
        }

        let mut current_hash = leaf_hash;
        let mut current_index = index;
        let mut proof_iter = self.proof_hashes.iter();
        let mut cur_len = total_leaves_count;

        while cur_len > 1 {
            let sibling_index = if current_index % 2 == 0 {
                current_index + 1
            } else {
                current_index - 1
            };

            if sibling_index < cur_len {
                let sibling_hash = proof_iter.next()?;
                if current_index % 2 == 0 {
                    current_hash = HasherHelper::<H>::hash_node(&current_hash, sibling_hash);
                } else {
                    current_hash = HasherHelper::<H>::hash_node(sibling_hash, &current_hash);
                }
            } else {
                // If no sibling (odd node promotion), current_hash bubbles up unchanged
            }

            current_index /= 2;
            cur_len = (cur_len + 1) / 2;
        }

        Some(current_hash)
    }

    /// Verify the proof against a known root.
    pub fn verify(&self, root: H::Hash, index: usize, leaf_hash: H::Hash, total_leaves_count: usize) -> bool {
        self.root(index, leaf_hash, total_leaves_count) == Some(root)
    }
}
