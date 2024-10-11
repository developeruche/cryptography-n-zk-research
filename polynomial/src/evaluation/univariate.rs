//! This is file contains implementation if a univariate polynomial interpolated over the
//! domain of the root of unity, this have the capacity to port coeffient form poly
//! to evaluation form and evaluation form poly to coeffiecient form, this operations
//! would be done uisng the FFT and IFFT algorithm.

use ark_ff::PrimeField;

use crate::univariant::UnivariantPolynomial;

pub struct UnivariateEval<F: PrimeField> {
    /// this is a list of the evaluation of the polynomial
    pub values: Vec<F>,
}

impl<F: PrimeField> UnivariateEval<F> {
    /// This function is used to create a new polynomial from the evaluation form
    pub fn new(values: Vec<F>) -> Self {
        UnivariateEval { values }
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
}
