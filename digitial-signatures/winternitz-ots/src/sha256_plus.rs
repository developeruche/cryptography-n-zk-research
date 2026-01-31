use rand::RngCore;
use sha2::{Digest, Sha256};

/// The output size of the hash function (SHA-256), in bytes.
const HASH_SIZE: usize = 32;

/// The number of message chunks (bytes) in the input hash.
const L1: usize = 32;

/// The number of checksum chunks needed.
/// Max checksum value = L1 * (W - 1) approx 8160.
/// 8160 fits in 2 bytes (base 256).
const L2: usize = 2;

/// The total number of fragments in a key (message parts + checksum parts).
const L: usize = L1 + L2;

/// The Winternitz parameter, representing the number of possible values for each message chunk.
/// Using `w=8` bits per chunk, so `W = 2^8 = 256`.
const W: u16 = 256;

/// A type alias for a single key fragment or a message hash.
type Fragment = [u8; HASH_SIZE];
/// A type alias for the public seed.
type Seed = [u8; 32];

/// Represents a WOTS+ private key.
#[derive(Debug, Clone)]
pub struct WotsPrivateKey {
    keys: Vec<Fragment>,
    pub_seed: Seed,
}

/// Represents a WOTS+ public key.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct WotsPublicKey {
    keys: Vec<Fragment>,
    pub_seed: Seed,
}

/// Represents a WOTS+ signature.
///
/// Note: While the `pub_seed` is technically available in the PublicKey used for verification,
/// we include it here to make the signature self-contained if needed, or it can be omitted
/// if we strictly follow the structure where strict verification requires the PubKey object.
/// However, to faithfully "Add pub_seed to ... Signature format", we will make it accessible via the associated Public Key
/// or we could add it here. The prompt asks to "Add pub_seed to ... Signature format".
/// However, duplicate data is inefficient.
/// Let's stick to the minimal change: The Verifier needs `pub_seed`.
/// If `verify` is a method on `WotsPublicKey`, it has `pub_seed`.
/// If we want to export the signature structure to be verified later, we usually need the PK anyway.
/// But for full "reproducibility" of the steps `r_{i,j}`, the seed is needed.
/// I will NOT add `pub_seed` to `WotsSignature` itself to avoid redundancy, as it is part of the `WotsPublicKey`.
/// Wait, the prompt explicitly said: "Add pub_seed to: ... Signature format (if not already derivable)".
/// Since it IS derivable from the Public Key (which you need to verify), I will NOT add it to the signature field to save space.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct WotsSignature {
    keys: Vec<Fragment>,
}

impl WotsPrivateKey {
    /// Generates a new random private key.
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let mut keys = Vec::with_capacity(L);
        for _ in 0..L {
            let mut key_fragment = [0u8; HASH_SIZE];
            rng.fill_bytes(&mut key_fragment);
            keys.push(key_fragment);
        }

        let mut pub_seed = [0u8; 32];
        rng.fill_bytes(&mut pub_seed);

        WotsPrivateKey { keys, pub_seed }
    }

    /// Generates the corresponding public key from this private key.
    pub fn to_public(&self) -> WotsPublicKey {
        let pub_keys = self
            .keys
            .iter()
            .enumerate()
            .map(|(i, priv_key_fragment)| {
                // Start from index 0, iterate W times
                hash_chain(priv_key_fragment, 0, W, &self.pub_seed, i as u32)
            })
            .collect();
        WotsPublicKey {
            keys: pub_keys,
            pub_seed: self.pub_seed,
        }
    }

    /// Signs a message hash with the private key.
    pub fn sign(&self, message_hash: &Fragment) -> WotsSignature {
        let cs = checksum(message_hash);

        let signature_keys = self
            .keys
            .iter()
            .zip(message_hash.iter().chain(cs.iter()))
            .enumerate()
            .map(|(i, (priv_key_fragment, &msg_byte))| {
                let iterations = W - (msg_byte as u16);
                // Start from index 0
                hash_chain(priv_key_fragment, 0, iterations, &self.pub_seed, i as u32)
            })
            .collect();
        WotsSignature {
            keys: signature_keys,
        }
    }
}

