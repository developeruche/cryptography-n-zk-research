use ark_ff::Field;
use polynomial::multilinear::Multilinear;



/// This struct is used to store the sum check proof
#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct SumCheckProof<F: Field> {
    /// This vector stores the round polynomials
    pub round_poly: Vec<Multilinear<F>>,
    /// This vectors store the polynomial from the first round
    pub round_0_poly: Multilinear<F>,
    /// This holds the sum of the polynomial evaluation over the boolean hypercube
    pub sum: F,
}