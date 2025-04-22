use crate::mpt_core::{Nibble, NibblePath};
use primitive_types::H256;
use serde::Serialize;
use sha3::{Digest, Keccak256};

/// Converts a byte slice into a vector of nibbles (4-bit values).
/// Example: b"ab" -> [6, 1, 6, 2]
pub fn bytes_to_nibbles(bytes: &[u8]) -> NibblePath {
    let mut nibbles = Vec::with_capacity(bytes.len() * 2);
    for byte in bytes {
        nibbles.push(byte >> 4); // High nibble
        nibbles.push(byte & 0x0F); // Low nibble
    }
    nibbles
}

/// Converts a nibble path back into bytes.
/// Panics if the nibble path has an odd length.
/// Example: [6, 1, 6, 2] -> b"ab"
pub fn nibbles_to_bytes(nibbles: &[Nibble]) -> Vec<u8> {
    assert!(
        nibbles.len() % 2 == 0,
        "Nibble path must have even length to convert back to bytes cleanly"
    );
    let mut bytes = Vec::with_capacity(nibbles.len() / 2);
    for chunk in nibbles.chunks_exact(2) {
        bytes.push((chunk[0] << 4) | chunk[1]);
    }
    bytes
}

/// Calculates the Keccak-256 hash of a serializable node using bincode.
/// NOTE: This uses bincode, NOT RLP encoding like Ethereum. Hashes will differ.
pub fn calculate_hash_internal<T: Serialize>(node: &T) -> H256 {
    let serialized = bincode::serialize(node).expect("Failed to serialize node");
    // Ethereum's actual empty node hash is Keccak256(RLP([])).
    // Our Node::Empty serializes to non-empty bytes via bincode,
    // so we hash that representation directly for consistency within this simplified model.
    H256::from_slice(Keccak256::digest(&serialized).as_slice())
}

/// Finds the length of the common prefix between two nibble paths.
pub fn common_prefix_len(a: &[Nibble], b: &[Nibble]) -> usize {
    a.iter()
        .zip(b.iter())
        .take_while(|(na, nb)| na == nb)
        .count()
}
