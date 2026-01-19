use rs_merkle::{MerkleTree as RSMerkleTree, algorithms::Sha256};
use sha2::{Digest, Sha256 as RustSha256};

#[test]
fn test_rs_merkle_odd_nodes() {
    let leaves: Vec<[u8; 32]> = vec![
        [1u8; 32],
        [2u8; 32],
        [3u8; 32],
    ];

    let tree = RSMerkleTree::<Sha256>::from_leaves(&leaves);
    let root = tree.root();
    
    // Calculate manually
    // H(a,b)
    let mut h = RustSha256::new();
    h.update(&leaves[0]);
    h.update(&leaves[1]);
    let h_ab = h.finalize();

    // H(c,c) - duplication
    let mut h = RustSha256::new();
    h.update(&leaves[2]);
    h.update(&leaves[2]);
    let h_cc = h.finalize();

    // Root (dup) = H(H(a,b), H(c,c))
    let mut h = RustSha256::new();
    h.update(&h_ab);
    h.update(&h_cc);
    let root_dup = h.finalize();

    // Root (promo) = H(H(a,b), c)
    let mut h = RustSha256::new();
    h.update(&h_ab);
    h.update(&leaves[2]);
    let root_promo = h.finalize();

    println!("RS Merkle Root: {:?}", hex::encode(root.unwrap()));
    println!("Calc Root Dup: {:?}", hex::encode(root_dup));
    println!("Calc Root Promo: {:?}", hex::encode(root_promo));

    if root.unwrap().as_slice() == root_dup.as_slice() {
        println!("CONCLUSION: DUPLICATION");
    } else if root.unwrap().as_slice() == root_promo.as_slice() {
        println!("CONCLUSION: PROMOTION");
    } else {
        println!("CONCLUSION: UNKNOWN");
    }
}
