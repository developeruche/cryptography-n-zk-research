//! This is file contains implementation if a univariate polynomial interpolated over the
//! domain of the root of unity, this have the capacity to port coeffient form poly
//! to evaluation form and evaluation form poly to coeffiecient form, this operations
//! would be done uisng the FFT and IFFT algorithm.
//!
//! This is modeled after arkworks implemenation.
use crate::univariant::UnivariantPolynomial;
use ark_ff::PrimeField;

use super::Domain;


#[derive(Debug, Clone)]
pub struct UnivariateEval<F: PrimeField> {
    /// this is a list of the evaluation of the polynomial
    pub values: Vec<F>,
    /// This is the domian of the polynomal; very important for the FFT and IFFT
    pub domain: Domain<F>,
}

impl<F: PrimeField> UnivariateEval<F> {
    /// This function is used to create a new polynomial from the evaluation form
    pub fn new(values: Vec<F>, domain: Domain<F>) -> Self {
        UnivariateEval { values, domain }
    }

    /// This function is used to convert the coefficient form of the polynomial to the evaluation form
    pub fn from_coefficients(coefficients: Vec<F>) -> Self {
        let mut coeffs = coefficients.clone();
        let domain = Domain::<F>::new(coefficients.len() as usize);
        let evals = domain.fft(&mut coeffs);

        UnivariateEval {
            values: evals,
            domain,
        }
    }

    /// This function is used to convert the evaluation form of the polynomial to the coefficient form
    pub fn to_coefficients(&self) -> Vec<F> {
        let evals = self.values.clone();
        self.domain.ifft(&evals)
    }

    /// This function is used to convert the evaluation form of the polynomial to the coefficient form as a polynomial
    pub fn to_coefficient_poly(&self) -> UnivariantPolynomial<F> {
        let coefficients = self.to_coefficients();
        UnivariantPolynomial::new(coefficients)
    }

    /// This function is used to multiply two polynomials in the evaluation form
    pub fn multiply(
        poly1: &UnivariantPolynomial<F>,
        poly2: &UnivariantPolynomial<F>,
    ) -> UnivariantPolynomial<F> {
        let mut poly1_coeffs = poly1.coefficients.clone();
        let mut poly2_coeffs = poly2.coefficients.clone();

        let length_of_poly_unscaled = poly1_coeffs.len() + poly2_coeffs.len() - 1;
        let length_of_poly = if length_of_poly_unscaled.is_power_of_two() {
            length_of_poly_unscaled
        } else {
            length_of_poly_unscaled.checked_next_power_of_two().unwrap()
        };
        let domain = Domain::<F>::new(length_of_poly);
        poly1_coeffs.resize(length_of_poly, F::ZERO);
        poly2_coeffs.resize(length_of_poly, F::ZERO);

        let poly_1_eval = domain.fft(&poly1_coeffs);
        let poly_2_eval = domain.fft(&poly2_coeffs);


        let mut result = vec![F::ZERO; length_of_poly];
        for i in 0..length_of_poly {
            result[i] = poly_1_eval[i] * poly_2_eval[i];
        }

        let coeff = domain.ifft(&result);
        UnivariantPolynomial::new(coeff[..length_of_poly_unscaled].to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interface::PolynomialInterface;
    use ark_test_curves::bls12_381::Fr;

    #[test]
    fn test_univariate_eval_0() {
        let poly = UnivariantPolynomial::<Fr>::random(8);
        let eval = UnivariateEval::from_coefficients(poly.coefficients.clone());
        let coeffs = eval.to_coefficients();
        assert_eq!(coeffs, poly.coefficients);
    }

    #[test]
    fn test_univariate_eval_1() {
        let poly = UnivariantPolynomial::<Fr>::new(vec![
            Fr::from(5),
            Fr::from(3),
            Fr::from(2),
            Fr::from(1),
        ]);
        let eval = UnivariateEval::from_coefficients(poly.coefficients.clone());
        let roots = eval.domain.get_roots_of_unity();

        assert!(poly.evaluate(&roots[0]) == eval.values[0]);
        assert!(poly.evaluate(&roots[1]) == eval.values[1]);
        assert!(poly.evaluate(&roots[2]) == eval.values[2]);
        assert!(poly.evaluate(&roots[3]) == eval.values[3]);

        let coeffs = eval.to_coefficients();
        let fractor_poly = UnivariantPolynomial::<Fr>::new(coeffs.clone());

        assert!(fractor_poly.evaluate(&roots[0]) == eval.values[0]);
        assert!(fractor_poly.evaluate(&roots[1]) == eval.values[1]);
        assert!(fractor_poly.evaluate(&roots[2]) == eval.values[2]);
        assert!(fractor_poly.evaluate(&roots[3]) == eval.values[3]);

        assert_eq!(coeffs, poly.coefficients);
    }

    #[test]
    fn test_univariate_eval_2() {
        let poly = UnivariantPolynomial::<Fr>::new(vec![
            Fr::from(1),
            Fr::from(2),
            Fr::from(3),
            Fr::from(4),
            Fr::from(5),
            Fr::from(6),
            Fr::from(7),
            Fr::from(8),
        ]);

        let eval = UnivariateEval::from_coefficients(poly.coefficients.clone());
        let roots = eval.domain.get_roots_of_unity();

        for i in 0..poly.coefficients.len() {
            assert!(poly.evaluate(&roots[i]) == eval.values[i]);
        }
    }

    #[test]
    fn test_univariate_eval_ran_500() {
        let poly = UnivariantPolynomial::<Fr>::random(500);

        let eval = UnivariateEval::from_coefficients(poly.coefficients.clone());
        let roots = eval.domain.get_roots_of_unity();

        for i in 0..poly.coefficients.len() {
            // poly(omega_i) = evals[i]
            assert!(poly.evaluate(&roots[i]) == eval.values[i]);
        }
    }

    #[test]
    fn test_univariate_eval_ran_500_n_vice() {
        let poly = UnivariantPolynomial::<Fr>::random(500);

        let eval = UnivariateEval::from_coefficients(poly.coefficients.clone());
        let roots = eval.domain.get_roots_of_unity();

        for i in 0..poly.coefficients.len() {
            assert!(poly.evaluate(&roots[i]) == eval.values[i]);
        }

        let coeffs = eval.to_coefficients();
        let fractor_poly = UnivariantPolynomial::<Fr>::new(coeffs.clone());

        for i in 0..poly.coefficients.len() {
            assert!(fractor_poly.evaluate(&roots[i]) == eval.values[i]);
        }
    }

    #[test]
    fn test_univariate_eval_multiply() {
        let poly1 = UnivariantPolynomial::<Fr>::new(vec![Fr::from(1), Fr::from(2)]);
        let poly2 = UnivariantPolynomial::<Fr>::new(vec![Fr::from(4), Fr::from(5)]);
        let result = UnivariateEval::multiply(&poly1, &poly2);
        let expected = poly1.clone() * poly2.clone();

        assert_eq!(result.coefficients, expected.coefficients);
    }

    #[test]
    fn test_univariate_eval_multiply_random_500() {
        let poly1 = UnivariantPolynomial::<Fr>::random(500);
        let poly2 = UnivariantPolynomial::<Fr>::random(500);
        let result = UnivariateEval::multiply(&poly1, &poly2);
        let expected = poly1.clone() * poly2.clone();

        assert_eq!(result.coefficients, expected.coefficients);
    }
}
