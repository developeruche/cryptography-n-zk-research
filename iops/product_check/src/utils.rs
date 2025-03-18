//! This file contains utility functions for the product check protocol.
use ark_ff::{PrimeField, batch_inversion};
use polynomial::{composed::multilinear::ComposedMultilinear, multilinear::Multilinear};

pub fn generate_fractional_polynomial<F: PrimeField>(
    poly_1: &ComposedMultilinear<F>,
    poly_2: &ComposedMultilinear<F>,
) -> Multilinear<F> {
    let mut poly_1_evals = vec![F::ONE; 1 << poly_1.polys[0].num_vars];
    for p1 in poly_1.polys.iter() {
        for (poly_1_eval, p1_i) in poly_1_evals.iter_mut().zip(p1.evaluations.iter()) {
            *poly_1_eval *= p1_i;
        }
    }
    let mut poly_2_evals = vec![F::ONE; 1 << poly_2.polys[0].num_vars];
    for p2 in poly_2.polys.iter() {
        for (poly_2_eval, p2_i) in poly_2_evals.iter_mut().zip(p2.evaluations.iter()) {
            *poly_2_eval *= p2_i;
        }
    }
    batch_inversion(&mut poly_2_evals);

    for (poly_1_eval, poly_2_eval) in poly_1_evals.iter_mut().zip(poly_2_evals.iter()) {
        *poly_1_eval *= poly_2_eval;
    }

    Multilinear::new(poly_1_evals, poly_1.polys[0].num_vars)
}
