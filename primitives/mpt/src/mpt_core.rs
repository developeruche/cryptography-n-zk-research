use crate::utils::calculate_hash_internal;
use lazy_static::lazy_static;
use primitive_types::H256;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

pub type Nibble = u8; // 0-15
pub type NibblePath = Vec<Nibble>;
/// Shared, mutable, thread-safe storage (Hash -> Serialized Node)
pub type Db = Arc<RwLock<HashMap<H256, Vec<u8>>>>;

// Represents the hash of an empty node/trie
lazy_static! {
    // In Ethereum, this is Keccak256(RLP([])).
    // We simulate by hashing our bincode representation of Node::Empty.
    // Note: This hash WILL NOT match the actual Ethereum empty root hash.
    pub static ref EMPTY_NODE_HASH: H256 = calculate_hash_internal(&Node::Empty);
}

/// Represents the different types of nodes in the Merkle Patricia Trie.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Node {
    /// Represents an empty slot or the end of a path without a value.
    Empty,
    /// Represents the end of a path that stores a value.
    Leaf { path: NibblePath, value: Vec<u8> },
    /// Represents a node that shares a common path prefix before diverging.
    Extension {
        path: NibblePath,
        next_node_hash: H256,
    },
    /// Represents a node where the path diverges into multiple possibilities (up to 16 children).
    /// Can optionally store a value if a key terminates at this branch.
    Branch {
        children: [Option<H256>; 16],
        value: Option<Vec<u8>>,
    },
}
