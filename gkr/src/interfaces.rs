use std::ops::{Add, Mul};

use ark_ff::PrimeField;
use crate::primitives::CircuitEvaluation;


pub trait CircuitInterface {
    /// This function evaluates the circuit
    fn evaluate<F>(&self, input: &[F]) -> CircuitEvaluation<F>
    where
        F: Add<Output = F> + Mul<Output = F> + Copy;
}