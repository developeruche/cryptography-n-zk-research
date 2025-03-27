use ark_ff::PrimeField;
use polynomial::multilinear::Multilinear;
use sum_check::composed::ComposedSumCheckProof;

#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct GKRProof<F: PrimeField> {
    /// This is the list of sum check proofs gotten during this protocol, a sumcheck proof for all the layers of the circuit
    pub sum_check_proofs: Vec<ComposedSumCheckProof<F>>,
    /// This is a vector contain result of eval of w_i(b)
    pub w_i_b: Vec<F>,
    /// This is a vector contain result of eval of w_i(c)
    pub w_i_c: Vec<F>,
    /// This is a multilinear polynomial representing the output of the Circuit
    pub w_0_mle: Multilinear<F>,
}
