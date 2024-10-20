//! This module contains the generic interfaces for the polynomial commitment scheme and the transcript.

use ark_ec::pairing::Pairing;
use ark_ff::PrimeField;
use kzg_rust::primitives::SRS;
use plonk_core::primitives::{PlonkProof, RoundOneOutput, Witness};
use polynomial::univariant::UnivariantPolynomial;

/// This is a generic interface for the transcript.
pub trait PlonkTranscriptInterface<F: PrimeField> {
    fn append(&mut self, label: &str, msg: Vec<u8>);
    fn sample(&mut self) -> [u8; 32];
    fn sample_n(&mut self, n: usize) -> Vec<[u8; 32]>;
    fn sample_as_field_element(&mut self) -> F;
    fn sample_n_as_field_elements(&mut self, n: usize) -> Vec<F>;
}

/// This is a generaic interface for the plonk prover
pub trait PlonkProverInterface<F: PrimeField, P: Pairing> {
    /// This function performs all the plonk protocol's 5 round and returns a proof.
    fn prove(&self, witness: Witness<F>) -> PlonkProof<F>;
    /// Plonk protocol round 1
    fn round_one(&self, witness: Witness<F>) -> RoundOneOutput<P>;
}
