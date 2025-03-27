//! This file contains utility functions for the product check protocol.

use ark_ff::{PrimeField, batch_inversion};
use fiat_shamir::FiatShamirTranscript;
use polynomial::{
    composed::{interfaces::ComposedMultilinearInterface, multilinear::ComposedMultilinear},
    interface::MultilinearPolynomialInterface,
    multilinear::Multilinear,
};
use sum_check::composed::ComposedSumCheckProof;
use zero_check::{ZeroCheck, interface::ZeroCheckInterface};

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

pub fn generate_product_poly<F: PrimeField>(fractional_poly: &Multilinear<F>) -> Multilinear<F> {
    let num_vars = fractional_poly.num_vars;
    let fractional_poly_evals = fractional_poly.evaluations.clone();
    let mut product_poly_evals = vec![];

    for i in 0..(1 << num_vars) - 1 {
        let (x_zero_index, x_one_index, sign) = get_index(i, num_vars);
        if !sign {
            product_poly_evals
                .push(fractional_poly_evals[x_zero_index] * fractional_poly_evals[x_one_index]);
        } else {
            product_poly_evals
                .push(product_poly_evals[x_zero_index] * product_poly_evals[x_one_index]);
        }
    }

    product_poly_evals.push(F::zero());

    Multilinear::new(product_poly_evals, num_vars)
}

pub fn perform_zero_check_protocol<F: PrimeField>(
    poly_1: &ComposedMultilinear<F>,
    poly_2: &ComposedMultilinear<F>,
    fractional_poly: &Multilinear<F>,
    product_poly: &Multilinear<F>,
    alpha: &F,
    transcript: &mut FiatShamirTranscript,
) -> Result<(ComposedSumCheckProof<F>, ComposedMultilinear<F>), anyhow::Error> {
    let num_vars = fractional_poly.num_vars;

    let mut p1_evals = vec![F::ZERO; 1 << num_vars];
    let mut p2_evals = vec![F::ZERO; 1 << num_vars];

    for x in 0..1 << num_vars {
        let (x0, x1, sign) = get_index(x, num_vars);
        if !sign {
            p1_evals[x] = fractional_poly.evaluations[x0];
            p2_evals[x] = fractional_poly.evaluations[x1];
        } else {
            p1_evals[x] = product_poly.evaluations[x0];
            p2_evals[x] = product_poly.evaluations[x1];
        }
    }

    let p1 = Multilinear::new(p1_evals, num_vars);
    let p2 = Multilinear::new(p2_evals, num_vars);

    let mut q_x = ComposedMultilinear::new(vec![product_poly.clone(), p1, p2 * -F::from(1u32)]); // Dubug Source: p(x) - (p1(x) * p2(x)), Composed Polynommial has no way to represent this structure, the solution to this would be to come up with one something similar to the Virual Polynomail by Espressolabs

    println!(
        "q_x: {:?}",
        q_x.evaluate(&vec![F::from(10u32), F::from(20u32), F::from(30u32)])
    );

    let mut mle_list = poly_2.polys.clone();
    mle_list.push(fractional_poly.clone() * *alpha);
    q_x.extend_mle(&mle_list);

    let mut poly_1_alpha = poly_1.clone();
    poly_1_alpha.polys[0] = poly_1_alpha.polys[0].clone() * -*alpha;
    q_x.extend_mle(&poly_1_alpha.polys);

    let zero_check = ZeroCheck::prove(&q_x, transcript)?;

    Ok((zero_check, q_x))
}

// Acknowledgement: the rest  of the code below was obtained from espressolabs hyperplonk implementation
pub fn get_index(i: usize, num_vars: usize) -> (usize, usize, bool) {
    let bit_sequence = bit_decompose(i as u64, num_vars);

    // the last bit comes first here because of LE encoding
    let x0 = project(&[[false].as_ref(), bit_sequence[..num_vars - 1].as_ref()].concat()) as usize;
    let x1 = project(&[[true].as_ref(), bit_sequence[..num_vars - 1].as_ref()].concat()) as usize;

    (x0, x1, bit_sequence[num_vars - 1])
}

pub(crate) fn project(input: &[bool]) -> u64 {
    let mut res = 0;
    for &e in input.iter().rev() {
        res <<= 1;
        res += e as u64;
    }
    res
}

pub fn bit_decompose(input: u64, num_var: usize) -> Vec<bool> {
    let mut res = Vec::with_capacity(num_var);
    let mut i = input;
    for _ in 0..num_var {
        res.push(i & 1 == 1);
        i >>= 1;
    }
    res
}
