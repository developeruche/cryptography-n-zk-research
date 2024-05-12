use ark_ff::Field;

use crate::interface::{PolynomialInterface, UnivariantPolynomialInterface};

pub struct UnivariantPolynomial<F> {
    pub coefficients: Vec<F>,
}


impl<F: Field> PolynomialInterface<F> for UnivariantPolynomial<F> {
    type Point = F;

    // This function returns the total degree of the polynomial
    fn degree(&self) -> usize {
        self.coefficients.len() - 1
    }

    /// This function evaluates the polynomial at a given point
    /// Example: 2x^2 + 3x + 1 ---> [1, 3, 2]
    /// evaluate(2) = 2*2^2 + 3*2 + 1 = 2*4 + 6 + 1 = 8 + 6 + 1 = 15
    /// evaluate(2) = 1 + 3*2 + 2*2^2 = 1 + 6 + 8 = 15
    /// evaluate(2) = 15
    fn evaluate(&self, x: &F) -> F {
        let mut sum = self.coefficients[0].clone();
        let mut varaiable = x.clone();

        for i in 1..self.coefficients.len() {
            sum += self.coefficients[i] * varaiable;
            varaiable *= varaiable;
        }

        sum
    }
}


impl<F: Field> UnivariantPolynomialInterface<F> for UnivariantPolynomial<F> {
    // This function creates a new polynomial from a list of coefficients vector
    fn from_coefficients_vec(coeffs: Vec<F>) -> Self {
        UnivariantPolynomial {
            coefficients: coeffs,
        }
    }

    // This function creates a new polynomial from a list of coefficients slice
    fn from_coefficients_slice(coeffs: &[F]) -> Self {
        UnivariantPolynomial {
            coefficients: coeffs.to_vec(),
        }
    }

    // This function returns an array of coefficients of this polynomial
    fn coefficients(&self) -> &[F] {
        &self.coefficients
    }

    // This function is used to create a new univariate polynomial using an interpolation [USING LAGRANGE INTERPOLATION]
    fn interpolate(point_xs: Vec<F>, domain: Vec<F>) -> Self {
        let mut coefficients = vec![F::zero(); point_xs.len()];

        // for i in 0..point_xs.len() {
        //     let mut numerator = F::one();
        //     let mut denominator = F::one();

        //     for j in 0..point_xs.len() {
        //         if i == j {
        //             continue;
        //         }

        //         numerator *= &domain[j];
        //         denominator *= &(domain[j] - &domain[i]);
        //     }

        //     let inverse_denominator = denominator.inverse().unwrap();
        //     let term = numerator * inverse_denominator;
        //     coefficients[i] = term;
        // }

        UnivariantPolynomial {
            coefficients
        }
    }
}


/// Implement the `Display` trait for `Polynomial` so that we can print it out.
impl<F: Field> std::fmt::Display for UnivariantPolynomial<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut result = String::new();

        for (i, coefficient) in self.coefficients.iter().enumerate() {
            if i == 0 {
                result.push_str(&format!("{:?}", coefficient));
            } else {
                result.push_str(&format!(" + {:?}x^{:?}", coefficient, i));
            }
        }

        write!(f, "{}", result)
    }
}

impl<F: Field> UnivariantPolynomial<F> {
    /// This function creates a new polynomial from a list of coefficients
    pub fn new(coefficients: Vec<F>) -> Self {
        UnivariantPolynomial {
            coefficients
        }
    }
}
