//! This is file contains implementation if a univariate polynomial interpolated over the
//! domain of the root of unity, this have the capacity to port coeffient form poly
//! to evaluation form and evaluation form poly to coeffiecient form, this operations
//! would be done uisng the FFT and IFFT algorithm.
//!
//! This is modeled after arkworks implemenation.
use crate::univariant::UnivariantPolynomial;
use ark_ff::PrimeField;

use super::Domain;

pub struct UnivariateEval<F: PrimeField> {
    /// this is a list of the evaluation of the polynomial
    pub values: Vec<F>,
    /// This is the domian of the polynomal; very important for the FFT and IFFT
    domain: Domain<F>,
}

impl<F: PrimeField> UnivariateEval<F> {
    /// This function is used to create a new polynomial from the evaluation form
    pub fn new(values: Vec<F>, domain: Domain<F>) -> Self {
        UnivariateEval { values, domain }
    }

    /// This function is used to convert the coefficient form of the polynomial to the evaluation form
    pub fn from_coefficients(coefficients: Vec<F>) -> Self {
        todo!()
    }

    /// This function is used to convert the evaluation form of the polynomial to the coefficient form
    pub fn to_coefficients(&self) -> Vec<F> {
        todo!()
    }

    /// This function is used to convert the evaluation form of the polynomial to the coefficient form as a polynomial
    pub fn to_coefficient_poly(&self) -> UnivariantPolynomial<F> {
        todo!()
    }

    /// This function is used to multiply two polynomials in the evaluation form
    pub fn multiply(&self, other: &Self) -> Self {
        todo!()
    }
}
