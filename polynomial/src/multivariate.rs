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
        // SANITY_CHECK: Ensure that the number of evaluations is equal to the number of variables raised to power of 2
        assert_eq!(
            evaluations.len(),
            hc_index.pow(num_vars as u32),
            "Number of evaluations must be equal to 2^num_vars"
        );

        Self {
            evaluations,
            num_vars,
            hc_index,
        }
    }

    /// This function is used to check if the polynomial is zero
    pub fn is_zero(&self) -> bool {
        self.evaluations.iter().all(|x| x.is_zero())
    }

    /// This function is used to return the bytes representation of the polynomial
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut m_ploy_bytes = Vec::new();

        for eval in &self.evaluations {
            let big_int = eval.into_bigint().to_bytes_be();
            m_ploy_bytes.extend_from_slice(&big_int);
        }

        m_ploy_bytes
    }
}
