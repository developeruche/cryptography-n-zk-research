use crate::data_structure::SumCheckProof;
use ark_ff::PrimeField;
use fiat_shamir::FiatShamirTranscript;
use polynomial::multilinear::Multilinear;

/// This trait is used to define the prover interface
pub trait ProverInterface<F: PrimeField> {
    /// This function returns the sum of the multilinear polynomial evaluation over the boolean hypercube.
    fn calculate_sum(poly: &Multilinear<F>) -> F;
    /// This function returns the round zero computed polynomial
    fn compute_round_zero_poly(
        poly: &Multilinear<F>,
        transcript: &mut FiatShamirTranscript,
    ) -> Multilinear<F>;
    /// This function computes sum check proof
    fn sum_check_proof(
        poly: &Multilinear<F>,
        transcript: &mut FiatShamirTranscript,
        sum: &F,
    ) -> SumCheckProof<F>;
}

/// The verifier interface is used to verify the sum check proof
pub trait VerifierInterface<F: PrimeField> {
    /// This function verifies the sum check proof
    fn verify(proof: &SumCheckProof<F>) -> bool;
}
