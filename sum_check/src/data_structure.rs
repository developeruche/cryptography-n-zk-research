use ark_ff::PrimeField;
use polynomial::interface::MultilinearPolynomialInterface;

/// This struct is used to store the sum check proof
#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct SumCheckProof<F: PrimeField, P: MultilinearPolynomialInterface<F>> {
    /// This is the polynomial that is used to generate the sum check proof
    pub polynomial: P,
    /// This vector stores the round polynomials
    pub round_poly: Vec<P>,
    /// This vectors store the polynomial from the first round
    pub round_0_poly: P,
    /// This holds the sum of the polynomial evaluation over the boolean hypercube
    pub sum: F,
}
