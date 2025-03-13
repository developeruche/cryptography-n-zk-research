use ark_ff::PrimeField;
use polynomial::{
    composed::multilinear::ComposedMultilinear, interface::MultilinearPolynomialInterface,
};

pub fn compute_multi_composed_bytes<F: PrimeField>(poly: &[ComposedMultilinear<F>]) -> Vec<u8> {
    let mut bytes = Vec::new();

    for p in poly.iter() {
        let p_bytes = p.to_bytes();
        bytes.extend_from_slice(&p_bytes);
    }

    bytes
}

pub fn perform_multi_partial_eval<F: PrimeField>(
    poly: &[ComposedMultilinear<F>],
    point: F,
    index: usize,
) -> Vec<ComposedMultilinear<F>> {
    let mut new_poly = Vec::new();

    for i in 0..poly.len() {
        new_poly.push(poly[i].partial_evaluation(point, index));
    }

    new_poly
}

pub fn perform_multi_eval<F: PrimeField>(
    poly: &[ComposedMultilinear<F>],
    point: &Vec<F>,
) -> Vec<F> {
    let mut result = Vec::new();

    for i in 0..poly.len() {
        result.push(poly[i].evaluate(point).unwrap());
    }

    result
}
