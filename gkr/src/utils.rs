use ark_ff::PrimeField;
use polynomial::multilinear::Multilinear;




pub fn usize_vec_to_mle<F: PrimeField>(
    vec: &[usize],
    max_size: usize,
) -> Multilinear<F> {
    let mut mle = Multilinear::zero(max_size);
    for i in vec {
        mle.evaluations[*i] = F::one();
    }
    
    mle
}