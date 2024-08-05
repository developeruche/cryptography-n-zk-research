use ark_ff::PrimeField;
use polynomial::{
    interface::MultilinearPolynomialInterface, multilinear::Multilinear,
    univariant::UnivariantPolynomial,
};
use sum_check::data_structure::SumCheckProof;

#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct GKRProof<F: PrimeField, P: MultilinearPolynomialInterface<F>> {
    /// This is the output of the Circuit evaluation
    pub output: Vec<F>,
    /// This is the list of sum check proofs gotten during this protocol
    pub sum_check_proofs: Vec<SumCheckProof<F, P>>,
    /// This is the list of q polynomials
    pub q_polynomials: Vec<UnivariantPolynomial<F>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_test_curves::bls12_381::Fr;
}
