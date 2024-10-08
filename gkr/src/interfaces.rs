use crate::primitives::{GKRProof, SuccinctGKRProof};
use ark_ec::pairing::Pairing;
use ark_ff::PrimeField;
use circuits::primitives::{Circuit, CircuitEvaluation};
use kzg_rust::primitives::MultiLinearSRS;

/// This is the interface for the GKR protocol
pub trait GKRProtocolInterface<F: PrimeField> {
    /// This function is used to create GKR proofs
    fn prove(circuit: &Circuit, evals: &CircuitEvaluation<F>) -> GKRProof<F>;

    /// This function is used to verify GKR proofs
    fn verify(circuit: &Circuit, input: &[F], proof: &GKRProof<F>) -> bool;
}

/// This is the interface for the GKR protocol
pub trait SuccinctGKRProtocolInterface<F: PrimeField, P: Pairing> {
    /// This function is used to create GKR proofs
    fn prove(
        circuit: &Circuit,
        evals: &CircuitEvaluation<F>,
        input_poly_commitment: &P::G1,
        srs: &MultiLinearSRS<P>,
    ) -> SuccinctGKRProof<F, P>;

    /// This function is used to verify GKR proofs
    fn verify(
        circuit: &Circuit,
        proof: &SuccinctGKRProof<F, P>,
        input_poly_commitment: &P::G1,
        srs: &MultiLinearSRS<P>,
    ) -> bool;
}
