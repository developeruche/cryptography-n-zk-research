# Digital Signatures Implementations
This repository contains implementations of popular digital signature algorithms in Rust, including ECDSA, Schnorr signatures, and Winternitz One-Time Signatures.

## Overview
Digital signatures are cryptographic schemes that provide:
- Authentication - Verify who created/sent a message
- Non-repudiation - Signer cannot deny signing later 
- Integrity - Message hasn't been modified

The repository currently implements:

### ECDSA (Elliptic Curve Digital Signature Algorithm)
Located in `ecdsa-rust/`, this implements:
- Core elliptic curve operations and point arithmetic
- ECDSA signature generation and verification 
- Support for arbitrary elliptic curves
- Tests with both small test curves and secp256k1

Key functionality:
```rust
// Generate keypair
let priv_key = BigUint::from(7u32);
let pub_key = ecdsa.generate_pub_key(&priv_key);

// Sign message
let message = "Bob -> 1 BTC -> Alice";
let signature = ecdsa.sign(&hash, &priv_key, &k_random);

// Verify signature
let verify_result = ecdsa.verify(&hash, &pub_key, &signature);
```

### Schnorr Signatures
Located in `schnorr/`, implements:
- Schnorr signature scheme using BLS12-381 curve
- Key generation, signing and verification
- Secure random number generation
- Tests and examples

Example usage:
```rust
// Generate keypair
let key_pair = KeyPair::new(U256::from(123456789u128));

// Sign message 
let message = "Hello, world!";
let signature = sign(key_pair.private_key, message.clone());

// Verify signature
let is_valid = signature.verify(message, key_pair.public_key);
```

### Winternitz One-Time Signature (WOTS)
Located in `winternitz-ots/`, implements:
- Post-quantum secure one-time signature scheme
- Hash-based signature using SHA-256
- Key generation, signing, and verification
- Configurable Winternitz parameter

Example usage:
```rust
// Generate keypair
let priv_key = WotsPrivateKey::new();
let pub_key = priv_key.to_public();

// Hash and sign a message
let message_hash = hash_message("This is a test message");
let signature = priv_key.sign(&message_hash);

// Verify the signature
let is_valid = pub_key.verify(&message_hash, &signature);
```

## Project Structure
```
digital-signatures/
├── ecdsa-rust/         # ECDSA implementation
│   ├── ec-core/        # Core EC operations
│   └── ecdsa/          # ECDSA signing/verification
│
├── schnorr/           # Schnorr signature implementation
│
└── winternitz-ots/    # Winternitz One-Time Signature implementation
```

## Getting Started
Each implementation has its own README with detailed setup and usage instructions. The code includes comprehensive tests and examples.

### Requirements
- Rust 1.50+
- Cargo

### Running Tests
From each implementation directory:
```bash
cargo test
```

## Security Notes
- Implementations are for educational purposes
- Production use requires security review
- Use secure random number generation
- Follow best practices for key management
- The Winternitz OTS scheme is a one-time signature scheme - each private key should only be used once

## Post-Quantum Considerations
The Winternitz One-Time Signature scheme is resistant to quantum computing attacks, unlike traditional digital signature schemes like ECDSA and Schnorr which are vulnerable to Shor's algorithm. WOTS relies solely on the security of cryptographic hash functions, which are believed to remain secure even with quantum computers.

## License
MIT

## Contributing
Contributions welcome! Please feel free to submit issues and pull requests.