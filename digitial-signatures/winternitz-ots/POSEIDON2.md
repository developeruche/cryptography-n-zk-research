# WOTS with Poseidon2 vs SHA256

This document explains the implementation of the Winternitz One-Time Signature (WOTS) scheme using **Poseidon2** over the **Goldilocks field**, and compares it to the standard **SHA256** implementation.

## Overview

Winternitz OTS is a quantum-resistant digital signature scheme that uses a hash function and a chain of repeated hashing. The security and efficiency dependent heavily on the underlying hash function and parameters.

### Implementations

| Feature | SHA256 Implementation (`src/lib.rs`) | Poseidon2 Implementation (`src/poseidon2.rs`) |
| :--- | :--- | :--- |
| **Hash Function** | SHA-256 (standard) | Poseidon2 (arithmetization-friendly) |
| **Field/Domain** | Bytes (`u8`) | Goldilocks Field Elements (`u64`) |
| **Digest Size** | 32 bytes (256 bits) | 4 Field Elements (~256 bits) |
| **Winternitz Parameter (W)** | 256 (8 bits per chain) | 16 (4 bits per chain) |
| **Message Chunks (L1)** | 32 chunks | 64 chunks |
| **Checksum Chunks (L2)** | 2 chunks | 3 chunks |
| **Total Chain Length (L)** | 34 | 67 |
| **Determinism** | Implicit (SHA256 is deterministic) | Explicit (`StdRng` seed required) |

## Detailed Comparison

### 1. Hash Function & Domain
- **SHA256**: Operates on raw bytes. The "Fragment" is `[u8; 32]`.
- **Poseidon2**: A cryptographic permutation designed for Zero-Knowledge Proof (ZKP) systems. It operates on field elements. We use the **Goldilocks** field ($p = 2^{64} - 2^{32} + 1$). The "Fragment" is a single field element `F` (64 bits), and the "Digest" is `[F; 4]`.

### 2. Parameters (`W`)
- **SHA256 (W=256)**: Interprets each byte of the message hash as a number between 0-255. This requires 256 iterations of the hash function for each chain step.
- **Poseidon2 (W=16)**: To keep proof generation efficient (witness generation for 256 iterations is costly in ZK), we reduced `W` to 16. This means we interpret the message in **4-bit nibbles** instead of 8-bit bytes. 

### 3. Chain Lengths (`L1`, `L2`)
Reducing `W` increases the number of chains needed:
- **SHA256**: 256 bits / 8 bits per chain = **32 chains** (`L1`).
- **Poseidon2**: 256 bits / 4 bits per chain = **64 chains** (`L1`).

The checksum calculation also changes:
- **SHA256**: Max checksum = $32 \times (256-1) = 8160$. Fits in 2 bytes (`L2=2`).
- **Poseidon2**: Max checksum = $64 \times (16-1) = 960$. Fits in 3 nibbles (`L2=3`).

### 4. Determinism
- **SHA256** implementation uses standard `sha2` crate which doesn't require seeding.
- **Poseidon2** permutation technically allows different constants/parameters. To ensure the exact same permutation is used for keys, signing, and verifying, the implementation explicitly seeds a `StdRng` with `0` inside `hash_chain`. This guarantees deterministic behavior across runs.

## Code Structure Differences

### Message Expansion
**SHA256**:
```rust
// SHA256 just iterates bytes
for &byte in msg { ... }
```

**Poseidon2**:
We must expand 4 field elements into 64 nibbles (4-bit chunks):
```rust
fn expand_message(digest: &Digest) -> Vec<u64> {
    // ... extracts 16 nibbles from each of the 4 field elements
}
```

### Hashing Loop
**SHA256**:
```rust
let mut hasher = Sha256::new();
hasher.update(&result);
result = hasher.finalize().into();
```

**Poseidon2**:
```rust
// [current, 0, ..., 0] -> Permutation -> [next, ...]
let mut input = [F::ZERO; WIDTH];
input[0] = current;
let output = poseidon.permute(input);
current = output[0];
```

## Why Poseidon2?
While SHA256 is efficient on standard CPUs, Poseidon2 is specifically optimized for arithmetic circuits used in STARKs and SNARKs. Implementing WOTS with Poseidon2 allows for efficient verification of signatures *inside* a zero-knowledge proof.
