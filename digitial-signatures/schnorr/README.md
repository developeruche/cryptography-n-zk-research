# Schnorr Digital Signatures
A Rust implementation of the Schnorr digital signature scheme using BLS12-381 elliptic curve.

## Overview
This library provides a simple and efficient implementation of Schnorr signatures, which are known for their simplicity and efficiency compared to traditional ECDSA signatures. The implementation uses the BLS12-381 elliptic curve and includes key generation, signing, and verification functionalities.

## Features
- Generate Schnorr key pairs (private and public keys)
- Sign messages using Schnorr signature scheme
- Verify Schnorr signatures
- Built on top of the BLS12-381 elliptic curve
- Secure random number generation for signature creation

## Installation
Add this to your `Cargo.toml`:

```toml
[dependencies]
schnorr = "0.1.0"
```

## Usage

### Basic Example
```rust
use schnorr::{sign, core::KeyPair};
use lambdaworks_math::unsigned_integer::element::U256;

fn main() {
    // Generate a key pair
    let key_pair = KeyPair::new(U256::from(123456789u128));
    
    // Message to sign
    let message = "Hello, world!".to_string();
    
    // Create signature
    let signature = sign(key_pair.private_key, message.clone()).unwrap();
    
    // Verify signature
    let is_valid = signature.verify(message, key_pair.public_key).unwrap();
    assert!(is_valid);
}
```

### Detailed Usage

1. **Generate Key Pair**
```rust
let private_key = U256::from(123456789u128);
let key_pair = KeyPair::new(private_key);
```

2. **Sign a Message**
```rust
let message = "Your message here".to_string();
let signature = sign(key_pair.private_key, message.clone()).unwrap();
```

3. **Verify a Signature**
```rust
let is_valid = signature.verify(message, key_pair.public_key).unwrap();
```

## Technical Details

The Schnorr signature scheme implemented here consists of the following components:

1. **Key Generation**:
   - Private key: Random scalar k
   - Public key: P = kG (where G is the generator point)

2. **Signing**:
   - Generate random k
   - Calculate r = kG
   - Calculate e = H(r || message)
   - Calculate s = k - e * private_key
   - Signature is (s, e)

3. **Verification**:
   - Calculate r' = g^s * public_key^e
   - Calculate e' = H(r' || message)
   - Verify e == e'

## Dependencies
- `lambdaworks-math`: For elliptic curve operations
- `rand`: For secure random number generation
- `sha256`: For hash operations
- `anyhow`: For error handling

## Security Notes
- This implementation uses cryptographically secure random number generation
- The library uses the BLS12-381 elliptic curve
- Hash function used is SHA-256

## License
This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing
Contributions are welcome! Please feel free to submit a Pull Request.

## Disclaimer
This implementation is for educational purposes. For production use, please review and audit the code thoroughly.