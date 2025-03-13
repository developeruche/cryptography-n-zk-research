use crate::{
    composed::{multicomposed::IntermidateClaimCheck, ComposedSumCheckProof},
    data_structure::SumCheckProof,
};
use ark_ff::PrimeField;
use fiat_shamir::FiatShamirTranscript;
use polynomial::{
    composed::multilinear::ComposedMultilinear, interface::MultilinearPolynomialInterface,
    multilinear::Multilinear, univariant::UnivariantPolynomial,
};

/// This trait is used to define the prover interface
pub trait ProverInterface<F: PrimeField> {
    /// This function returns the sum of the multilinear polynomial evaluation over the boolean hypercube.
    fn calculate_sum(poly: &Multilinear<F>) -> F;
    /// This function returns the round zero computed polynomial
    fn compute_round_zero_poly<P: MultilinearPolynomialInterface<F>>(
        poly: &P,
        transcript: &mut FiatShamirTranscript,
    ) -> P;
    /// This function computes sum check proof
    fn sum_check_proof<P: MultilinearPolynomialInterface<F> + Clone>(
        poly: &P,
        transcript: &mut FiatShamirTranscript,
        sum: &F,
    ) -> SumCheckProof<F, P>;
}

/// The verifier interface is used to verify the sum check proof
pub trait VerifierInterface<F: PrimeField> {
    /// This function verifies the sum check proof
    fn verify<P: MultilinearPolynomialInterface<F> + Clone>(proof: &SumCheckProof<F, P>) -> bool;
}

/// This trait is used to define the composed prover interface
pub trait ComposedProverInterface<F: PrimeField> {
    /// This function returns the sum of the multilinear polynomial evaluation over the boolean hypercube.
    fn calculate_sum(poly: &ComposedMultilinear<F>) -> F;
    /// This function returns the round zero computed polynomial
    fn compute_round_zero_poly(
        poly: &ComposedMultilinear<F>,
        transcript: &mut FiatShamirTranscript,
    ) -> UnivariantPolynomial<F>;
    /// This function computes sum check proof
    fn sum_check_proof(
        poly: &ComposedMultilinear<F>,
        transcript: &mut FiatShamirTranscript,
        sum: &F,
    ) -> (ComposedSumCheckProof<F>, Vec<F>);
}

/// The verifier interface is used to verify the sum check proof
pub trait ComposedVerifierInterface<F: PrimeField> {
    /// This function verifies the sum check proof
    fn verify(proof: &ComposedSumCheckProof<F>, poly: &ComposedMultilinear<F>) -> bool;
}

/// This trait is used to define the multi-composed prover interface
pub trait MultiComposedProverInterface<F: PrimeField> {
    /// This function returns the sum of the multilinear polynomial evaluation over the boolean hypercube.
    fn calculate_sum(poly: &[ComposedMultilinear<F>]) -> F;
    /// This function computes sum check proof
    fn sum_check_proof(
        poly: &[ComposedMultilinear<F>],
        sum: &F,
    ) -> (ComposedSumCheckProof<F>, Vec<F>);
    /// This function computes sum check proof
    fn sum_check_proof_without_initial_polynomial(
        poly: &[ComposedMultilinear<F>],
        sum: &F,
    ) -> (ComposedSumCheckProof<F>, Vec<F>);
    fn sum_check_proof_internal(
        poly_: &[ComposedMultilinear<F>],
        transcript: &mut FiatShamirTranscript,
        sum: &F,
    ) -> (ComposedSumCheckProof<F>, Vec<F>);
}

/// The verifier interface is used to verify the sum check proof
pub trait MultiComposedVerifierInterface<F: PrimeField> {
    /// This function verifies the sum check proof
    fn verify(proof: &ComposedSumCheckProof<F>, poly: &[ComposedMultilinear<F>]) -> bool;
    fn verify_except_last_check(proof: &ComposedSumCheckProof<F>) -> IntermidateClaimCheck<F>;
    fn verify_internal(
        proof: &ComposedSumCheckProof<F>,
        transcript: &mut FiatShamirTranscript,
    ) -> Result<IntermidateClaimCheck<F>, &'static str>;
}
