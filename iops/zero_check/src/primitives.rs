//! This file contains the primitives used in the zero check protocol.

use ark_ff::PrimeField;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ZeroCheckSubClaim<F: PrimeField> {
    /// the evaluuation point
    pub points: Vec<F>,
    /// Expected evaluation
    pub expected_points: F,
    // the initial challenge r which is used to build eq(x, r)
    pub initial_challenge: Vec<F>,
}
