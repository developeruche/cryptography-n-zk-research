# Hash Functions

This directory contains implementations of various cryptographic hash functions in Rust.

## Overview

This repository houses different hash function implementations, focusing on providing both educational resources and practical implementations. The current implementations include:

- SHA-256
- Keccak-256

## Implementations

### SHA-256

The SHA-256 implementation (`sha256-hash-function/`) provides:
- Complete implementation of the SHA-256 algorithm
- Detailed documentation of the algorithm's components
- Testing suite for verification

Key features:
- Preprocessing of input data
- Message schedule computation
- Core hash computation
- Various helper functions and operations

### Keccak-256

The Keccak-256 implementation (`keccak256/`) is:
- Currently under development
- Will implement the Keccak-f[1600] permutation
- Will follow the FIPS 202 standard

## Structure

```
hash-functions/
├── keccak256/
│   ├── src/
│   └── Cargo.toml
└── sha256-hash-function/
    ├── src/
    ├── README.md
    └── Cargo.toml
```

## Getting Started

Each hash function implementation is a separate Rust crate. To use any of the implementations:

1. Navigate to the specific implementation directory
2. Build using cargo:
```bash
cargo build
```
3. Run tests:
```bash
cargo test
```

## Contributing

Contributions are welcome! Please feel free to submit pull requests, open issues, or suggest improvements.

## License

Each implementation may have its own license. Please check individual directories for specific licensing information.