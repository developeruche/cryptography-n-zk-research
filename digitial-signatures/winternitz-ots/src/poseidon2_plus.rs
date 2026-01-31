use p3_field::{PrimeCharacteristicRing, PrimeField64};
use p3_goldilocks::{Goldilocks, Poseidon2Goldilocks};
use p3_symmetric::Permutation;
use rand::{Rng, SeedableRng, rngs::StdRng}; // Added Rng trait

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

/// Represents a WOTS+ private key.
#[derive(Debug, Clone)]
pub struct WotsPrivateKey {
    chains: Vec<F>,
    pub_seed: Digest,
}

/// Represents a WOTS+ public key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WotsPublicKey {
    chains: Vec<F>,
    pub_seed: Digest,
}

/// Represents a WOTS+ signature.
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

        let mut pub_seed = [F::ZERO; DIGEST_SIZE];
        for i in 0..DIGEST_SIZE {
            let val: u64 = rng.random();
            pub_seed[i] = F::from_u64(val);
        }

        WotsPrivateKey { chains, pub_seed }
    }

    /// Generates the corresponding public key.
    pub fn to_public(&self) -> WotsPublicKey {
        let pub_keys = self
            .chains
            .iter()
            .enumerate()
            .map(|(i, priv_val)| hash_chain(*priv_val, 0, W as u16, &self.pub_seed, i as u32))
            .collect();
        WotsPublicKey {
            chains: pub_keys,
            pub_seed: self.pub_seed,
        }
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
            .enumerate()
            .map(|(i, (priv_val, &msg_val))| {
                let iterations = (W - msg_val) as u16;
                hash_chain(*priv_val, 0, iterations, &self.pub_seed, i as u32)
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
            .enumerate()
            .map(|(i, (sig_val, &msg_val))| {
                let iterations = msg_val as u16;
                let start_idx = (W - msg_val) as u16;
                hash_chain(*sig_val, start_idx, iterations, &self.pub_seed, i as u32)
            })
            .collect();

        self.chains == recovered_keys
    }
}

/// Derives the randomization value r_{i,j}
/// PRF(pub_seed, chain_idx, step_idx)
fn derive_randomizer(pub_seed: &Digest, chain_idx: u32, step_idx: u16) -> F {
    // Setup Poseidon2 permutation
    // Use fixed seed for deterministic behavior of the PRF instantiation?
    // Actually typically we just use the Permutation struct directly if possible,
    // but p3 library might expect a struct.
    // The previous code used `Poseidon2Goldilocks::new_from_rng_128`.
    // We should probably instantiate it once or efficiently.
    // For now, let's just make it fresh here as it's stateless.
    let mut rng = StdRng::seed_from_u64(0);
    let poseidon = Poseidon2Goldilocks::<WIDTH>::new_from_rng_128(&mut rng);

    let mut input = [F::ZERO; WIDTH];
    // pub_seed takes 4 slots
    input[0] = pub_seed[0];
    input[1] = pub_seed[1];
    input[2] = pub_seed[2];
    input[3] = pub_seed[3];
    // chain_idx
    input[4] = F::from_u64(chain_idx as u64);
    // step_idx
    input[5] = F::from_u64(step_idx as u64);
    // 6, 7 are zero padding

    let output = poseidon.permute(input);
    output[0]
}

/// One-way function using Poseidon2 with WOTS+ randomization.
/// x_{i, j+1} = H(x_{i, j} XOR r_{i, j})
fn hash_chain(
    start: F,
    start_index: u16,
    iterations: u16,
    pub_seed: &Digest,
    chain_addr: u32,
) -> F {
    let mut current = start;
    // Use a fixed seed for deterministic behavior
    let mut rng = StdRng::seed_from_u64(0);
    let poseidon = Poseidon2Goldilocks::<WIDTH>::new_from_rng_128(&mut rng);

    for k in 0..iterations {
        let current_step_idx = start_index + k;

        // 1. Derive randomizer
        let r = derive_randomizer(pub_seed, chain_addr, current_step_idx);

        // 2. XOR
        // Goldilocks field is not binary field, but user requested XOR.
        // We do bitwise XOR on the canonical u64 representation and wrap back to field.
        let val_cur = current.as_canonical_u64();
        let val_r = r.as_canonical_u64();
        let mut val_xor = val_cur ^ val_r;

        // Manual reduction to ensure canonical input for from_u64
        // Goldilocks modulus is 2^64 - 2^32 + 1.
        // We use the trait constant to be generic/safe.
        let order = <F as PrimeField64>::ORDER_U64;
        if val_xor >= order {
            val_xor -= order;
        }

        let input_elem = F::from_u64(val_xor);

        // 3. Hash H(input)
        let mut input = [F::ZERO; WIDTH];
        input[0] = input_elem;

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
            if i >= DIGEST_SIZE {
                break;
            }
            let mut val = 0u64;
            for &b in chunk {
                val = (val << 8) | (b as u64);
            }
            digest[i] = F::from_u64(val);
        }
        digest
    }

    #[test]
    fn test_poseidon_wots_plus_success() {
        let priv_key = WotsPrivateKey::new();
        let pub_key = priv_key.to_public();
        let msg = mock_hash_message("TestMessage1234");

        let sig = priv_key.sign(&msg);
        assert!(pub_key.verify(&msg, &sig), "Verification failed");
    }

    #[test]
    fn test_poseidon_wots_plus_fail_wrong_msg() {
        let priv_key = WotsPrivateKey::new();
        let pub_key = priv_key.to_public();
        let msg1 = mock_hash_message("MessageA");
        let msg2 = mock_hash_message("MessageB");

        let sig = priv_key.sign(&msg1);
        assert!(
            !pub_key.verify(&msg2, &sig),
            "Should fail for wrong message"
        );
    }

    #[test]
    fn test_randomizer_determinism() {
        let mut rng = rand::rng();
        let mut seed = [F::ZERO; DIGEST_SIZE];
        for i in 0..DIGEST_SIZE {
            seed[i] = F::from_u64(rng.random());
        }

        let r1 = derive_randomizer(&seed, 10, 5);
        let r2 = derive_randomizer(&seed, 10, 5);
        assert_eq!(r1, r2, "Randomizer must be deterministic");

        let r3 = derive_randomizer(&seed, 10, 6);
        assert_ne!(
            r1, r3,
            "Different Step index should give different randomizer"
        );
    }
}
