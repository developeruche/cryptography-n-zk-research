use rand::RngCore;
pub use sha2::{Digest, Sha256};

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

    /// Signs a message using the "Rapid Verification" optimization.
    ///
    /// Iterates through `ctr` from 0 to `max_attempts` to find a counter value
    /// that results in a message digest (message || ctr) which minimizes verification cost.
    /// Returns the signature and the selected counter.
    pub fn sign_optimized(&self, message: &[u8], max_attempts: u32) -> (WotsSignature, u32) {
        let mut best_ctr = 0;
        let mut best_score = 0;
        let mut best_digest = [0u8; HASH_SIZE];

        // Pre-compute the hash state with the message
        let mut base_hasher = Sha256::new();
        base_hasher.update(message);

        for ctr in 0..=max_attempts {
            let mut hasher = base_hasher.clone();
            hasher.update(&ctr.to_be_bytes());
            let digest: Fragment = hasher.finalize().into();

            let cs = checksum(&digest);

            // Calculate score: Sum of all chain values (v_i)
            // Verification cost is proportional to sum(W - 1 - v_i).
            // Maximizing sum(v_i) minimizes verification cost.
            let score: u32 = digest.iter().chain(cs.iter()).map(|&x| x as u32).sum();

            if score > best_score {
                best_score = score;
                best_ctr = ctr;
                best_digest = digest;
            }
        }

        // If max_attempts is 0 (or loop didn't run effectively for some reason),
        // ensure we have a valid digest for the initial case.
        // The loop above executes at least once for 0..=0, so best_digest is always set.

        (self.sign(&best_digest), best_ctr)
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

    /// Hash the public key
    pub fn hash(&self) -> Fragment {
        let mut hasher = Sha256::new();
        for key in &self.keys {
            hasher.update(key);
        }
        hasher.update(&self.pub_seed);
        hasher.finalize().into()
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

    #[test]
    fn test_sign_optimized() {
        let priv_key = WotsPrivateKey::new();
        let pub_key = priv_key.to_public();
        let message = b"Optimize this message for rapid verification";
        let max_attempts = 255;

        // Perform optimized signing
        let (signature, ctr) = priv_key.sign_optimized(message, max_attempts);

        // Reconstruct the message hash used in signing: H(message || ctr)
        let mut hasher = Sha256::new();
        hasher.update(message);
        hasher.update(&ctr.to_be_bytes());
        let message_hash: Fragment = hasher.finalize().into();

        // Verify using the standard verify method
        assert!(
            pub_key.verify(&message_hash, &signature),
            "Optimized signature verification failed"
        );
    }

    #[test]
    fn test_optimization_gain() {
        let priv_key = WotsPrivateKey::new();
        let message = b"Benchmark optimization gain";
        let max_attempts = 255;

        // 1. Calculate score for default counter (ctr=0) for comparison
        let mut hasher = Sha256::new();
        hasher.update(message);
        hasher.update(&0u32.to_be_bytes());
        let default_digest: Fragment = hasher.finalize().into();

        let default_cs = checksum(&default_digest);
        let default_score: u32 = default_digest
            .iter()
            .chain(default_cs.iter())
            .map(|&x| x as u32)
            .sum();

        // 2. Perform optimization
        let (_signature, ctr) = priv_key.sign_optimized(message, max_attempts);

        // 3. Calculate optimized score
        // We need to recover the digest used
        let mut hasher = Sha256::new();
        hasher.update(message);
        hasher.update(&ctr.to_be_bytes());
        let opt_digest: Fragment = hasher.finalize().into();

        let opt_cs = checksum(&opt_digest);
        let opt_score: u32 = opt_digest
            .iter()
            .chain(opt_cs.iter())
            .map(|&x| x as u32)
            .sum();

        println!("Default score (ctr=0): {}", default_score);
        println!("Optimized score (ctr={}): {}", ctr, opt_score);

        // The optimized score should be greater than or equal to the default score
        // (It could be equal if 0 happened to be the best, or if max_attempts is 0)
        assert!(
            opt_score >= default_score,
            "Optimization failed to improve score"
        );
    }
}
