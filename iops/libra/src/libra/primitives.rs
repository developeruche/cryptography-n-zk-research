//! Primitive for the libra protocol.
use p3_field::{ExtensionField, Field};
use sum_check::primitives::SumCheckProof;

pub struct LibraProof<F: Field, E: ExtensionField<F>> {
    pub circuit_output: Vec<F>,
    pub sumcheck_proofs: Vec<LibraSumCheckProof<F, E>>,
    pub wb_s_add_x: Vec<E>,
    pub wc_s_add_x: Vec<E>,
    pub wb_s_add_y: Vec<E>,
    pub wc_s_add_y: Vec<E>,
    pub wb_s_mul: Vec<E>,
    pub wc_s_mul: Vec<E>,
}

pub struct LibraSumCheckProof<F: Field, E: ExtensionField<F>> {
    pub add_i_x_proof: SumCheckProof<F, E>,
    pub add_i_y_proof: SumCheckProof<F, E>,
    pub mul_i_proof: SumCheckProof<F, E>,
}

pub struct LibraSumCheckChallenges<E> {
    pub add_i_x_challenges: Vec<E>,
    pub add_i_y_challenges: Vec<E>,
    pub mul_i_challenges: Vec<E>,
}

impl<F: Field, E: ExtensionField<F>> LibraProof<F, E> {
    pub fn new(
        output: Vec<F>,
        sumcheck_proofs: Vec<LibraSumCheckProof<F, E>>,
        wb_s_add_x: Vec<E>,
        wc_s_add_x: Vec<E>,
        wb_s_add_y: Vec<E>,
        wc_s_add_y: Vec<E>,
        wb_s_mul: Vec<E>,
        wc_s_mul: Vec<E>,
    ) -> LibraProof<F, E> {
        LibraProof {
            circuit_output: output,
            sumcheck_proofs,
            wb_s_add_x,
            wc_s_add_x,
            wb_s_add_y,
            wc_s_add_y,
            wb_s_mul,
            wc_s_mul,
        }
    }
}
