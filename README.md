# Cryptography and ZK research
-------------------------------

A Cryptography and Zero Knowledge Proof Research Repo Implementing Research papers, Cryptographic primitives, trying out imaginary exploits and so on.

<br>
<br>
<br>





















## Primitives and Toolkits
-------------------------------
These are a collection of cryptographic primitives and toolkits that I have implemented in Rust. They are designed to be modular, 
efficient, and easy to use. The goal is to provide a solid foundation for building secure, succinct, and privacy-preserving applications.

<br>

### Circuits 
This is a library for creating and manipulating circuits. The library is designed to be modular and extensible. The library is designed to be used in the context of snarks, but can be used for any type of circuit.

**Features**
- [x] Arithmetic Circuit representation
- [x] Boolean Circuit representation
- [x] Circuit evaluation
- [x] Circuit optimization
- [x] Arithmetic Circuit to R1CS conversion
- [x] GKR `Add` and `Mul` Multilinear Extension (MLE)

[codebase](https://github.com/developeruche/cryptography-n-zk-research/tree/main/circuits)
<br>
<br>

### Polynomial
This is the implementation of a polynomial in Rust. The Polynomial struct allows you to create a polynomial, evaluate it at a specific point, and add or multiply two polynomials together.

The variations of polynomials built in here are;
- [x] Univariate Polynomial
- [x] Multivariate Polynomial
- [x] Multilinear Polynomial

... the last two could give a man 2^N complexity nightmare :).

