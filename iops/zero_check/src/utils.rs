//! Utility functions for the zero check protocol.
use ark_ff::PrimeField;
use polynomial::{interface::MultilinearPolynomialInterface, multilinear::Multilinear};

pub fn generate_eq_poly<F: PrimeField>(r_s: &[F]) -> Multilinear<F> {
    // x_iy_i + (1 - x_i)(1 - y_i)
    let eq_term = Multilinear::new(vec![F::ONE, F::ZERO, F::ZERO, F::ONE], 2);
    let mut eq_poly = eq_term.partial_evaluation(r_s[0], 1);

    for i in 1..r_s.len() {
        eq_poly = eq_poly.mul_distinct(&eq_term.partial_evaluation(r_s[i], 1));
    }

    eq_poly
}
