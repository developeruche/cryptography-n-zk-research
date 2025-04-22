use mpt_core::{Db, Nibble, Node, EMPTY_NODE_HASH};
use primitive_types::H256;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use utils::{bytes_to_nibbles, calculate_hash_internal, common_prefix_len};
pub mod mpt_core;
pub mod utils;

/// The main structure for the Merkle Patricia Trie.
#[derive(Debug)]
pub struct MerklePatriciaTrie {
    root_hash: H256,
    db: Db, // In-memory Key-Value store (Hash -> Serialized Node)
}

impl MerklePatriciaTrie {
    /// Creates a new, empty Merkle Patricia Trie.
    pub fn new() -> Self {
        let db = Arc::new(RwLock::new(HashMap::new()));
        // Store the representation of the Empty node under its hash initially.
        // This ensures that when we try to fetch EMPTY_NODE_HASH, we find a valid node.
        let empty_node_serialized =
            bincode::serialize(&Node::Empty).expect("Failed to serialize empty node");
        db.write()
            .unwrap()
            .insert(*EMPTY_NODE_HASH, empty_node_serialized);

        MerklePatriciaTrie {
            root_hash: *EMPTY_NODE_HASH,
            db,
        }
    }

    /// Gets the current root hash of the trie. This hash represents the entire state.
    pub fn root_hash(&self) -> H256 {
        self.root_hash
    }

    /// Retrieves and deserializes a node from the DB using its hash.
    /// Returns None if the hash is not found or deserialization fails.
    fn get_node(&self, hash: &H256) -> Option<Node> {
        let db_read = self.db.read().unwrap();
        db_read.get(hash).and_then(|serialized_node| {
            // Attempt to deserialize; return None on error
            bincode::deserialize(serialized_node).ok()
        })
    }

    /// Serializes and stores a node in the DB, returning its hash.
    /// If the node is Node::Empty, returns the predefined EMPTY_NODE_HASH without storing.
    /// Otherwise, calculates the hash, serializes, and stores if not already present.
    fn store_node(&self, node: &Node) -> H256 {
        // Optimization: Don't store multiple copies of the empty node representation.
        if let Node::Empty = node {
            return *EMPTY_NODE_HASH;
        }

        let hash = calculate_hash_internal(node);
        let serialized_node =
            bincode::serialize(node).expect("Failed to serialize node for storage");

        let mut db_write = self.db.write().unwrap();
        // Use entry API for efficiency and to handle potential concurrent writes correctly
        db_write.entry(hash).or_insert(serialized_node);
        hash
    }

