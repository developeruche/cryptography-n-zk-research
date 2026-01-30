use p3_field::{PrimeCharacteristicRing, PrimeField64};
use p3_goldilocks::{Goldilocks, Poseidon2Goldilocks};
use p3_symmetric::Permutation;
use rand::{Rng, SeedableRng, rngs::StdRng};

/// The width of the Poseidon2 permutation.
pub const WIDTH: usize = 8;

/// The number of field elements in a digest/fragment.
/// We use the first 4 elements of the permutation output as the digest.
pub const DIGEST_SIZE: usize = 4;

/// The Winternitz parameter W.
/// Using W=16 (4 bits).
pub const W: u64 = 16;

/// The total number of nibbles (4-bit chunks) in a 4-element * 64-bit message.
/// 4 * 64 = 256 bits.
/// 256 / 4 = 64 chunks.
pub const MSG_CHUNKS: usize = 64;

/// Checksum calculation:
/// Max sum = L1 * (W-1) = 64 * 15 = 960.
/// 960 < 2^10. Fits in 12 bits -> 3 nibbles (4-bit chunks).
/// So L2 = 3.
pub const L2: usize = 3;

/// Total chain length.
pub const L: usize = MSG_CHUNKS + L2;

pub type F = Goldilocks;
pub type Digest = [F; DIGEST_SIZE]; 


/// Represents a WOTS private key.
#[derive(Debug, Clone, Default)]
pub struct WotsPrivateKey {
    chains: Vec<F>,
}

/// Represents a WOTS public key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WotsPublicKey {
    chains: Vec<F>,
}

/// Represents a WOTS signature.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WotsSignature {
    chains: Vec<F>,
}


impl WotsPrivateKey {
    /// Generates a new random private key.
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let mut chains = Vec::with_capacity(L);
        for _ in 0..L {
            let val: u64 = rng.random(); 
            chains.push(F::from_u64(val));
        }
        WotsPrivateKey { chains }
    }

    /// Generates the corresponding public key.
    pub fn to_public(&self) -> WotsPublicKey {
        let pub_keys = self
            .chains
            .iter()
            .map(|priv_val| hash_chain(*priv_val, W as u16)) 
            .collect();
        WotsPublicKey { chains: pub_keys }
    }

    /// Signs a message (digest of 4 field elements).
    pub fn sign(&self, message_digest: &Digest) -> WotsSignature {
        let expanded = expand_message(message_digest);
        let cs = checksum(&expanded);
        
        let full_msg = expanded.iter().chain(cs.iter());

        let signature_keys = self
            .chains
            .iter()
            .zip(full_msg)
            .map(|(priv_val, &msg_val)| {
                let iterations = (W - msg_val) as u16;
                hash_chain(*priv_val, iterations)
            })
            .collect();

        WotsSignature {
            chains: signature_keys,
        }
    }
}

impl WotsPublicKey {
    pub fn verify(&self, message_digest: &Digest, signature: &WotsSignature) -> bool {
        if signature.chains.len() != L {
            return false;
        }

        let expanded = expand_message(message_digest);
        let cs = checksum(&expanded);
        let full_msg: Vec<u64> = expanded.iter().chain(cs.iter()).cloned().collect();

        let recovered_keys: Vec<F> = signature
            .chains
            .iter()
            .zip(full_msg.iter())
            .map(|(sig_val, &msg_val)| {
                let iterations = msg_val as u16;
                hash_chain(*sig_val, iterations)
            })
            .collect();

        self.chains == recovered_keys
    }
}


/// One-way function using Poseidon2.
fn hash_chain(start: F, iterations: u16) -> F {
    let mut current = start;
    // Use a fixed seed for deterministic behavior
    let mut rng = StdRng::seed_from_u64(0);
    let poseidon = Poseidon2Goldilocks::<WIDTH>::new_from_rng_128(&mut rng); 

    for _ in 0..iterations {
        let mut input = [F::ZERO; WIDTH];
        input[0] = current;
        
        let output = poseidon.permute(input);
        
        current = output[0];
    }
    current
}

/// Expands a digest (4 field elements) into nibbles (4-bit chunks).
fn expand_message(digest: &Digest) -> Vec<u64> {
    let mut res = Vec::with_capacity(MSG_CHUNKS);
    for &elem in digest {
        let val = <F as PrimeField64>::as_canonical_u64(&elem);
        for i in 0..16 {
            // Big-endian nibbles
            let shift = 60 - (i * 4);
            let nibble = (val >> shift) & 0xF;
            res.push(nibble);
        }
    }
    res
}

fn checksum(msg: &[u64]) -> Vec<u64> {
    let mut c: u64 = 0;
    for &val in msg {
        c += W - val;
    }
    
    // c fits in 12 bits (3 nibbles).
    let mut res = Vec::with_capacity(L2);
    res.push((c >> 8) & 0xF);
    res.push((c >> 4) & 0xF);
    res.push(c & 0xF);
    
    res
}


#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to create a dummy message digest from a string.
    fn mock_hash_message(message: &str) -> Digest {
        let mut digest = [F::ZERO; DIGEST_SIZE];
        let bytes = message.as_bytes();
        // Pack bytes into u64s
        for (i, chunk) in bytes.chunks(8).enumerate() {
            if i >= DIGEST_SIZE { break; }
            let mut val = 0u64;
            for &b in chunk {
                val = (val << 8) | (b as u64);
            }
            digest[i] = F::from_u64(val);
        }
        digest
    }

    #[test]
    fn test_poseidon_wots_success() {
        let priv_key = WotsPrivateKey::new();
        let pub_key = priv_key.to_public();
        let msg = mock_hash_message("TestMessage1234"); 
        
        let sig = priv_key.sign(&msg);
        assert!(pub_key.verify(&msg, &sig), "Verification failed");
    }

    #[test]
    fn test_poseidon_wots_fail_wrong_msg() {
        let priv_key = WotsPrivateKey::new();
        let pub_key = priv_key.to_public();
        let msg1 = mock_hash_message("MessageA");
        let msg2 = mock_hash_message("MessageB");
        
        let sig = priv_key.sign(&msg1);
        assert!(!pub_key.verify(&msg2, &sig), "Should fail for wrong message");
    }

    #[test]
    fn test_checksum() {
         let zeros = [0u64; MSG_CHUNKS]; // 64 chunks
         // Sum = 64 * 16 = 1024 = 0x400. -> [4, 0, 0]
         let cs = checksum(&zeros);
         assert_eq!(cs, vec![4, 0, 0]);
    }
}
