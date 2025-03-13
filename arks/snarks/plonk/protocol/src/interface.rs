//! This module contains the generic interfaces for the polynomial commitment scheme and the transcript.
use ark_ec::pairing::Pairing;
use ark_ff::PrimeField;
use plonk_core::primitives::{
    PlonkProof, RoundFiveOutput, RoundFourOutput, RoundOneOutput, RoundThreeOutput, RoundTwoOutput,
    Witness,
};
use polynomial::evaluation::univariate::UnivariateEval;

/// This is a generaic interface for the plonk prover
pub trait PlonkProverInterface<F: PrimeField, P: Pairing> {
    /// This function performs all the plonk protocol's 5 round and returns a proof.
    fn prove(&mut self, witness: &Witness<F>) -> PlonkProof<P, F>;
    /// Plonk protocol round 1
    fn round_one(&mut self, witness: &Witness<F>) -> RoundOneOutput<P, F>;
    /// Plonk protocol round 2
    fn round_two(&mut self, raw_witness: &Witness<F>) -> RoundTwoOutput<P, F>;
    /// Plonk protocol round 3
    fn round_three(
        &mut self,
        raw_witness: &Witness<F>,
        round_one_output: &RoundOneOutput<P, F>,
        round_two_output: &RoundTwoOutput<P, F>,
    ) -> RoundThreeOutput<P, F>;
    /// Plonk protocol round 4
    fn round_four(
        &mut self,
        round_one_output: &RoundOneOutput<P, F>,
        round_three_output: &RoundThreeOutput<P, F>,
    ) -> RoundFourOutput<F>;
    /// Plonk protocol round 5
    fn round_five(
        &mut self,
        witness: &Witness<F>,
        round_one_output: &RoundOneOutput<P, F>,
        round_two_output: &RoundTwoOutput<P, F>,
        round_three_output: &RoundThreeOutput<P, F>,
        round_four_output: &RoundFourOutput<F>,
    ) -> RoundFiveOutput<P, F>;
}

pub trait PlonkVerifierInterface<F: PrimeField, P: Pairing> {
    /// This function verifies the plonk proof.
    fn verify(&self, proof: &PlonkProof<P, F>, public_input: UnivariateEval<F>) -> bool;
}
