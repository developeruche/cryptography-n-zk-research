use ark_ff::PrimeField;
use polynomial::{
    interface::MultilinearPolynomialInterface, multilinear::Multilinear,
    univariant::UnivariantPolynomial,
};
use sum_check::data_structure::SumCheckProof;

#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct W<F: PrimeField> {
    /// This is the addition multilinear extension
    add_i: Multilinear<F>,
    /// This is the multiplication multilinear extension
    mul_i: Multilinear<F>,
    /// This is the w_b equation
    w_b: Multilinear<F>,
    /// This is the w_c equation
    w_c: Multilinear<F>,
}

#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct GKRProof<F: PrimeField, P: MultilinearPolynomialInterface<F>> {
    /// This is the output of the Circuit evaluation
    pub output: Vec<F>,
    /// This is the list of sum check proofs gotten during this protocol
    pub sum_check_proofs: Vec<SumCheckProof<F, P>>,
    /// This is the list of q polynomials
    pub q_polynomials: Vec<UnivariantPolynomial<F>>,
}

impl<F: PrimeField> W<F> {
    pub fn new(
        add_i: Multilinear<F>,
        mul_i: Multilinear<F>,
        w_b: Multilinear<F>,
        w_c: Multilinear<F>,
    ) -> Self {
        W {
            add_i,
            mul_i,
            w_b,
            w_c,
        }
    }
}
