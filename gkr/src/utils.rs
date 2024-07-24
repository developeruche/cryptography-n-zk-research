use ark_ff::PrimeField;
use polynomial::{
    interface::MultilinearPolynomialInterface, multilinear::Multilinear, utils::compute_domain,
};

pub fn gen_w_mle<F: PrimeField>(evals: &[Vec<F>], layer_index: usize) -> Multilinear<F> {
    // see if the layer index is out of bounds
    if layer_index >= evals.len() {
        panic!("Layer index out of bounds");
    }
    let domain = compute_domain(evals[layer_index].len(), 0);

    Multilinear::interpolate(evals[layer_index].clone(), domain)
}
