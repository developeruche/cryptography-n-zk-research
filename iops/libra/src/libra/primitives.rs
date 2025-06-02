//! Primitive for the libra protocol.

use p3_field::{ExtensionField, Field};
use sum_check::primitives::SumCheckProof;

pub struct LibraProof<F: Field, E: ExtensionField<F>> {
    pub circuit_output: Vec<F>,
    pub sumcheck_proofs: Vec<SumCheckProof<F, E>>,
    pub wbs: Vec<E>,
    pub wcs: Vec<E>,
}

impl<F: Field, E: ExtensionField<F>> LibraProof<F, E> {
    pub fn new(
        output: Vec<F>,
        sumcheck_proofs: Vec<SumCheckProof<F, E>>,
        wbs: Vec<E>,
        wcs: Vec<E>,
    ) -> LibraProof<F, E> {
        LibraProof {
            circuit_output: output,
            sumcheck_proofs,
            wbs,
            wcs,
        }
    }
}
