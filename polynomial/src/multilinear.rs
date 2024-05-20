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



/// Implement the PolynomialInterface for Multilinear
impl <F: Field> PolynomialInterface<F> for Multilinear<F> {
    /// The type of evaluation points for this polynomial.
    type Point = Vec<F>;

    /// Return the total degree of the polynomial
    fn degree(&self) -> usize {
        unimplemented!()
    }

    /// Evaluates `self` at the given `point` in `Self::Point`. this is done using partial evaluations
    fn evaluate(&self, point: &Self::Point) -> F {
        unimplemented!()
    }

    /// Checks if the polynomial is zero
    fn is_zero(&self) -> bool {
        unimplemented!()
    }
}

impl <F: Field> MultivariantPolynomialInterface<F> for Multilinear<F> {
    /// This function returns the number of variables in the polynomial
    fn num_vars(&self) -> usize {
        unimplemented!()
    }

    /// This function returns the evaluations of the polynomial at the different points
    fn evaluations(&self) -> &[F] {
        unimplemented!()
    }

    /// This function creates a new polynomial from a list of evaluations
    fn partial_evaluations(evaluations: Vec<F>, num_vars: usize) -> Self {
        unimplemented!()
    }
}
