use m_tree::{DefaultHasher, MerkleProof};
use winternitz_ots::sha256_plus::{WotsPublicKey, WotsSignature};

#[derive(Debug)]
pub struct XmssPublicData {
    /// This is the max amount of signatures that can be created
    pub max_signatures: u64,
    /// This is the root of the merkle tree (the main public key)
    pub root: [u8; 32],
}

#[derive(Debug)]
pub struct XmssSignature {
    /// This is the authentication path
    pub auth_path: MerkleProof<DefaultHasher>,
    /// This is the WOTS signature
    pub wots_signature: WotsSignature,
    /// This is the public key
    pub public_key: WotsPublicKey,
    /// This is the counter
    pub counter: u32,
    /// This is the index of the leaf in the merkle tree
    pub index: u64,
}
