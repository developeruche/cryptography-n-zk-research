use ark_ff::Field;
use polynomial::multilinear::Multilinear;
use crate::data_structure::SumCheckProof;



/// This trait is used to define the prover interface
pub trait ProverInterface<F: Field> {
    /// This function returns the sum of the multilinear polynomial evaluation over the boolean hypercube.
    fn calculate_sum(&self, poly: &Multilinear<F>) -> F;
    /// This function returns the round zero computed polynomial
    fn round_zero_poly(&self) -> Multilinear<F>;
    /// This function returns poly cimouted in round j
    fn round_j_poly(&self, j: usize) -> Multilinear<F>;
    /// This function computes sum check proof
    fn sum_check_proof(&self) -> SumCheckProof<F>;
}


/// The verifier interface is used to verify the sum check proof
pub trait VerifierInterface<F: Field> {
    /// This function verifies the sum check proof
    fn verify(&self, proof: &SumCheckProof<F>) -> bool;
}