    /// Retrieves the value associated with a key from the trie.
    /// Returns `Some(value)` if the key exists, `None` otherwise.
    pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        let nibbles = bytes_to_nibbles(key);
        // Start the recursive search from the root node
        self.recursive_get(&self.root_hash, &nibbles)
    }

    /// Internal recursive function to traverse the trie and find a value.
    fn recursive_get(&self, node_hash: &H256, path: &[Nibble]) -> Option<Vec<u8>> {
        // Fetch the node corresponding to the current hash
        let node = self.get_node(node_hash)?; // If node not in DB, path doesn't exist

        match node {
            Node::Empty => None, // Reached an empty slot, key not found
            Node::Leaf {
                path: leaf_path,
                value,
            } => {
                // Check if the remaining path matches the leaf's path exactly
                if path == leaf_path.as_slice() {
                    Some(value) // Found the value
                } else {
                    None // Paths don't match
                }
            }
            Node::Extension {
                path: ext_path,
                next_node_hash,
            } => {
                // Check if the remaining path starts with the extension's path
                let prefix_len = common_prefix_len(path, &ext_path);
                if prefix_len == ext_path.len() {
                    // Full extension path matches, continue search deeper
                    // Recurse with the next node hash and the rest of the path
                    self.recursive_get(&next_node_hash, &path[prefix_len..])
                } else {
                    None // Path diverges within the extension path
                }
            }
            Node::Branch { children, value } => {
                if path.is_empty() {
                    // The search path ends exactly at this branch node. Return its value (if any).
                    value
                } else {
                    // Path continues. Take the next nibble as the index into children.
                    let nibble_index = path[0] as usize;
                    match children[nibble_index] {
                        // If a child exists at that index, recurse down that path
                        Some(child_hash) => self.recursive_get(&child_hash, &path[1..]),
                        // No child exists for this nibble, key not found
                        None => None,
                    }
                }
            }
        }
    }

    /// Inserts or updates a key-value pair in the trie.
    /// Updates the trie's root hash.
    /// Note: Inserting an empty value is currently disallowed as delete isn't implemented.
    pub fn put(&mut self, key: &[u8], value: Vec<u8>) {
        if value.is_empty() {
            // Ethereum MPT treats insertion of empty value as deletion.
            // Since delete is complex and not implemented here, we disallow it.
            eprintln!("Warning: Inserting empty values (which implies deletion) is not supported in this simplified implementation. Key: {:?}", hex::encode(key));
            return;
            // If delete were implemented: self.delete(key); return;
        }
        let nibbles = bytes_to_nibbles(key);
        // Start the recursive put operation from the current root
        // Handle potential errors during insertion (e.g., node not found unexpectedly)
        match self.recursive_put(self.root_hash, &nibbles, value) {
            Ok(new_root_hash) => {
                // Update the trie's root hash if the operation was successful
                self.root_hash = new_root_hash;
            }
            Err(e) => {
                // In a real application, propagate this error properly
                eprintln!("Error during put operation: {}", e);
            }
        }
    }

    /// Recursive helper for inserting or updating a key-value pair.
    /// Traverses the trie, modifying nodes as needed, and returns the
    /// hash of the updated node at the current level.
    /// Returns `Result<H256, String>` where Ok contains the new hash for this level,
    /// and Err contains an error message.
    fn recursive_put(
        &mut self,
        node_hash: H256,
        path: &[Nibble],
        value: Vec<u8>,
    ) -> Result<H256, String> {
        // Fetch the current node to be modified or traversed
        let current_node = self
            .get_node(&node_hash)
            .ok_or_else(|| format!("Node not found for hash: {:?}", node_hash))?;

        // The core logic: decide how to modify the trie based on the current node type
        let updated_node_hash = match current_node {
            // Case 1: Inserting into an empty slot
            Node::Empty => {
                // Create a new Leaf node containing the remaining path and value
                let new_leaf = Node::Leaf {
                    path: path.to_vec(),
                    value,
                };
                // Store the new leaf and return its hash
                Ok(self.store_node(&new_leaf))
            }

            // Case 2: Current node is a Leaf
            Node::Leaf {
                path: leaf_path,
                value: leaf_value,
            } => {
                let prefix_len = common_prefix_len(path, &leaf_path);

                // Subcase 2a: The new key matches the existing leaf's key exactly
                if prefix_len == leaf_path.len() && prefix_len == path.len() {
                    // If values are the same, no change needed, return original hash
                    if leaf_value == value {
                        Ok(node_hash)
                    } else {
                        // Values differ, create an updated leaf with the new value
                        let updated_leaf = Node::Leaf {
                            path: leaf_path,
                            value,
                        };
                        Ok(self.store_node(&updated_leaf))
                    }
                } else {
                    // Subcase 2b: Paths diverge or one is a prefix of the other.
                    // Need to replace the Leaf with a Branch node.
                    let mut new_branch_children = [None; 16];
                    let mut new_branch_value = None;

                    // Insert the original leaf's value into the new branch
                    if prefix_len == leaf_path.len() {
                        // Original leaf's path ends here, set branch value
                        new_branch_value = Some(leaf_value);
                    } else {
                        // Original leaf continues, create a new sub-leaf/node
                        let leaf_nibble = leaf_path[prefix_len] as usize;
                        let remaining_leaf_path = leaf_path[prefix_len + 1..].to_vec();
                        // Create node for remainder of original leaf path
                        let node_for_old_leaf = Node::Leaf {
                            path: remaining_leaf_path,
                            value: leaf_value,
                        };
                        new_branch_children[leaf_nibble] =
                            Some(self.store_node(&node_for_old_leaf));
                    }

                    // Insert the new key/value into the new branch
                    if prefix_len == path.len() {
                        // New key's path ends here, set branch value
                        new_branch_value = Some(value);
                    } else {
                        // New key continues, create a new sub-leaf/node
                        let new_nibble = path[prefix_len] as usize;
                        let remaining_new_path = path[prefix_len + 1..].to_vec();
                        // Create node for remainder of new key's path
                        let node_for_new_value = Node::Leaf {
                            path: remaining_new_path,
                            value,
                        };
                        new_branch_children[new_nibble] =
                            Some(self.store_node(&node_for_new_value));
                    }

                    // Create the actual branch node
                    let new_branch = Node::Branch {
                        children: new_branch_children,
                        value: new_branch_value,
                    };

                    // If the common prefix was non-empty, we need an Extension node pointing to the Branch
                    if prefix_len > 0 {
                        let branch_hash = self.store_node(&new_branch);
                        let common_prefix_path = path[..prefix_len].to_vec(); // Path shared by both
                        let extension_node = Node::Extension {
                            path: common_prefix_path,
                            next_node_hash: branch_hash,
                        };
                        Ok(self.store_node(&extension_node))
                    } else {
                        // Common prefix was empty, the Branch is the new node at this level
                        Ok(self.store_node(&new_branch))
                    }
                }
            }

            // Case 3: Current node is an Extension
            Node::Extension {
                path: ext_path,
                next_node_hash,
            } => {
                let prefix_len = common_prefix_len(path, &ext_path);

                // Subcase 3a: The new key's path matches the extension path fully or partially
                if prefix_len == ext_path.len() {
                    // Full extension path matches, recurse deeper into the trie
                    let remaining_path = &path[prefix_len..];
                    let updated_next_hash =
                        self.recursive_put(next_node_hash, remaining_path, value)?;
                    // If the recursive call didn't change the child hash, no change here either
                    if updated_next_hash == next_node_hash {
                        Ok(node_hash)
                    } else {
                        // Child hash changed, create a new Extension node pointing to the updated child
                        let updated_ext = Node::Extension {
                            path: ext_path,
                            next_node_hash: updated_next_hash,
                        };
                        Ok(self.store_node(&updated_ext))
                    }
                } else {
                    // Subcase 3b: Paths diverge within the extension path.
                    // Need to split the extension and create a new Branch node.
                    let common_prefix_path = ext_path[..prefix_len].to_vec();
                    let ext_remaining_path = ext_path[prefix_len + 1..].to_vec(); // Path remaining for original extension
                    let path_remaining = path[prefix_len + 1..].to_vec(); // Path remaining for new value

                    let ext_diverge_nibble = ext_path[prefix_len] as usize; // Nibble where original extension continues
                    let path_diverge_nibble = path[prefix_len] as usize; // Nibble where new path continues

                    // Create the new branch node that resolves the divergence
                    let mut branch_children = [None; 16];
                    let branch_value = None; // Branch itself doesn't hold a value here

                    // 1. Add node representing the original extension's continuation
                    // If the remaining extension path is empty, the original `next_node_hash` becomes a direct child.
                    // Otherwise, create an intermediate node (Leaf or Extension) for the remaining path.
                    let existing_node_repr_hash = if ext_remaining_path.is_empty() {
                        next_node_hash // Original target becomes direct child
                    } else {
                        // Need a node for the rest of the original extension's path
                        // This could technically be a Leaf if the original next_node was a value-holding branch/leaf
                        // For simplicity here, assume it's usually another Extension or Branch, represented by Extension node.
                        // A more robust impl might fetch next_node_hash to create Leaf if appropriate.
                        let node_for_old_ext = Node::Extension {
                            path: ext_remaining_path,
                            next_node_hash,
                        };
                        self.store_node(&node_for_old_ext)
                    };
                    branch_children[ext_diverge_nibble] = Some(existing_node_repr_hash);

                    // 2. Add node representing the new value being inserted
                    let node_for_new_value = Node::Leaf {
                        path: path_remaining,
                        value,
                    };
                    branch_children[path_diverge_nibble] =
                        Some(self.store_node(&node_for_new_value));

                    // Store the new branch node
                    let new_branch = Node::Branch {
                        children: branch_children,
                        value: branch_value,
                    };
                    let new_branch_hash = self.store_node(&new_branch);

                    // If the common prefix before divergence was non-empty, create an Extension pointing to the branch
                    if prefix_len > 0 {
                        let new_root_ext = Node::Extension {
                            path: common_prefix_path,
                            next_node_hash: new_branch_hash,
                        };
                        Ok(self.store_node(&new_root_ext))
                    } else {
                        // Common prefix was empty, the branch is the new node at this level
                        Ok(new_branch_hash)
                    }
                }
            }

            // Case 4: Current node is a Branch
            Node::Branch {
                children: mut branch_children,
                value: branch_value,
            } => {
                // Subcase 4a: The path ends exactly at this branch node
                if path.is_empty() {
                    // Update the branch's value
                    if branch_value.as_deref() == Some(value.as_slice()) {
                        Ok(node_hash) // Value is the same, no change
                    } else {
                        // Value differs, create updated branch with new value
                        let updated_branch = Node::Branch {
                            children: branch_children,
                            value: Some(value),
                        };
                        Ok(self.store_node(&updated_branch))
                    }
                } else {
                    // Subcase 4b: Path continues down one of the children
                    let nibble_index = path[0] as usize;
                    // Get the hash of the child, or the empty hash if no child exists
                    let child_hash = branch_children[nibble_index].unwrap_or(*EMPTY_NODE_HASH);

                    // Recursively call put on the child node
                    let remaining_path = &path[1..];
                    let new_child_hash = self.recursive_put(child_hash, remaining_path, value)?;

                    // If the child's hash didn't change, the branch doesn't change
                    if branch_children[nibble_index] == Some(new_child_hash) {
                        Ok(node_hash)
                    } else {
                        // Child hash changed, update the branch's children array
                        // Normalize: Store None if the child became empty, otherwise store Some(hash)
                        branch_children[nibble_index] = if new_child_hash == *EMPTY_NODE_HASH {
                            None
                        } else {
                            Some(new_child_hash)
                        };
                        // Create the updated branch node
                        let updated_branch = Node::Branch {
                            children: branch_children,
                            value: branch_value,
                        };
                        Ok(self.store_node(&updated_branch))
                    }
                }
            }
        };

        updated_node_hash // Return the hash of the node at this level (potentially updated)
    }

    /// Prints the structure of the trie starting from the root (for debugging).
    /// Helps visualize the internal node layout.
    pub fn print_structure(&self) {
        println!("\n--- Trie Structure ---");
        println!("Root Hash: {:?}", self.root_hash);
        println!("DB Size: {} nodes", self.db.read().unwrap().len());
        self.recursive_print(&self.root_hash, 0);
        println!("----------------------");
    }

    /// Recursive helper for printing the trie structure.
    fn recursive_print(&self, node_hash: &H256, indent_level: usize) {
        let indent = "| ".repeat(indent_level);
        // Fetch the node, handle case where node might be missing (shouldn't happen in consistent trie)
        let node = match self.get_node(node_hash) {
            Some(n) => n,
            None => {
                // Print hash in short form for readability
                let short_hash = hex::encode(&node_hash.0[..4]);
                println!("{}Error: Node not found for hash ...{}", indent, short_hash);
                return;
            }
        };
        // Print hash in short form for readability
        let short_hash = hex::encode(&node_hash.0[..4]);

        match node {
            Node::Empty => {
                // Should ideally not be printed directly if EMPTY_NODE_HASH is handled correctly,
                // but useful for debugging if it appears unexpectedly.
                println!("{}[Empty Node] @ ...{}", indent, short_hash);
            }
            Node::Leaf { path, value } => {
                println!(
                    "{}Leaf(Path: {:?}, Val: '{}') @ ...{}",
                    indent,
                    path,
                    String::from_utf8_lossy(&value),
                    short_hash
                );
            }
            Node::Extension {
                path,
                next_node_hash,
            } => {
                println!("{}Extension(Path: {:?}) @ ...{}", indent, path, short_hash);
                // Recursively print the node pointed to by the extension
                self.recursive_print(&next_node_hash, indent_level + 1);
            }
            Node::Branch { children, value } => {
                // Display branch value if present
                let value_str = value.map_or("None".to_string(), |v| {
                    format!("'{}'", String::from_utf8_lossy(&v))
                });
                println!("{}Branch(Val: {}) @ ...{}", indent, value_str, short_hash);
                // Iterate through children and print non-empty ones
                for (i, child_opt) in children.iter().enumerate() {
                    if let Some(child_hash) = child_opt {
                        // Print the index (nibble) leading to the child
                        println!("{}{}: -> Child", indent, i);
                        // Recursively print the child node
                        self.recursive_print(child_hash, indent_level + 1);
                    }
                }
            }
        }
    }
}
