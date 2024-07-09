use crate::primitives::CircuitEvaluation;
use ark_ff::PrimeField;
use polynomial::multilinear::Multilinear;
use std::ops::{Add, Mul};

/// This is the interface for the circuit
pub trait CircuitInterface {
    /// This function evaluates the circuit
    fn evaluate<F>(&self, input: &[F]) -> CircuitEvaluation<F>
    where
        F: Add<Output = F> + Mul<Output = F> + Copy;
}

/// This is the interface for the GKR protocol circuit
pub trait GKRProtocolCircuitInterface {
    /// This function returns the addition mle for a indicated layer
    fn get_add_n_mul_mle<F: PrimeField>(
        &self,
        layer_index: usize,
    ) -> (Multilinear<F>, Multilinear<F>);
}
