# Cryptography and ZK research
-------------------------------

## ECDSA

This library provides an implementation of the Elliptic Curve Digital Signature Algorithm (ECDSA) in Rust. It allows you to:

**Features**
- Generate ECDSA keypairs (private and public keys)
- Sign messages using a private key
- Verify signatures using a public key


[codebase]()
<br>
<br>

## KZG polynomial commitment scheme

This is a Rust implementation of the Kate Commitments (KZG) polynomial commitment scheme. KZG allows you to commit to a polynomial while keeping the contents hidden.

**Features**

- Creates KZG parameters for a given degree.
- Commits to a polynomial of a specified degree.
- Opens a commitment at a specific point, revealing the evaluated value and a proof.
- Verifies the opening proof for a commitment.

## KZG airdrop over bn254 Elliptic curve

This is a Rust implementation of the KZG commitment scheme over the bn254 elliptic curve. It allows you to commit to a polynomial and prove that a specific evaluation of the polynomial results in a certain value. This is useful for various cryptographic applications where you want to prove possession of data without revealing it.


## Light tornodo cash (powered by Groth16)

In the realm of blockchain technology, privacy remains a crucial aspect. This research project delves into a lightweight implementation inspired by the core concepts of cryptocurrency mixers. Our goal is to explore anonymity-enhancing techniques for crypto transactions while adhering to legal and ethical frameworks.

## Non-interactive chaum pederson zero-knowledge proof

The Chaum-Pedersen Zero-Knowledge Proof (ZKP) protocol allows a prover to convince a verifier that they possess a secret value (knowledge) without revealing the actual value itself. It operates in an interactive setting, meaning the prover and verifier exchange messages back and forth.

## Non-Interactive Implementation
The standard Chaum-Pedersen protocol is interactive, but it can be converted into a non-interactive version using the Fiat-Shamir heuristic. This transformation removes the need for real-time interaction:

## Polynomial

This is the implementation of a polynomial in Rust. The Polynomial struct allows you to create a polynomial, evaluate it at a specific point, and add or multiply two polynomials together.

The variation of polynomial built in here are;
- Univariate Polynomial
- Multivariate Polynomial
- Multilinear Polynomial

... the last two could give a man 2^N complexity nightmare :).

## Schnorr Digital Signature

## SHA256 hash function
In the realm of cybersecurity, ensuring data integrity and authenticity is paramount. Cryptographic hash functions play a vital role in achieving this objective. One of the most widely employed and trusted algorithms in this domain is SHA-256 (Secure Hash Algorithm 256-bit). This introduction will delve into the concept of SHA-256, outlining its functionalities, key characteristics, and the prevalent applications that leverage its capabilities.

## Verifiable Random Function (VRF)

## BLS Multi Sign Threshold Wallet (Powered by Shimir secret sharing protocol)

## Sum Check Protocol

Suppose we are given a v-variate polynomial g defined over a finite field F. The purpose of the sum-check protocol is for the prover to provide the verifier with the sum of evaluations over the boolean hypercube.











NOTE: THIS IS NOT TO BE USED IN PRODUCTION. THIS IS A RESEARCH PROJECT.
