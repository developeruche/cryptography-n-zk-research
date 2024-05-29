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
