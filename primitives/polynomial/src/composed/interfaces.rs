use crate::multilinear::Multilinear;
use ark_ff::PrimeField;

pub trait ComposedMultilinearInterface<F: PrimeField> {
    /// Returns the element wise product of the polymonials
    fn elementwise_product(&self) -> Vec<F>;
    /// Returns the possible maximum degree of the composed polynomial
    fn max_degree(&self) -> usize;
    /// Returns the product of the composed polynomial with another multilinear polynomial
    fn mul_by_mle(&self, mle: &Multilinear<F>) -> Self;
}
