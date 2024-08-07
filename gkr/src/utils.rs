use ark_ff::PrimeField;
use polynomial::{
    interface::{MultilinearPolynomialInterface, PolynomialInterface},
    multilinear::Multilinear,
    univariant::UnivariantPolynomial,
};

pub fn gen_w_mle<F: PrimeField>(evals: &[Vec<F>], layer_index: usize) -> Multilinear<F> {
    // see if the layer index is out of bounds
    if layer_index >= evals.len() {
        panic!("Layer index out of bounds");
    }

    Multilinear::interpolate(&evals[layer_index])
}

pub fn gen_l<F: PrimeField>(
    b: &[F],
    c: &[F],
) -> Result<Vec<UnivariantPolynomial<F>>, &'static str> {
    // perfroming some santiy checks
    if b.len() != c.len() {
        return Err("Length of b and c must be the same");
    }

    Ok(b.iter()
        .zip(c.iter())
        .map(|(b, c)| {
            let mut coeffs = vec![*b, *c - b];
            UnivariantPolynomial::new(coeffs)
        })
        .collect())
}

pub fn evaluate_l<F: PrimeField>(l: &[UnivariantPolynomial<F>], x: F) -> Vec<F> {
    l.iter().map(|l_i| l_i.evaluate(&x)).collect()
}

pub fn gen_q<F: PrimeField>(
    l: &[UnivariantPolynomial<F>],
    w: Multilinear<F>,
) -> Result<UnivariantPolynomial<F>, &'static str> {
    // performing some sanity checks
    if l.len() != w.num_vars() {
        return Err("Length of l and w must be the same");
    }

    todo!()
}
