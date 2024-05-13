use std::ops::{Add, Mul};

use ark_ff::Field;

use crate::{interface::{PolynomialInterface, UnivariantPolynomialInterface}, utils::get_langrange_basis};


#[derive(Clone, PartialEq, Eq, Hash, Default)]
pub struct UnivariantPolynomial<F> {
    pub coefficients: Vec<F>,
}


impl<F: Field> PolynomialInterface<F> for UnivariantPolynomial<F> {
    type Point = F;

    /// This function returns the total degree of the polynomial
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
    /// This function creates a new polynomial from a list of coefficients vector
    fn from_coefficients_vec(coeffs: Vec<F>) -> Self {
        UnivariantPolynomial {
            coefficients: coeffs,
        }
    }

    /// This function creates a new polynomial from a list of coefficients slice
    fn from_coefficients_slice(coeffs: &[F]) -> Self {
        UnivariantPolynomial {
            coefficients: coeffs.to_vec(),
        }
    }

    /// This function returns an array of coefficients of this polynomial
    fn coefficients(&self) -> &[F] {
        &self.coefficients
    }

    /// This function is used to create a new univariate polynomial using an interpolation [USING LAGRANGE INTERPOLATION]
    /// params: point_ys: Vec<F> - a list of y values
    /// params: domain: Vec<F> - a list of x values
    fn interpolate(point_ys: Vec<F>, domain: Vec<F>) -> Self {
        let mut coefficients = UnivariantPolynomial::default();

        // Get the langrange basis
        let langrange_basis = get_langrange_basis(&domain);
        coefficients = point_ys.iter().zip(langrange_basis.iter()).map(|(y, basis)| y * basis);

        coefficients
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

// impl<F: Field> Mul for UnivariantPolynomial<F> {
//     type Output = Self;

//     fn mul(self, other: Self) -> Self {
//         let mut result = UnivariantPolynomial::default();

//         for i in 0..self.coefficients.len() {
//             for j in 0..other.coefficients.len() {
//                 let mut new_coefficient = self.coefficients[i] * other.coefficients[j];
//                 let mut new_degree = i + j;

//                 result.coefficients[new_degree] += new_coefficient;
//             }
//         }

//         result
//     }
// }

impl<F: Field> Add for UnivariantPolynomial<F> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let result = if self.degree() >= other.degree() {
            let mut result_coff = self.coefficients.clone();

            for i in 0..self.coefficients.len() {
                result_coff.push(self.coefficients[i] + other.coefficients.get(i).unwrap_or(&F::zero()));
            }

            UnivariantPolynomial::from_coefficients_vec(result_coff)
        } else {
            let mut result_coff = self.coefficients.clone();

            for i in 0..other.coefficients.len() {
                result_coff.push(other.coefficients[i] + self.coefficients.get(i).unwrap_or(&F::zero()));
            }

            UnivariantPolynomial::from_coefficients_vec(result_coff)
        };

        result
    }
}
