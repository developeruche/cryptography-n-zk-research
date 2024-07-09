# Sum check protocol
--------------------

Sum-Check Protocol The Sum-Check Protocol is a fundamental technique in theoretical computer science, particularly in the field of interactive proof systems and complexity theory. It’s often used to prove properties about polynomials and is a key component in constructing various interactive proofs, including those for NP-complete problems.

Suppose we are given a v-variate polynomial g defined over a finite field F. The purpose of the sum-check protocol is for the prover to provide the verifier with the sum of evaluations over the boolean hypercube.


###  Build Guide

1. Clone the repository

```bash
git clone **
```

2. Build the project

```bash
cargo build --release
```

3. Run the project test

```bash
cargo test
```

### Usage (How to compute proofs and verify claims)

```rust

    let poly = Multilinear::new(
        vec![
            Fr::from(1),
            Fr::from(3),
            Fr::from(5),
            Fr::from(7),
            Fr::from(2),
            Fr::from(4),
            Fr::from(6),
            Fr::from(8),
            Fr::from(3),
            Fr::from(5),
            Fr::from(7),
            Fr::from(9),
            Fr::from(4),
            Fr::from(6),
            Fr::from(8),
            Fr::from(10),
        ],
        4,
    );
    let mut prover = Prover::new(poly);
    prover.calculate_sum();
    
    println!("Sum: {:?}", prover.sum);
    
    let proof = prover.sum_check_proof();
    let mut verifer = Verifier::new();
    
    assert!(verifer.verify(&proof));

```
