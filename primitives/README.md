# Cryptographic & ZK Primitives
This folder contains core cryptographic and zero-knowledge primitives used throughout the project.

## Components

### Circuits
Implementation of arithmetic and binary circuits that can be used with various ZK proof systems:

- Binary circuits
- Arithmetic circuits
- Circuit adapters for different proving systems (R1CS, Plonkish)
- Support for Groth16 proof generation and verification

Key features:
- Circuit layer composition
- Circuit evaluation 
- Constraint generation
- Circuit adapters for different proving models

### Polynomial
Various polynomial implementations optimized for ZK proofs:

- Univariate polynomials
- Multivariate polynomials 
- Multilinear polynomials
- Polynomial composition
- FFT evaluation domains
- Virtual polynomials for efficient representation

Key functionality:
- Polynomial arithmetic
- Interpolation
- Evaluation
- FFT/IFFT operations
- Efficient composition

### Transcripts
Cryptographic transcript implementations for Fiat-Shamir transformations:

#### Fiat-Shamir
- Non-interactive proof transcript generation
- Challenge derivation
- Field element sampling
- Transcript composition with labels

#### Merlin
- Strobe-based transcripts
- Domain separation
- Witness commitment

## Usage

The primitives in this folder serve as building blocks for implementing zero-knowledge proof systems. They provide the core mathematical and cryptographic operations needed for:

- Circuit building and evaluation
- Polynomial operations
- Proof transcript generation
- Challenge derivation
- Commitment schemes

See the individual component READMEs for specific usage examples and API documentation.

## Dependencies

The primitives rely on these key dependencies:

- `ark-ff`: Finite field arithmetic
- `ark-poly`: Polynomial implementations
- `ark-ec`: Elliptic curve operations  
- `sha3`: Cryptographic hash functions

## Testing

Each component contains comprehensive unit tests. Run tests with:

```bash
cargo test --all
```

## Contributing

When adding new primitives:

1. Create a new subfolder with appropriate name
2. Include README with documentation
3. Add unit tests
4. Update this main README 
5. Follow the modular architecture pattern

## License

This project is licensed under [LICENSE NAME] - see the LICENSE file for details.