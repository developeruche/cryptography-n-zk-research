use rand::RngCore;
use sha2::{Digest, Sha256};

// --- Constants & Type Aliases ---

/// The number of fragments in a key, determined by the hash output size.
/// For SHA-256, this is 32 bytes.
const L: usize = 32;
/// The Winternitz parameter, representing the number of possible values for each message chunk.
/// Using `w=8` bits per chunk, so `W = 2^8 = 256`.
const W: u16 = 256;

/// A type alias for a single key fragment or a message hash.
type Fragment = [u8; L];

// --- Structs ---

/// Represents a WOTS private key, composed of L randomly generated fragments.
#[derive(Debug, Default)]
pub struct WotsPrivateKey {
    keys: Vec<Fragment>,
}

/// Represents a WOTS public key, derived by repeatedly hashing the private key fragments.
#[derive(Debug, PartialEq, Eq)]
pub struct WotsPublicKey {
    keys: Vec<Fragment>,
}

/// Represents a WOTS signature.
#[derive(Debug, PartialEq, Eq)]
pub struct WotsSignature {
    keys: Vec<Fragment>,
}

// --- Implementations ---

impl WotsPrivateKey {
    /// Generates a new random private key.
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut keys = Vec::with_capacity(L);
        for _ in 0..L {
            let mut key_fragment = [0u8; L];
            rng.fill_bytes(&mut key_fragment);
            keys.push(key_fragment);
        }
        WotsPrivateKey { keys }
    }

    /// Generates the corresponding public key from this private key.
    /// Each public key fragment is created by hashing the private key fragment `W` times.
    pub fn to_public(&self) -> WotsPublicKey {
        let pub_keys = self
            .keys
            .iter()
            .map(|priv_key_fragment| hash_chain(priv_key_fragment, W))
            .collect();
        WotsPublicKey { keys: pub_keys }
    }

    /// Signs a message hash with the private key.
    pub fn sign(&self, message_hash: &Fragment) -> WotsSignature {
        let signature_keys = self
            .keys
            .iter()
            .zip(message_hash.iter())
            .map(|(priv_key_fragment, &msg_byte)| {
                let iterations = W - (msg_byte as u16);
                hash_chain(priv_key_fragment, iterations)
            })
            .collect();
        WotsSignature {
            keys: signature_keys,
        }
    }
}

impl WotsPublicKey {
    /// Verifies a signature against a message hash and the public key.
    /// It works by "recovering" the public key from the signature and message,
    /// then checking if it matches the known public key.
    pub fn verify(&self, message_hash: &Fragment, signature: &WotsSignature) -> bool {
        // Re-calculate the public key from the signature and message hash.
        let recovered_keys: Vec<Fragment> = signature
            .keys
            .iter()
            .zip(message_hash.iter())
            .map(|(sig_fragment, &msg_byte)| {
                let iterations = msg_byte as u16;
                hash_chain(sig_fragment, iterations)
            })
            .collect();

        // Compare the recovered public key fragments with the actual public key fragments.
        self.keys == recovered_keys
    }
}

// --- Private Helper Functions ---

/// Performs a hash chain operation: `H(H(...H(start)...))` for `iterations` times.
fn hash_chain(start: &Fragment, iterations: u16) -> Fragment {
    let mut result = *start;
    for _ in 0..iterations {
        let mut hasher = Sha256::new();
        hasher.update(&result);
        result = hasher.finalize().into();
    }
    result
}

// --- Tests ---
#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to hash a message string into a 32-byte array.
    fn hash_message(message: &str) -> Fragment {
        let mut hasher = Sha256::new();
        hasher.update(message.as_bytes());
        hasher.finalize().into()
    }

    #[test]
    fn signature_verification_success() {
        // 1. Generate key pair
        let priv_key = WotsPrivateKey::new();
        let pub_key = priv_key.to_public();

        // 2. Create and sign a message
        let message_hash = hash_message("This is a test message");
        let signature = priv_key.sign(&message_hash);

        // 3. Verify the signature
        assert!(
            pub_key.verify(&message_hash, &signature),
            "Signature should be valid"
        );
    }

    #[test]
    fn signature_verification_fail_wrong_key() {
        // 1. Generate two different key pairs
        let priv_key1 = WotsPrivateKey::new();
        let pub_key1 = priv_key1.to_public(); // The "correct" public key
        let priv_key2 = WotsPrivateKey::new(); // Used to generate the signature

        // 2. Sign message with the wrong private key
        let message_hash = hash_message("Another message");
        let signature = priv_key2.sign(&message_hash);

        // 3. Verify against the correct public key (should fail)
        assert!(
            !pub_key1.verify(&message_hash, &signature),
            "Verification should fail for a signature from a different key"
        );
    }

    #[test]
    fn signature_verification_fail_wrong_message() {
        // 1. Generate key pair
        let priv_key = WotsPrivateKey::new();
        let pub_key = priv_key.to_public();

        // 2. Create two different messages
        let original_message_hash = hash_message("Original message");
        let tampered_message_hash = hash_message("Tampered message!");

        // 3. Sign the original message
        let signature = priv_key.sign(&original_message_hash);

        // 4. Try to verify the signature against the tampered message (should fail)
        assert!(
            !pub_key.verify(&tampered_message_hash, &signature),
            "Verification should fail for a tampered message"
        );
    }

    #[test]
    fn keys_and_signatures_are_consistent() {
        let priv_key = WotsPrivateKey::new();
        let message_hash = hash_message("Consistency check");

        let pub_key1 = priv_key.to_public();
        let pub_key2 = priv_key.to_public();
        assert_eq!(
            pub_key1, pub_key2,
            "Public key generation should be deterministic"
        );

        let signature1 = priv_key.sign(&message_hash);
        let signature2 = priv_key.sign(&message_hash);
        assert_eq!(
            signature1, signature2,
            "Signature generation should be deterministic"
        );
    }

    #[test]
    fn different_messages_yield_different_signatures() {
        let priv_key = WotsPrivateKey::new();
        let message_hash1 = hash_message("Message One");
        let message_hash2 = hash_message("Message Two");

        let signature1 = priv_key.sign(&message_hash1);
        let signature2 = priv_key.sign(&message_hash2);

        assert_ne!(
            signature1, signature2,
            "Different messages should produce different signatures"
        );
    }
}
