# Sum check protocol
--------------------

Suppose we are given a v-variate polynomial g defined over a finite field F. The purpose of the sum-check protocol is for the prover to provide the verifier with the sum of evaluations over the boolean hypercube.

### Structures

Sum Check Proof
```rust 
	#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
	pub struct SumCheckProof<F: PrimeField> {
	    /// This is the polynomial that is used to generate the sum check proof
	    pub polynomial: Multilinear<F>,
	    /// This vector stores the round polynomials
	    pub round_poly: Vec<Multilinear<F>>,
	    /// This vectors store the polynomial from the first round
	    pub round_0_poly: Multilinear<F>,
	    /// This holds the sum of the polynomial evaluation over the boolean hypercube
	    pub sum: F,
	}
```

Prover 

```rust 
#[derive(Clone, Default, Debug)]
pub struct Prover<F: PrimeField> {
    /// This is the polynomial to calculate the sum check proof
    pub poly: Multilinear<F>,
    /// This struct is used to store the sum check proof
    pub round_poly: Vec<Multilinear<F>>,
    /// This vectors store the polynomial from the first round
    pub round_0_poly: Multilinear<F>,
    /// This holds the sum of the polynomial evaluation over the boolean hypercube
    pub sum: F,
    /// This is this fiat-shamir challenge transcript
    pub transcript: FiatShamirTranscript,
}
```

Verifier 

```rust 
#[derive(Clone, Default, Debug)]
pub struct Verifier<F: PrimeField> {
    /// This is this fiat-shamir challenge transcript
    pub transcript: FiatShamirTranscript,
    phantom: std::marker::PhantomData<F>,
}

```

Interfaces

```rust 
use crate::data_structure::SumCheckProof;
use ark_ff::PrimeField;

/// This trait is used to define the prover interface
pub trait ProverInterface<F: PrimeField> {
    /// This function returns the sum of the multilinear polynomial evaluation over the boolean hypercube.
    fn calculate_sum(&mut self);
    /// This function returns the round zero computed polynomial
    fn compute_round_zero_poly(&mut self);
    /// This function computes sum check proof
    fn sum_check_proof(&mut self) -> SumCheckProof<F>;
}

/// The verifier interface is used to verify the sum check proof
pub trait VerifierInterface<F: PrimeField> {
    /// This function verifies the sum check proof
    fn verify(&mut self, proof: &SumCheckProof<F>) -> bool;
}

```