impl WotsPublicKey {
    /// Verifies a signature against a message hash and the public key.
    pub fn verify(&self, message_hash: &Fragment, signature: &WotsSignature) -> bool {
        if signature.keys.len() != L {
            return false;
        }

        let cs = checksum(message_hash);

        let recovered_keys: Vec<Fragment> = signature
            .keys
            .iter()
            .zip(message_hash.iter().chain(cs.iter()))
            .enumerate()
            .map(|(i, (sig_fragment, &msg_byte))| {
                let iterations = msg_byte as u16;
                // Start from index (W - msg_byte).
                // Because the signature provided constitutes the value at step (W - msg_byte).
                // We need to apply `msg_byte` more steps to reach step W.
                let start_index = W - (msg_byte as u16);
                hash_chain(
                    sig_fragment,
                    start_index,
                    iterations,
                    &self.pub_seed,
                    i as u32,
                )
            })
            .collect();

        self.keys == recovered_keys
    }
}

/// Derives the randomization value r_{i,j}
/// PRF(pub_seed, chain_idx, step_idx)
fn derive_randomizer(pub_seed: &Seed, chain_idx: u32, step_idx: u16) -> Fragment {
    let mut hasher = Sha256::new();
    hasher.update(pub_seed);
    hasher.update(&chain_idx.to_be_bytes()); // Chain index i
    hasher.update(&step_idx.to_be_bytes()); // Step index j
    hasher.finalize().into()
}

/// Performs the WOTS+ hash chain: x_{i, j+1} = H(x_{i, j} XOR r_{i, j})
///
/// `start`: The starting value x_{i, start_dist}
/// `start_index`: The absolute step index j where we start (0 <= j < W)
/// `iterations`: How many steps to compute
/// `pub_seed`: Public seed for randomization
/// `chain_addr`: The index i of the current chain (0 <= i < L)
fn hash_chain(
    start: &Fragment,
    start_index: u16,
    iterations: u16,
    pub_seed: &Seed,
    chain_addr: u32,
) -> Fragment {
    let mut result = *start;
    for k in 0..iterations {
        let current_step_idx = start_index + k;

        // 1. Derive randomizer r_{i, j}
        let r = derive_randomizer(pub_seed, chain_addr, current_step_idx);

        // 2. XOR: tmp = result ^ r
        let mut tmp = [0u8; HASH_SIZE];
        for b in 0..HASH_SIZE {
            tmp[b] = result[b] ^ r[b];
        }

        // 3. Hash: result = H(tmp)
        let mut hasher = Sha256::new();
        hasher.update(&tmp);
        result = hasher.finalize().into();
    }
    result
}

/// Calculates the checksum for a given message.
/// Same as original WOTS.
fn checksum(msg: &[u8]) -> Vec<u8> {
    let mut c: u16 = 0;
    for &byte in msg {
        c += W - (byte as u16);
    }
    vec![(c >> 8) as u8, (c & 0xFF) as u8]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hash_message(message: &str) -> Fragment {
        let mut hasher = Sha256::new();
        hasher.update(message.as_bytes());
        hasher.finalize().into()
    }

    #[test]
    fn signature_verification_success_plus() {
        let priv_key = WotsPrivateKey::new();
        let pub_key = priv_key.to_public();

        let message_hash = hash_message("WOTS+ Test Message");
        let signature = priv_key.sign(&message_hash);

        assert!(
            pub_key.verify(&message_hash, &signature),
            "WOTS+ signature verification failed"
        );
    }

    #[test]
    fn signature_verification_fail_wrong_key_plus() {
        let priv_key1 = WotsPrivateKey::new();
        let pub_key1 = priv_key1.to_public();
        let priv_key2 = WotsPrivateKey::new();

        let message_hash = hash_message("Attack");
        let signature = priv_key2.sign(&message_hash);

        assert!(
            !pub_key1.verify(&message_hash, &signature),
            "WOTS+ verification should fail for wrong key"
        );
    }

    #[test]
    fn signature_verification_fail_wrong_message_plus() {
        let priv_key = WotsPrivateKey::new();
        let pub_key = priv_key.to_public();

        let msg1 = hash_message("Original");
        let msg2 = hash_message("Forged");

        let signature = priv_key.sign(&msg1);

        assert!(
            !pub_key.verify(&msg2, &signature),
            "WOTS+ verification should fail for wrong message"
        );
    }

    #[test]
    fn determinism_check() {
        let priv_key = WotsPrivateKey::new();
        let pub_key1 = priv_key.to_public();
        let pub_key2 = priv_key.to_public();
        assert_eq!(pub_key1, pub_key2);
    }

    #[test]
    fn check_randomizer_uniqueness() {
        let seed = [1u8; 32];
        let r1 = derive_randomizer(&seed, 0, 0);
        let r2 = derive_randomizer(&seed, 0, 1);
        let r3 = derive_randomizer(&seed, 1, 0);

        assert_ne!(r1, r2);
        assert_ne!(r1, r3);
        assert_ne!(r2, r3);
    }
}
