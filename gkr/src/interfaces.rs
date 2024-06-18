use crate::primitives::GKRProof;
use ark_ff::PrimeField;
use circuits::primitives::{Circuit, CircuitEvaluation};

/// This is the interface for the GKR protocol
pub trait GKRProtocolInterface<F: PrimeField> {
    /// This function is used to create GKR proofs
    fn prove(circuit: &Circuit, evals: &CircuitEvaluation<F>) -> GKRProof<F>;

    /// This function is used to verify GKR proofs
    fn verify(circuit: &Circuit, input: &[F], proof: &GKRProof<F>) -> bool;
}
