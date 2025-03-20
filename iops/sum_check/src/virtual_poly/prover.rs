//! Prover module for the sumcheck protocol of the virtual polynomial.

use ark_ff::PrimeField;
use polynomial::virtual_polynomial::VirtualPolynomial;

#[derive(Clone, Default, Debug)]
pub struct VirtualProver<F: PrimeField> {
    /// sampled randomness given by the verifier
    pub challenges: Vec<F>,
    /// the current round number
    pub(crate) round: usize,
    /// pointer to the virtual polynomial
    pub(crate) poly: VirtualPolynomial<F>,
    /// points with precomputed barycentric weights for extrapolating smaller
    /// degree uni-polys to `max_degree + 1` evaluations.
    pub(crate) extrapolation_aux: Vec<(Vec<F>, Vec<F>)>,
}

impl<F: PrimeField> VirtualProverInterface<F> for VirtualProver {
    fn calculate_sum(poly: &polynomial::virtual_polynomial::VirtualPolynomial<F>) -> F {
        todo!()
    }

    fn sum_check_proof(
        poly: &polynomial::virtual_polynomial::VirtualPolynomial<F>,
        transcript: &mut fiat_shamir::FiatShamirTranscript,
        sum: &F,
    ) -> (crate::composed::VirtualSumCheckProof<F>, Vec<F>) {
        todo!()
    }
}
