//! # Merkle Tree Library
//!
//! A professional Rust implementation of Merkle Trees, compatible with `rs_merkle`.
//! This library provides efficient tree construction, multi-proof generation,
//! and verification using custom hashers.

use std::marker::PhantomData;

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

    /// Generate a Merkle proof for the given leaf indices.
    pub fn proof(&self, indices: &[usize]) -> MerkleProof<H> {
        let mut proof_hashes = Vec::new();
        
        if self.leaves.is_empty() || indices.is_empty() {
             return MerkleProof {
                proof_hashes,
                leaves_count: self.leaves.len(),
                _phantom: PhantomData,
            };
        }

        let mut current_indices = indices.to_vec();
        current_indices.sort_unstable();
        current_indices.dedup();

        // Iterate through layers
        for layer in &self.layers[0..self.layers.len() - 1] {
            let mut next_indices = Vec::new();
            let len = layer.len();
            
            let mut i = 0;
            while i < current_indices.len() {
                let idx = current_indices[i];
                let sibling_idx = if idx % 2 == 0 { idx + 1 } else { idx - 1 };
                
                let mut has_sibling = false;

                // Check if we have the sibling in our current set
                if idx % 2 == 0 {
                    if i + 1 < current_indices.len() && current_indices[i+1] == sibling_idx {
                        has_sibling = true;
                        i += 1; 
                    }
                }
                
                // If missing sibling exists in tree, add to proof
                if !has_sibling && sibling_idx < len {
                    proof_hashes.push(layer[sibling_idx]);
                }

                next_indices.push(idx / 2);
                i += 1;
            }

            current_indices = next_indices;
        }

        MerkleProof {
            proof_hashes,
            leaves_count: self.leaves.len(),
            _phantom: PhantomData,
        }
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

impl<H: Hasher> MerkleProof<H> {
    /// Calculate the root from the proof and a subset of leaves.
    pub fn root(&self, leaf_indices: &[usize], leaf_hashes: &[H::Hash], total_leaves_count: usize) -> Option<H::Hash> {
        if leaf_indices.len() != leaf_hashes.len() || total_leaves_count != self.leaves_count || total_leaves_count == 0 {
            return None;
        }

        let mut current_hashes = leaf_hashes.to_vec();
        let mut current_indices = leaf_indices.to_vec();
        
        let mut combined: Vec<(usize, H::Hash)> = current_indices.iter().zip(current_hashes.iter()).map(|(&i, &h)| (i, h)).collect();
        combined.sort_by_key(|&(i, _)| i);
        current_indices = combined.iter().map(|&(i, _)| i).collect();
        current_hashes = combined.iter().map(|&(_, h)| h).collect();

        let mut proof_iter = self.proof_hashes.iter();
        let mut cur_len = total_leaves_count;
        
        while cur_len > 1 {
            let mut next_indices = Vec::new();
            let mut next_hashes = Vec::new();

            let mut i = 0;
            while i < current_indices.len() {
                let idx = current_indices[i];
                let hash = current_hashes[i];
                let sibling_idx = if idx % 2 == 0 { idx + 1 } else { idx - 1 };
                
                let mut sibling_hash = None;

                if i + 1 < current_indices.len() && current_indices[i+1] == sibling_idx {
                    sibling_hash = Some(current_hashes[i+1]);
                    i += 1;
                }

                if sibling_hash.is_none() {
                    if sibling_idx < cur_len {
                         if let Some(h) = proof_iter.next() {
                             sibling_hash = Some(*h);
                         } else {
                             return None; 
                         }
                    }
                }

                let parent_hash = if let Some(sh) = sibling_hash {
                    if idx % 2 == 0 {
                        HasherHelper::<H>::hash_node(&hash, &sh)
                    } else {
                        HasherHelper::<H>::hash_node(&sh, &hash)
                    }
                } else {
                    hash 
                };

                next_indices.push(idx / 2);
                next_hashes.push(parent_hash);
                
                i += 1;
            }

            current_indices = next_indices;
            current_hashes = next_hashes;
            cur_len = (cur_len + 1) / 2;
        }

        current_hashes.first().cloned()
    }

    /// Verify the proof against a known root.
    pub fn verify(&self, root: H::Hash, leaf_indices: &[usize], leaf_hashes: &[H::Hash], total_leaves_count: usize) -> bool {
        self.root(leaf_indices, leaf_hashes, total_leaves_count) == Some(root)
    }
}
