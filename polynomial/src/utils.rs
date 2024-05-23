use ark_ff::Field;

use crate::{interface::UnivariantPolynomialInterface, univariant::UnivariantPolynomial};

pub fn get_langrange_basis<F: Field>(
    domain: &Vec<F>,
    y_s: &Vec<F>,
) -> Vec<UnivariantPolynomial<F>> {
    let mut basis = Vec::new();

    if domain.len() != y_s.len() {
        panic!(
            "The length of domain and y_s should be the same: {}, {}",
            domain.len(),
            y_s.len()
        );
    }

    for i in 0..domain.len() {
        let mut basis_element = UnivariantPolynomial::new(vec![F::one()]);

        for j in 0..domain.len() {
            if i == j {
                continue;
            }

            // basis_element *= "x - domain[j]" / (domain[i] - domain[j]);
            let numerator = UnivariantPolynomial::from_coefficients_vec(vec![-domain[j], F::one()]);
            let denominator = domain[i] - domain[j];
            basis_element = basis_element
                * (numerator
                    * UnivariantPolynomial::from_coefficients_vec(vec![denominator
                        .inverse()
                        .unwrap()]));
        }

        basis.push(basis_element * UnivariantPolynomial::from_coefficients_vec(vec![y_s[i]]));
    }

    basis
}

/// This function is a helper function used to evaluate a multilinear polynomial at a given point
/// This is how the equation looks like:
/// y = x * y_2 + (1 - x) * y_1 where x is a field element
pub fn multilinear_evalutation_equation<F: Field>(x: F, y_1: F, y_2: F) -> F {
    x * y_2 + (F::one() - x) * y_1
}

/// returns a vector of (y_1, y_2)
pub fn round_pairing_index(len: usize, delta: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    for y_1 in 0..len / 2 {
        result.push((y_1 + delta, (len / 2) + y_1 + delta));
    }

    result
}

/// returns a vector of (y_1, y_2), this is used in a exrension manner
pub fn round_pairing_index_ext(len: usize, log_iterations: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    let iterations = 1 << log_iterations;

    println!("Current iter: {:?}", iterations);

    for i in 0..iterations {
        println!("Each iter input: {:?} - {:?}", len / iterations, result.len() * 2);
        let round = round_pairing_index(len / iterations, result.len() * 2);
        result.extend(round);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_test_curves::bls12_381::Fr;

    #[test]
    fn test_multilinear_evalutation_equation() {
        let x = Fr::from(5);
        let y_1 = Fr::from(3);
        let y_2 = Fr::from(2);

        assert_eq!(multilinear_evalutation_equation(x, y_1, y_2), Fr::from(-2));
    }

    #[test]
    fn test_round_pairing_index() {
        let len = 4;
        let result = round_pairing_index(len, 0);
        assert_eq!(result, vec![(0, 2), (1, 3)]);
    }

    #[test]
    fn test_round_pairing_index_ext_0() {
        let len = 8;
        let result = round_pairing_index_ext(len, 0);
        assert_eq!(result, vec![(0, 4), (1, 5), (2, 6), (3, 7)]);
    }

    #[test]
    fn test_round_pairing_index_ext_1() {
        let len = 8;
        let result = round_pairing_index_ext(len, 1);
        assert_eq!(result, vec![(0, 2), (1, 3), (4, 6), (5, 7)]);
    }

    #[test]
    fn test_round_pairing_index_ext_2() {
        let len = 8;
        let result = round_pairing_index_ext(len, 2);
        assert_eq!(result, vec![(0, 1), (2, 3), (4, 5), (6, 7)]);
    }

    #[test]
    fn test_round_pairing_index_ext_16_0() {
        let len = 16;
        let result = round_pairing_index_ext(len, 0);
        assert_eq!(result, vec![(0, 8), (1, 9), (2, 10), (3, 11), (4, 12), (5, 13), (6, 14), (7, 15)]);
    }

    #[test]
    fn test_round_pairing_index_ext_16_1() {
        let len = 16;
        let result = round_pairing_index_ext(len, 1);
        assert_eq!(result, vec![(0, 4), (1, 5), (2, 6), (3, 7), (8, 12), (9, 13), (10, 14), (11, 15)]);
    }

    #[test]
    fn test_round_pairing_index_ext_16_2() {
        let len = 16;
        let result = round_pairing_index_ext(len, 2);
        assert_eq!(result, vec![(0, 2), (1, 3), (4, 6), (5, 7), (8, 10), (9, 11), (12, 14), (13, 15)]);
    }

    #[test]
    fn test_round_pairing_index_ext_16_3() {
        let len = 16;
        let result = round_pairing_index_ext(len, 3);
        assert_eq!(result, vec![(0, 1), (2, 3), (4, 5), (6, 7), (8, 9), (10, 11), (12, 13), (14, 15)]);
    }
}
