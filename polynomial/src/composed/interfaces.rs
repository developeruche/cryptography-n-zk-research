use ark_ff::PrimeField;

pub trait ComposedMultilinearInterface<F: PrimeField> {
    /// Returns the element wise product of the polymonials
    fn elementwise_product(&self) -> Vec<F>;
}
