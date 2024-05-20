use crate::{
    interface::{PolynomialInterface, UnivariantPolynomialInterface},
    utils::get_langrange_basis,
};
use ark_ff::Field;
pub use ark_test_curves;
use std::ops::{Add, AddAssign, Mul};

#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct UnivariantPolynomial<F> {
    /// This is the co-coefficients of the polynomial
    pub coefficients: Vec<F>,
}

impl<F: Field> PolynomialInterface<F> for UnivariantPolynomial<F> {
    type Point = F;

    /// This function returns the total degree of the polynomial
    fn degree(&self) -> usize {
        if self.coefficients.is_empty() {
            0
        } else {
            self.coefficients.len() - 1
        }
    }

    /// This function evaluates the polynomial at a given point
    /// Example: 2x^2 + 3x + 1 ---> [1, 3, 2]
    /// evaluate(2) = 2*2^2 + 3*2 + 1 = 2*4 + 6 + 1 = 8 + 6 + 1 = 15
    /// evaluate(2) = 1 + 3*2 + 2*2^2 = 1 + 6 + 8 = 15
    /// evaluate(2) = 15
    fn evaluate(&self, x: &F) -> F {
        self.coefficients
            .iter()
            .rev()
            .fold(F::zero(), |acc, coeff| acc * x + coeff)
    }

    fn is_zero(&self) -> bool {
        self.coefficients.is_empty()
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
        let langrange_poly_vec = get_langrange_basis(&domain, &point_ys);
        let langrange_poly = langrange_poly_vec
            .iter()
            .fold(UnivariantPolynomial::new(vec![]), |acc, x| acc + x.clone());

        langrange_poly
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
        UnivariantPolynomial { coefficients }
    }
}

impl<F: Field> Mul for UnivariantPolynomial<F> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        // check for zero polynomials
        if self.is_zero() || other.is_zero() {
            return UnivariantPolynomial::new(vec![]);
        }

        // Create a new polynomial with the degree of the two polynomials
        let poly_product_degree = self.degree() + other.degree();

        // during poly mul we would need d + 1 element to represent a polynomial of degree d
        let mut poly_product_coefficients = vec![F::zero(); poly_product_degree + 1];

        for i in 0..=self.degree() {
            for j in 0..=other.degree() {
                poly_product_coefficients[i + j] += self.coefficients[i] * other.coefficients[j];
            }
        }

        UnivariantPolynomial::new(poly_product_coefficients)
    }
}

impl<F: Field> Mul for &UnivariantPolynomial<F> {
    type Output = UnivariantPolynomial<F>;

    fn mul(self, other: Self) -> Self::Output {
        // check for zero polynomials
        if self.is_zero() || other.is_zero() {
            return UnivariantPolynomial::new(vec![]);
        }

        // Create a new polynomial with the degree of the two polynomials
        let poly_product_degree = self.degree() + other.degree();

        // during poly mul we would need d + 1 element to represent a polynomial of degree d
        let mut poly_product_coefficients = vec![F::zero(); poly_product_degree + 1];

        for i in 0..=self.degree() {
            for j in 0..=other.degree() {
                poly_product_coefficients[i + j] += self.coefficients[i] * other.coefficients[j];
            }
        }

        UnivariantPolynomial::new(poly_product_coefficients)
    }
}

impl<F: Field> Add for UnivariantPolynomial<F> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let result = if self.degree() >= other.degree() {
            let mut result_coff = Vec::new();

            for i in 0..self.coefficients.len() {
                result_coff
                    .push(self.coefficients[i] + other.coefficients.get(i).unwrap_or(&F::zero()));
            }

            UnivariantPolynomial::from_coefficients_vec(result_coff)
        } else {
            let mut result_coff = Vec::new();

            for i in 0..other.coefficients.len() {
                result_coff
                    .push(other.coefficients[i] + self.coefficients.get(i).unwrap_or(&F::zero()));
            }

            UnivariantPolynomial::from_coefficients_vec(result_coff)
        };

        result
    }
}

