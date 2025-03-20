//! Verifier module for the sumcheck protocol of the virtual polynomial.
use ark_ff::PrimeField;

#[derive(Clone, Default, Debug)]
pub struct VirtualVerifier<F: PrimeField> {
    pub(crate) round: usize,
    pub(crate) num_vars: usize,
    pub(crate) max_degree: usize,
    pub(crate) finished: bool,
    /// a list storing the univariate polynomial in evaluation form sent by the
    /// prover at each round
    pub(crate) polynomials_received: Vec<Vec<F>>,
    /// a list storing the randomness sampled by the verifier at each round
    pub(crate) challenges: Vec<F>,
}
