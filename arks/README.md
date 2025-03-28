# ARKS (Agruments of Knowledge)

This folder contains the implementations of arguments of knowledge (ARKs), SNARKS and STARKs.

## Structure

```
arkworks-zk-research/
├── snarks/
    ├── arks/
        ├── groth16/           # Groth16 SNARK implementation
        ├── plonk/             # PLONK proof system
        └── succinct_gkr/      # Succinct GKR protocol 
```

## SNARK Implementations

### Groth16

A implementation of the Groth16 zk-SNARK proving system. Key features:

- Standard Groth16 setup, prove and verify functionality
- R1CS to QAP transformation
- Support for both manually constructed R1CS and Circuit-derived R1CS
- Uses Arkworks' BLS12-381 curve implementation

### PLONK 

An implementation of the PLONK proving system including:

- Custom circuit compiler and assembly language
- Support for addition and multiplication gates
- Complete proving and verification workflow
- Polynomial commitment schemes
- Fiat-Shamir transforms

### Succinct GKR Protocol

An implementation of a succinct version of the GKR protocol including:

- Support for arithmetic circuits
- Multilinear extension based proof generation
- KZG polynomial commitments
- Sum-check protocol integration
- Benchmarking system

## Usage Requirements

- Rust 2021 edition
- Arkworks dependencies (ark-ff, ark-ec, ark-bls12-381, etc.)

## Features

- Modular design with clear interfaces
- Extensive test coverage 
- Benchmarking support
- Support for different polynomial commitment schemes
- Integration with Arkworks curve implementations

## Testing

Each implementation includes unit tests and integration tests that can be run with:

```bash
cargo test
```

## Benchmarks 

Benchmarks are available for the GKR protocol implementation and can be run with:

```bash
cargo bench
```

## Status

This is a research implementation focused on exploring different zero-knowledge proof systems. The code is intended for study and experimentation rather than production use.

## Contributing

Contributions welcome! Please ensure you add tests for new functionality and follow the existing code style.

## License
MIT

## References

- Groth16 Protocol
- PLONK Protocol 
- GKR Protocol
- Arkworks Framework