[codebase](https://github.com/developeruche/cryptography-n-zk-research/tree/main/polynomial)
<br>
<br>

### SHA256 hash function
In the realm of cybersecurity, ensuring data integrity and authenticity is paramount. Cryptographic hash functions play a vital role in achieving this objective. One of the most widely employed and trusted algorithms in this domain is SHA-256 (Secure Hash Algorithm 256-bit). This introduction will delve into the concept of SHA-256, outlining its functionalities, key characteristics, and the prevalent applications that leverage its capabilities.

**Features**
- [x] Technical paper explanation
- [x] Operation implementation
- [ ] Complete hash function implementation

[codebase](https://github.com/developeruche/cryptography-n-zk-research/tree/main/sha256-hash-function)
<br>
<br>



<br>
<br>

### Protocol Implementations
-------------------------------
These are implementations of cryptographic protocols that I have implemented in Rust. 
These protocols are built using the primitives and toolkits mentioned above as building blocks.
These implementations follows research papers and are designed to be efficient, secure, and easy to use,
But with the primary purpose of research and education.

<br>


### GKR 
The GKR protocol is fascinating, fairly not as complicated as other protocols but heavily generic and useful. The GKR protocol involves running one protocol (Sum check) inside this protocol (GKR). The GKR protocol, named after Shafi Goldwasser, Yael Tauman Kalai, and Guy N. Rothblum, is a zero-knowledge proof protocol that focuses on efficient delegation of computation. Specifically, it is designed to verify computations represented as layered arithmetic circuits with logarithmic depth in the number of inputs. The GKR protocol is known for its efficiency and ability to handle large-scale computations.


**Features**
- [x] In Progress


[codebase](https://github.com/developeruche/cryptography-n-zk-research/tree/main/gkr)
<br>
<br>



### Groth16 
The Groth16 protocol is a highly efficient zero-knowledge succinct non-interactive argument of knowledge (zk-SNARK) for general arithmetic circuit satisfiability. It was introduced by Jens Groth in 2016 and is notable for its concise proofs and efficient verification. 

**Features**
- [x] Circuit Pre-processing
- [x] Trusted Setup
- [x] Prover
- [x] Verifer


[codebase](https://github.com/developeruche/cryptography-n-zk-research/tree/main/gkr)
<br>
<br>

### ECDSA

This library provides an implementation of the Elliptic Curve Digital Signature Algorithm (ECDSA) in Rust. It allows you to:

**Features**
- [x] Generate ECDSA keypairs (private and public keys)
- [x] Sign messages using a private key
- [x] Verify signatures using a public key


[codebase](https://github.com/developeruche/ecdsa-rust)
<br>
<br>


### KZG polynomial commitment scheme

This is a RUST implementation of the Kate Commitments (KZG) polynomial commitment scheme. KZG allows you to commit to a polynomial while keeping the contents hidden.

**Features**
1. Univariate Polynomial Commitment
  - [x] Creates KZG parameters for a given degree.
  - [x] Commits to a polynomial of a specified degree.
  - [x] Opens a commitment at a specific point, revealing the evaluated value and proof.
  - [x] Verifies the opening proof for a commitment.
  - [] Open and Verifer bacth on single polynomial multiple points.

2. Multilinear Polynomial Commitment
  - [x] Creates KZG parameters for a given number of variables.
  - [x] Commits to a polynomial of a specified number of variables.
  - [x] Opens a commitment at a specific point, revealing the evaluated value and proof.
  - [x] Verifies the opening proof for a commitment.

[codebase](https://github.com/developeruche/kzg-commitment-rust)
<br>
<br>

### Interactive Chaum Pederson zero-knowledge proof
The Chaum-Pedersen Zero-Knowledge Proof (ZKP) protocol allows a prover to convince a verifier that they possess a secret value (knowledge) without revealing the actual value itself. It operates in an interactive setting, meaning the prover and verifier exchange messages back and forth.

**Features**
- [x] Interactive proof generation and verification.

[codebase](https://github.com/developeruche/non-interactive-chaum-pedersen-lib)
<br>
<br>


### Non-Interactive Implementation
The standard Chaum-Pedersen protocol is interactive, but it can be converted into a non-interactive version using the Fiat-Shamir heuristic. This transformation removes the need for real-time interaction:

**Features**
- [x] Non-interactive proof generation
- [x] Non-interactive proof verification

[codebase](https://github.com/developeruche/non-interactive-chaum-pedersen-lib)
<br>
<br>

### Schnorr Digital Signature
A Schnorr signature is a digital signature produced by the Schnorr signature algorithm, which Claus Schnorr described. It’s known for its simplicity and efficiency.

**Features**
- [x] Generate Schnorr keypairs (private and public keys)
- [x] Sign messages using a private key
- [x] Verify signatures using a public key

[codebase](https://github.com/developeruche/cryptography-n-zk-research/tree/main/schnorr)
<br>
<br>


### Sum Check Protocol
Suppose we are given a v-variate polynomial `g` defined over a finite field `F`. 
The purpose of the sum-check protocol is for the prover to provide the verifier with the sum of evaluations over the boolean hypercube.

**Features**
- [x] Technical documentation
- [x] library implementation
- [x] library test implementation

[codebase](https://github.com/developeruche/cryptography-n-zk-research/tree/main/sum_check)
<br>
<br>

### Fiat Shamir 
The Fiat-Shamir transcript is a cryptographic technique used to transform an interactive proof system into a non-interactive one. This transformation enhances the efficiency and practicality of certain cryptographic protocols, including zero-knowledge proofs, by eliminating the need for interaction between the prover and the verifier.


**Features**
- [x] Externally adadptable interface
- [x] Transcript implementation

[codebase](https://github.com/developeruche/cryptography-n-zk-research/tree/main/fiat_shamir)
<br>
<br>





<br>
<br>


























## Projects
These are zk, cryptographic and blockchain projects that I have implemented in Rust and other tools. These projects builds on the primitives, toolkits, protocols mentioned above and 
other amazing projects built by great minds in the field of cryptography and zero-knowledge proofs.

<br>

### KZG airdrop over bn254 Elliptic curve

This is a RUST implementation of the KZG commitment scheme over the bn254 elliptic curve. Using this implementation to perform a token airdrop distribution using instead of using the regular `Merkle tree` commitment scheme.

**Features**
- [x] Toolkit for data preparation and formatting.
- [x] Trusted setup importation and implementation.
- [x] Massive proof generation script (Still needs optimization)
- [x] Rust proof verification
- [ ] Solidity proof verification

[codebase](https://github.com/developeruche/kzg-airdrop-bn254)
<br>
<br>


### Light tornado cash (powered by Groth16)
In the realm of blockchain technology, privacy remains a crucial aspect. This research project delves into a lightweight implementation inspired by the core concepts of cryptocurrency mixers. Our goal is to explore anonymity-enhancing techniques for crypto transactions while adhering to legal and ethical frameworks.

**Features**
- [x] Circom Circuit (Main Circuit and Hash functions)
- [x] Solidity Interface and Verification smart contract
- [ ] Basic UI

[codebase](https://github.com/developeruche/light-tornodo-cash)
<br>
<br>

### Verifiable Random Function (VRF)
A Verifiable Random Function (VRF) is a cryptographic primitive that provides a way to generate a random output that can be publicly verified as having been produced by a specific input and a specific secret key. It combines the properties of a hash function with those of a digital signature.

**Features**
- [x] Rust implementaion of VRS
- [x] Solidity binding interface

[codebase](https://github.com/developeruche/vrf-rust-solidity/tree)
<br>
<br>

### BLS Multi Sign Threshold Wallet (Powered by Shimir secret sharing protocol)
This is a mini project, using the Shamir Secret Sharing Scheme lib. This project would share the private key to an account among many other entities and a chosen number of these entities can come together to recompute this initial private key.

**Features**
- [x] Binding Lib
- [x] Script for secret sharing

[codebase](https://github.com/developeruche/cryptography-n-zk-research/tree/main/bls-multi-sign-threshold-wallet)
<br>
<br>

### Circom Groth16
This is a experimental project aiming to prove and verify a circom circuit using the Groth16 implemenation done in this reasearch project.

**Features**
- [x] Circom circuit
- [ ] Circom intermediate representation binding
- [ ] Groth16 implementation (Prove and Vefier)

[codebase](https://github.com/developeruche/cryptography-n-zk-research/tree/main/circom-groth16)
<br>
<br>

### Succinct GRK
This is a experimental project aiming to using the GKR protocol with Multilinear poloynomial commitment scheme to create a SNARK.

**Features**
- [x] Circuit Representation
- [ ] Prover function
- [ ] Verifier function

[codebase](https://github.com/developeruche/cryptography-n-zk-research/tree/main/circom-groth16)
<br>
<br>

<br>
<br>
























## Research Papers and Study Notes
This sections contains reasearch papers and study note that I have written on various topics in cryptography and zero-knowledge proofs.

<br>

<br>
<br>





















<br>
<br>
<br>
<br>



NOTE: THIS IS NOT TO BE USED IN PRODUCTION. THIS IS A RESEARCH PROJECT.
