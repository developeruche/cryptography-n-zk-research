use ark_ff::Field;

use crate::{interface::UnivariantPolynomialInterface, univariant::UnivariantPolynomial};

pub fn get_langrange_basis<F: Field>(domain: &Vec<F>) -> Vec<UnivariantPolynomial<F>> {
    let mut basis = Vec::new();

    for i in 0..domain.len() {
        let mut basis_element = UnivariantPolynomial::default();

        for j in 0..domain.len() {
            if i == j {
                continue;
            }

            // basis_element *= "x - domain[j]" / (domain[i] - domain[j]);
            let numerator = UnivariantPolynomial::from_coefficients_vec(vec![-domain[j], F::one()]);
            let denominator = domain[i] - domain[j];
            // basis_element = numerator / denominator;
        }

        basis.push(basis_element);
    }

    basis
}
