# Cryptography and ZK research
-------------------------------

## ECDSA

This library provides an implementation of the Elliptic Curve Digital Signature Algorithm (ECDSA) in Rust. It allows you to:

**Features**
- [x] Generate ECDSA keypairs (private and public keys)
- [x] Sign messages using a private key
- [x] Verify signatures using a public key


[codebase](https://github.com/developeruche/ecdsa-rust)
<br>
<br>

## KZG polynomial commitment scheme

This is a Rust implementation of the Kate Commitments (KZG) polynomial commitment scheme. KZG allows you to commit to a polynomial while keeping the contents hidden.

**Features**

- [x] Creates KZG parameters for a given degree.
- [x] Commits to a polynomial of a specified degree.
- [x] Opens a commitment at a specific point, revealing the evaluated value and proof.
- [x] Verifies the opening proof for a commitment.

[codebase](https://github.com/developeruche/ecdsa-rust)
<br>
<br>

## KZG airdrop over bn254 Elliptic curve

This is a Rust implementation of the KZG commitment scheme over the bn254 elliptic curve. Using this implementation to perform a token airdrop distribution using instead of using the regular `Merkle tree` commitment scheme.

**Features**
- [ ] Toolkit for data preparation and formatting.
- [ ] Trusted setup importation and implementation.
- [ ] Massive proof generation script
- [ ] Rust proof verification
- [ ] Solidity proof verification

[codebase](https://github.com/developeruche/ecdsa-rust)
<br>
<br>

## Light tornado cash (powered by Groth16)
In the realm of blockchain technology, privacy remains a crucial aspect. This research project delves into a lightweight implementation inspired by the core concepts of cryptocurrency mixers. Our goal is to explore anonymity-enhancing techniques for crypto transactions while adhering to legal and ethical frameworks.

**Features**
- [x] Circom Circuit (Main Circuit and Hash functions)
- [x] Solidity Interface and Verification smart contract
- [ ] Basic UI

[codebase](https://github.com/developeruche/ecdsa-rust)
<br>
<br>

## Non-interactive Chaum Pederson zero-knowledge proof
The Chaum-Pedersen Zero-Knowledge Proof (ZKP) protocol allows a prover to convince a verifier that they possess a secret value (knowledge) without revealing the actual value itself. It operates in an interactive setting, meaning the prover and verifier exchange messages back and forth.

**Features**
- [x] Interactive proof generation and verification.

[codebase](https://github.com/developeruche/ecdsa-rust)
<br>
<br>


## Non-Interactive Implementation
The standard Chaum-Pedersen protocol is interactive, but it can be converted into a non-interactive version using the Fiat-Shamir heuristic. This transformation removes the need for real-time interaction:

**Features**
- [x] Non-interactive proof generation
- [x] Non-interactive proof verification

[codebase](https://github.com/developeruche/ecdsa-rust)
<br>
<br>

## Polynomial
This is the implementation of a polynomial in Rust. The Polynomial struct allows you to create a polynomial, evaluate it at a specific point, and add or multiply two polynomials together.

The variations of polynomials built in here are;
- Univariate Polynomial
- Multivariate Polynomial
- Multilinear Polynomial

... the last two could give a man 2^N complexity nightmare :).

[codebase](https://github.com/developeruche/ecdsa-rust)
<br>
<br>


## Schnorr Digital Signature
A Schnorr signature is a digital signature produced by the Schnorr signature algorithm, which Claus Schnorr described. It’s known for its simplicity and efficiency.

**Features**
- [x] Generate Schnorr keypairs (private and public keys)
- [x] Sign messages using a private key
- [x] Verify signatures using a public key

[codebase](https://github.com/developeruche/ecdsa-rust)
<br>
<br>


## SHA256 hash function
In the realm of cybersecurity, ensuring data integrity and authenticity is paramount. Cryptographic hash functions play a vital role in achieving this objective. One of the most widely employed and trusted algorithms in this domain is SHA-256 (Secure Hash Algorithm 256-bit). This introduction will delve into the concept of SHA-256, outlining its functionalities, key characteristics, and the prevalent applications that leverage its capabilities.

**Features**
- [x] Technical paper explanation
- [x] Operation implementation
- [ ] Complete hash function implementation

[codebase](https://github.com/developeruche/ecdsa-rust)
<br>
<br>

## Verifiable Random Function (VRF)
A Verifiable Random Function (VRF) is a cryptographic primitive that provides a way to generate a random output that can be publicly verified as having been produced by a specific input and a specific secret key. It combines the properties of a hash function with those of a digital signature.

**Features**
- [x] Rust implementaion of VRS
- [x] Solidity binding interface

[codebase](https://github.com/developeruche/ecdsa-rust)
<br>
<br>


## BLS Multi Sign Threshold Wallet (Powered by Shimir secret sharing protocol)
This is a mini project, using the Shamir Secret Sharing Scheme lib. This project would share the private key to an account among many other entities and a chosen number of these entities can come together to recompute this initial private key.

**Features**
- [x] Binding Lib
- [x] Script for secret sharing

[codebase](https://github.com/developeruche/ecdsa-rust)
<br>
<br>

## Sum Check Protocol
Suppose we are given a v-variate polynomial `g` defined over a finite field `F`. The purpose of the sum-check protocol is for the prover to provide the verifier with the sum of evaluations over the boolean hypercube.

**Features**
- [x] Technical documentation
- [x] library implementation
- [x] library test implementation

[codebase](https://github.com/developeruche/ecdsa-rust)
<br>
<br>

### Circuits 

### Fiat Shamir 

### GKR 

### Groth16 









NOTE: THIS IS NOT TO BE USED IN PRODUCTION. THIS IS A RESEARCH PROJECT.
