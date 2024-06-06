use std::ops::{Add, Mul};

use crate::primitives::{Circuit, CircuitEvaluation, GKRProof};
use ark_ff::PrimeField;

/// This is the interface for the circuit
pub trait CircuitInterface {
    /// This function evaluates the circuit
    fn evaluate<F>(&self, input: &[F]) -> CircuitEvaluation<F>
    where
        F: Add<Output = F> + Mul<Output = F> + Copy;
}

/// This is the interface for the GKR protocol
pub trait GKRProtocolInterface<F: PrimeField> {
    /// This function is used to create GKR proofs
    fn prove(circuit: &Circuit, evals: &CircuitEvaluation<F>) -> GKRProof<F>;

    /// This function is used to verify GKR proofs
    fn verify(circuit: &Circuit, input: &[F], proof: &GKRProof<F>) -> bool;
}
