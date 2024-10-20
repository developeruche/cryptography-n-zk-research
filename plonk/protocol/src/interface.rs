//! This module contains the generic interfaces for the polynomial commitment scheme and the transcript.

use ark_ec::pairing::Pairing;
use ark_ff::PrimeField;
use plonk_core::primitives::{PlonkProof, Witness};
use polynomial::univariant::UnivariantPolynomial;

/// This is a generic interface for the transcript.
pub trait PlonkTranscriptInterface<F: PrimeField> {
    fn append(&mut self, label: &str, msg: Vec<u8>);
    fn sample(&mut self) -> [u8; 32];
    fn sample_n(&mut self, n: usize) -> Vec<[u8; 32]>;
    fn sample_as_field_element(&mut self) -> F;
    fn sample_n_as_field_elements(&mut self, n: usize) -> Vec<F>;
}

/// This is a generic interface for the polynomial commitment scheme.
pub trait PlonkPCSInterface<P: Pairing> {
    type SRS;
    /// This is used to commit to a polynomial.
    fn commit(srs: &Self::SRS, poly: &UnivariantPolynomial<P::ScalarField>) -> P::G1;
    /// This is used to open a polynomial at a point.
    fn open<F: PrimeField>(
        srs: &Self::SRS,
        poly: &UnivariantPolynomial<F>,
        point: &F,
    ) -> (F, P::G1);
    /// This is used to verify a polynomial commitment.
    fn verify<F: PrimeField>(
        srs: &Self::SRS,
        commitment: &P::G1,
        point: &F,
        point_evaluation: &F,
        proof: &P::G1,
    ) -> bool;
}

/// This is a generaic interface for the plonk prover
pub trait PlonkProverInterface<F: PrimeField, P: Pairing> {
    /// This function performs all the plonk protocol's 5 round and returns a proof.
    fn prove(&self, witness: &Witness<F>) -> PlonkProof<F>;
}
