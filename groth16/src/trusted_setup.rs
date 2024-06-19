use ark_ff::PrimeField;
use crate::{interfaces::TrustedSetupInterface, primitives::{QAPPolys, TrustedSetup, TrustedSetupExcecution}};




impl<F: PrimeField> TrustedSetupInterface<F> for TrustedSetup<F> {
    fn run_trusted_setup(&self, circuit_details: &QAPPolys<F>) -> TrustedSetupExcecution<F> {
        unimplemented!()
    }
}