impl<F: Field> AddAssign for UnivariantPolynomial<F> {
    fn add_assign(&mut self, rhs: Self) {
        if self.degree() >= rhs.degree() {
            for i in 0..self.coefficients.len() {
                self.coefficients[i] += rhs.coefficients.get(i).unwrap_or(&F::zero());
            }
        } else {
            let mut result_coff = self.coefficients.clone();

            for i in 0..rhs.coefficients.len() {
                result_coff
                    .push(rhs.coefficients[i] + self.coefficients.get(i).unwrap_or(&F::zero()));
            }

            self.coefficients = result_coff;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_poly::{evaluations::multivariate::multilinear, multivariate};
    use ark_test_curves::bls12_381::Fr;

    #[test]
    fn test_univariant_polynomial_addtion() {
        let poly1 = UnivariantPolynomial::new(vec![Fr::from(1), Fr::from(2), Fr::from(3)]);
        let poly2 = UnivariantPolynomial::new(vec![Fr::from(1), Fr::from(2), Fr::from(3)]);
        let poly3 = poly1.clone() + poly2.clone();

        assert_eq!(
            poly3.coefficients,
            vec![Fr::from(2), Fr::from(4), Fr::from(6)]
        );
    }

    #[test]
    fn test_univariant_polynomial_addtion_assign() {
        let mut poly1 = UnivariantPolynomial::new(vec![Fr::from(1), Fr::from(2), Fr::from(3)]);
        let poly2 = UnivariantPolynomial::new(vec![Fr::from(1), Fr::from(2), Fr::from(3)]);
        poly1 += poly2.clone();

        assert_eq!(
            poly1.coefficients,
            vec![Fr::from(2), Fr::from(4), Fr::from(6)]
        );
    }

    #[test]
    fn test_univariant_polynomial_multiplication() {
        let poly1 = UnivariantPolynomial::new(vec![Fr::from(1), Fr::from(3), Fr::from(2)]);
        let poly2 = UnivariantPolynomial::new(vec![Fr::from(3), Fr::from(2)]);

        let poly3 = poly1.clone() * poly2.clone();
        assert_eq!(
            poly3.coefficients,
            vec![Fr::from(3), Fr::from(11), Fr::from(12), Fr::from(4)]
        );
    }

    #[test]
    fn test_univariant_polynomial_multiplication_scalar() {
        let poly1 = UnivariantPolynomial::new(vec![Fr::from(1), Fr::from(3), Fr::from(2)]);
        let poly2 = UnivariantPolynomial::new(vec![Fr::from(3)]);

        let poly3 = poly1.clone() * poly2.clone();
        assert_eq!(
            poly3.coefficients,
            vec![Fr::from(3), Fr::from(9), Fr::from(6)]
        );
    }

    #[test]
    fn test_univariant_polynomial_interpolation_1() {
        let point_ys = vec![Fr::from(0), Fr::from(4), Fr::from(16)];
        let domain = vec![Fr::from(0), Fr::from(2), Fr::from(4)];

        let poly = UnivariantPolynomial::interpolate(point_ys, domain);
        assert_eq!(
            poly.coefficients,
            vec![Fr::from(0), Fr::from(0), Fr::from(1)]
        );
    }

    #[test]
    fn test_univariant_polynomial_interpolation_2() {
        let point_ys = vec![Fr::from(5), Fr::from(7), Fr::from(13)];
        let domain = vec![Fr::from(0), Fr::from(1), Fr::from(2)];

        let poly = UnivariantPolynomial::interpolate(point_ys, domain);
        assert_eq!(
            poly.coefficients,
            vec![Fr::from(5), Fr::from(0), Fr::from(2)]
        );
    }

    #[test]
    fn test_univariant_polynomial_interpolation_3() {
        // fq_from_vec(vec![0, 1, 3, 4, 5, 8]),
        // fq_from_vec(vec![12, 48, 3150, 11772, 33452, 315020]),
        let point_ys = vec![
            Fr::from(12),
            Fr::from(48),
            Fr::from(3150),
            Fr::from(11772),
            Fr::from(33452),
            Fr::from(315020),
        ];
        let domain = vec![
            Fr::from(0),
            Fr::from(1),
            Fr::from(3),
            Fr::from(4),
            Fr::from(5),
            Fr::from(8),
        ];

        let poly = UnivariantPolynomial::interpolate(point_ys, domain);
        let eval = poly.evaluate(&Fr::from(3));
        println!("{:?}", eval);
        assert_eq!(
            poly.coefficients,
            vec![
                Fr::from(12),
                Fr::from(8),
                Fr::from(1),
                Fr::from(7),
                Fr::from(12),
                Fr::from(8)
            ]
        );
    }

    #[test]
    fn test_univariant_polynomial_interpolation_4() {
        let point_ys = vec![Fr::from(565), Fr::from(1631), Fr::from(3537), Fr::from(-7)];
        let domain = vec![Fr::from(5), Fr::from(7), Fr::from(9), Fr::from(1)];

        let poly = UnivariantPolynomial::interpolate(point_ys, domain);
        assert_eq!(
            poly.coefficients,
            vec![Fr::from(0), Fr::from(-12), Fr::from(0), Fr::from(5)]
        );
    }
}
