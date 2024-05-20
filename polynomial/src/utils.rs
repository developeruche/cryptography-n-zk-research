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
pub fn round_pairing_index(len: usize) ->Vec<(usize, usize)> {
    let mut result = Vec::new();
    for y_1 in 0..len / 2 {
        result.push((y_1, (len / 2) + y_1));
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
        let result = round_pairing_index(len);
        assert_eq!(result, vec![(0, 2), (1, 3)]);
    }
}
