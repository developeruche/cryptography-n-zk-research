use ark_ff::Field;

use crate::interface::{MultivariantPolynomialInterface, PolynomialInterface};

/// A multilinear polynomial over a field.
#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct Multilinear<F> {
    /// The number of variables in the polynomial.
    num_vars: usize,
    /// The evaluations of the polynomial at the different points.
    evaluations: Vec<F>,
}

impl<F: Field> Multilinear<F> {
    /// This function creates a new multilinear polynomial from a list of evaluations
    pub fn new(evaluations: Vec<F>, num_vars: usize) -> Self {
        // SANITY_CHECK: Ensure that the number of evaluations is equal to the number of variables raised to power of 2
        assert_eq!(
            evaluations.len(),
            1 << num_vars,
            "Number of evaluations must be equal to 2^num_vars"
        );
        Self {
            num_vars,
            evaluations,
        }
    }
}

/// Implement the PolynomialInterface for Multilinear
impl<F: Field> PolynomialInterface<F> for Multilinear<F> {
    /// The type of evaluation points for this polynomial.
    type Point = Vec<F>;

    /// Return the total degree of the polynomial
    fn degree(&self) -> usize {
        // I have not been able to figure out how to calculate the degree of a multilinear polynomial... YET!
        unimplemented!()
    }

    /// Evaluates `self` at the given `point` in `Self::Point`. this is done using partial evaluations
    fn evaluate(&self, point: &Self::Point) -> F {
        unimplemented!()
    }

    /// Checks if the polynomial is zero
    fn is_zero(&self) -> bool {
        self.evaluations.is_empty()
    }
}

impl<F: Field> MultivariantPolynomialInterface<F> for Multilinear<F> {
    /// This function returns the number of variables in the polynomial
    fn num_vars(&self) -> usize {
        self.num_vars
    }

    /// This function creates a new polynomial from a list of evaluations
    fn partial_evaluations(evaluations: Vec<F>, num_vars: usize) -> Self {
        unimplemented!()
    }
}
