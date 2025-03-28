# Product Check Protocol Implementation

This crate implements the product check protocol, which is a cryptographic protocol used to verify the correctness of polynomial multiplication in zero-knowledge proofs.

## Overview

The product check protocol allows a prover to convince a verifier that certain polynomial relationships hold without revealing the actual polynomials. It's particularly useful in zk-SNARK constructions and other cryptographic protocols.

## Features

- Implementation of the product check protocol using KZG commitments
- Support for multilinear polynomials
- Integration with Fiat-Shamir heuristic for non-interactive proofs
- Zero-check protocol integration
- Efficient batch operations for polynomial evaluation

## Core Components

- `ProductCheck`: Main struct implementing the product check protocol
- `ProductCheckInterface`: Trait defining the protocol interface
- `ProductCheckProof`: Struct containing proof elements
- Utility functions for polynomial operations and protocol execution

## Usage

```rust
use product_check::{ProductCheck, ProductCheckInterface};

// Initialize polynomials and KZG SRS
let poly_1 = // ... composed multilinear polynomial
let poly_2 = // ... composed multilinear polynomial
let kzg_srs = // ... KZG structured reference string

// Create transcript for Fiat-Shamir
let mut transcript = FiatShamirTranscript::default();

// Generate proof
let (proof, product_poly, fractional_poly, q_x) = 
    ProductCheck::prove(&poly_1, &poly_2, &kzg_srs, &mut transcript)?;

// Verify proof
let (final_query, final_eval, alpha) = 
    ProductCheck::verify(&proof, &q_x, &mut transcript)?;
```

## Dependencies

This crate is part of a larger workspace and depends on:
- `sum_check`
- `fiat_shamir`
- `ark-ff`
- `ark-ec`
- `polynomial`
- `zero_check`
- `pcs`

## Testing

Run tests using:
```bash
cargo test
```