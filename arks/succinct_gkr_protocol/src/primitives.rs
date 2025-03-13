use ark_ec::pairing::Pairing;
use ark_ff::PrimeField;
use polynomial::multilinear::Multilinear;
use sum_check::composed::ComposedSumCheckProof;




#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct SuccinctGKRMultilinearKZGOPenningProof<P: Pairing, F: PrimeField> {
    pub opening: F,
    pub opening_proof: Vec<P::G1>,
}

#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct SuccinctGKRProof<F: PrimeField, P: Pairing> {
    /// This is the list of sum check proofs gotten during this protocol
    pub sum_check_proofs: Vec<ComposedSumCheckProof<F>>,
    /// This is a vector contain result of eval of w_i(b)
    pub w_i_b: Vec<F>,
    /// This is a vector contain result of eval of w_i(c)
    pub w_i_c: Vec<F>,
    /// This is a multilinear polynomial representing the output of the Circuit
    pub w_0_mle: Multilinear<F>,
    /// This is the proof of the opening of the multilinear polynomial and the evaluation for b
    pub w_i_b_last_proof: SuccinctGKRMultilinearKZGOPenningProof<P, F>,
    /// This is the proof of the opening of the multilinear polynomial and the evaluation for c
    pub w_i_c_last_proof: SuccinctGKRMultilinearKZGOPenningProof<P, F>,
}

impl<P: Pairing, F: PrimeField> SuccinctGKRMultilinearKZGOPenningProof<P, F> {
    pub fn default() -> Self {
        Self {
            opening: F::zero(),
            opening_proof: Vec::new(),
        }
    }
}
