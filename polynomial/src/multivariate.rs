use ark_ff::{BigInteger, PrimeField};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use std::ops::{Add, AddAssign};

/// A multivariate polynomial.
#[derive(Clone, PartialEq, Eq, Hash, Default, Debug, CanonicalSerialize, CanonicalDeserialize)]
pub struct Multivariate<F: PrimeField> {
    /// The coefficients of the polynomial.
    pub evaluations: Vec<F>,
    /// This is the number of variables in the polynomial.
    pub num_vars: usize,
    /// This is the index of the operationals hypercube
    pub hc_index: usize,
}

impl<F: PrimeField> Multivariate<F> {
    /// This is the function for creating a new multivariate polynomial.
    pub fn new(evaluations: Vec<F>, num_vars: usize, hc_index: usize) -> Self {
        Self {
            evaluations,
            num_vars,
            hc_index,
        }
    }
}