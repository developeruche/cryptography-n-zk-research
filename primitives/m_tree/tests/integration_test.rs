use m_tree::{MerkleTree, Hasher};
use rs_merkle::{MerkleTree as RSMerkleTree, algorithms::Sha256 as RSSha256};
use sha2::{Digest, Sha256};

#[derive(Clone)]
struct MySha256;

impl Hasher for MySha256 {
    type Hash = [u8; 32];

    fn hash(data: &[u8]) -> Self::Hash {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().into()
    }
}

#[test]
fn test_merkle_tree_root_compatibility() {
    let leaves: Vec<[u8; 32]> = vec![
        [1u8; 32],
        [2u8; 32],
        [3u8; 32],
        [4u8; 32],
        [5u8; 32],
    ];

    // rs_merkle
    let rs_tree = RSMerkleTree::<RSSha256>::from_leaves(&leaves);
    let rs_root = rs_tree.root().unwrap();

    // m_tree
    let my_tree = MerkleTree::<MySha256>::from_leaves(&leaves);
    let my_root = my_tree.root().unwrap();

    assert_eq!(rs_root, my_root, "Roots should match");
}

#[test]
fn test_merkle_proof_verification() {
    let leaves: Vec<[u8; 32]> = (0..7).map(|i| [i as u8; 32]).collect();

    // rs_merkle
    let rs_tree = RSMerkleTree::<RSSha256>::from_leaves(&leaves);
    let rs_root = rs_tree.root().unwrap();
    let indices = vec![1, 3, 4];
    
    // m_tree
    let my_tree = MerkleTree::<MySha256>::from_leaves(&leaves);
    let my_proof = my_tree.proof(&indices);
    
    // Verify using my proof logic
    let leaf_hashes: Vec<[u8; 32]> = indices.iter().map(|&i| leaves[i]).collect();
    let valid = my_proof.verify(rs_root, &indices, &leaf_hashes, leaves.len());
    
    assert!(valid, "Proof should be valid");
    
    // Verify root calculation matches
    let calculated_root = my_proof.root(&indices, &leaf_hashes, leaves.len()).unwrap();
    assert_eq!(calculated_root, rs_root, "Calculated root from proof should match");
}
