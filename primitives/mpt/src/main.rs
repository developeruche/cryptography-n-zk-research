use mpt::{mpt_core::EMPTY_NODE_HASH, MerklePatriciaTrie};

fn main() {
    // Create a new trie instance
    let mut trie = MerklePatriciaTrie::new();
    println!("Initial Root Hash: {:?}", trie.root_hash());
    assert_eq!(
        trie.root_hash(),
        *EMPTY_NODE_HASH,
        "Initial root should be empty hash"
    );

    // Insert some key-value pairs
    println!("\n--- Inserting Data ---");
    trie.put(b"key1", b"value1".to_vec());
    println!("Put 'key1': 'value1'. New Root: {:?}", trie.root_hash());

    trie.put(b"key2", b"value2".to_vec());
    println!("Put 'key2': 'value2'. New Root: {:?}", trie.root_hash());

    // Insert a key that shares a prefix
    trie.put(b"k", b"value_short".to_vec());
    println!("Put 'k': 'value_short'. New Root: {:?}", trie.root_hash());

    // Update an existing key
    trie.put(b"key1", b"value1_updated".to_vec());
    println!(
        "Updated 'key1': 'value1_updated'. New Root: {:?}",
        trie.root_hash()
    );

    // Insert keys demonstrating branch creation and extension splitting
    println!("\n--- Testing Prefixes ---");
    trie.put(b"dog", b"puppy".to_vec());
    let r_dog = trie.root_hash();
    println!("Put 'dog': 'puppy'. Root: {:?}", r_dog);

    trie.put(b"doge", b"coin".to_vec()); // Should create a branch under 'dog'
    let r_doge = trie.root_hash();
    println!("Put 'doge': 'coin'. Root: {:?}", r_doge);
    assert_ne!(
        r_dog, r_doge,
        "Root hash should change after inserting 'doge'"
    );

    trie.put(b"do", b"verb".to_vec()); // Should create branch at 'do', possibly involving extension
    let r_do = trie.root_hash();
    println!("Put 'do': 'verb'. Root: {:?}", r_do);
    assert_ne!(r_doge, r_do, "Root hash should change after inserting 'do'");

    // Print the final structure for visualization
    trie.print_structure();

    // --- Retrieve values ---
    println!("\n--- Retrieving Values ---");
    println!(
        "Get 'key1': {:?}",
        trie.get(b"key1")
            .map(|v| String::from_utf8_lossy(&v).into_owned())
    );
    println!(
        "Get 'key2': {:?}",
        trie.get(b"key2")
            .map(|v| String::from_utf8_lossy(&v).into_owned())
    );
    println!(
        "Get 'k': {:?}",
        trie.get(b"k")
            .map(|v| String::from_utf8_lossy(&v).into_owned())
    );
    println!(
        "Get 'dog': {:?}",
        trie.get(b"dog")
            .map(|v| String::from_utf8_lossy(&v).into_owned())
    );
    println!(
        "Get 'doge': {:?}",
        trie.get(b"doge")
            .map(|v| String::from_utf8_lossy(&v).into_owned())
    );
    println!(
        "Get 'do': {:?}",
        trie.get(b"do")
            .map(|v| String::from_utf8_lossy(&v).into_owned())
    );
    println!("Get 'nonexistent': {:?}", trie.get(b"nonexistent"));

    // --- Verify retrieved values ---
    assert_eq!(trie.get(b"key1").unwrap(), b"value1_updated");
    assert_eq!(trie.get(b"key2").unwrap(), b"value2");
    assert_eq!(trie.get(b"k").unwrap(), b"value_short");
    assert_eq!(trie.get(b"dog").unwrap(), b"puppy");
    assert_eq!(trie.get(b"doge").unwrap(), b"coin");
    assert_eq!(trie.get(b"do").unwrap(), b"verb");
    assert!(trie.get(b"nonexistent").is_none());

    println!("\nAll assertions passed.");
}
