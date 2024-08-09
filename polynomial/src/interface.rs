use ark_ff::PrimeField;

/// Describes the common interface for univariate and multivariate polynomials
/// This F generic parameter should be a field
pub trait PolynomialInterface<F: PrimeField> {
    /// The type of evaluation points for this polynomial.
    /// this could be a set of real numbers or roots of unity depending on the intrepolation logic
    type Point;

    /// Return the total degree of the polynomial
    fn degree(&self) -> usize;

    /// Evaluates `self` at the given `point` in `Self::Point`.
    fn evaluate(&self, point: &Self::Point) -> F;

    /// Checks if the polynomial is zero
    fn is_zero(&self) -> bool;
}

pub trait UnivariantPolynomialInterface<F: PrimeField>: PolynomialInterface<F> {
    /// This function returs an array of co-efficents of this polynomial
    fn coefficients(&self) -> &[F];
    /// This function createsa new polynomial from a list of coefficients slice
    fn from_coefficients_slice(coeffs: &[F]) -> Self;
    /// This function creates a new polynomial from a list of coefficients vector
    fn from_coefficients_vec(coeffs: Vec<F>) -> Self;
    /// This function is used to create a new univariate polynomial using an interpolation
    fn interpolate(point_ys: Vec<F>, domain: Vec<F>) -> Self;
}

pub trait MultilinearPolynomialInterface<F: PrimeField> {
    /// This function returns the number of variables in the polynomial
    fn num_vars(&self) -> usize;
    /// This function creates a new polynomial from a list of evaluations
    fn partial_evaluation(&self, evaluation_point: F, variable_index: usize) -> Self;
    /// This function allows for multiple parial evaluations
    fn partial_evaluations(&self, evaluation_points: Vec<F>, variable_indices: Vec<usize>) -> Self;
    /// This function is used to evaluate the polynomial at a given point
    fn evaluate(&self, point: &Vec<F>) -> Option<F>;
    /// Extend polynomials with new variables
    /// given f(x,y) = 2xy + 3y + 4x + 5 this function can extend it to f(x,y,z) = 2xy + 3y + 4x + 5 + 0z
    fn extend_with_new_variables(&self, num_of_new_variables: usize) -> Self;
    /// Addition for multilinear polynomials with 2 distinict variables
    /// given f(x,y) = 2xy + 3y + 4x + 5 and f(a,b) = 2ab + 3b + 4a + 5
    /// f(x,y) + f(a,b) = 2xy + 3y + 4x + 5 + 2ab + 3b + 4a + 5
    fn add_distinct(&self, rhs: &Self) -> Self;
    /// Multiplication for multilinear polynomials with 2 distinict variables
    /// given f(x,y) = 2xy + 3y + 4x + 5 and f(a,b) = 2ab + 3b + 4a + 5
    /// f(x,y) * f(a,b) = 4xyab + 6yab + 8xab + 10ab + 6y + 8x + 10
    fn mul_distinct(&self, rhs: &Self) -> Self;
    /// Interpolation for multilinear polynomials with 2 distinict variables
    fn interpolate(y_s: &[F]) -> Self;
    /// This function returns the additive identity of the polynomial
    fn zero(num_vars: usize) -> Self;
    /// This function is used to check if the polynomial is zero
    fn is_zero(&self) -> bool;
    /// This function performs `Add` but in contexct of the type
    fn internal_add(&self, rhs: &Self) -> Self;
    /// This function performs `AddAssign` but in contexct of the type
    fn internal_add_assign(&mut self, rhs: &Self);
    /// This function is used to return the bytes representation of the polynomial
    fn to_bytes(&self) -> Vec<u8>;
}

pub trait MultivariatePolynomialInterface<F: PrimeField> {
    /// This function returns the number of variables in the polynomial
    fn num_vars(&self) -> usize;
    /// This function returns the operational hypercube index
    fn hc_index(&self) -> usize;
    /// This function is used to evaluate the polynomial at a given point
    fn evaluate(&self, point: &Vec<F>) -> Option<F>;
    /// This function creates a new polynomial from a list of evaluations
    fn partial_evaluation(&self, evaluation_point: F, variable_index: usize) -> Self;
    /// This function allows for multiple parial evaluations
    fn partial_evaluations(&self, evaluation_points: Vec<F>, variable_indices: Vec<usize>) -> Self;
}
