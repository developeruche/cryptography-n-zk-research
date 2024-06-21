use crate::{
    interfaces::TrustedSetupInterface,
    primitives::{QAPPolys, TrustedSetup, TrustedSetupExcecution},
};
use ark_ff::PrimeField;

impl<F: PrimeField> TrustedSetupInterface<F> for TrustedSetup<F> {
    fn run_trusted_setup(&self, circuit_details: &QAPPolys<F>) -> TrustedSetupExcecution<F> {
        unimplemented!()
    }

    fn get_verification_key(&self) -> crate::primitives::VerificationKey<F> {
        todo!()
    }

    fn get_proving_key(&self) -> crate::primitives::ProvingKey<F> {
        todo!()
    }

    fn run_trusted_setup_toxic_variables(
        &self,
        circuit_details: &QAPPolys<F>,
    ) -> TrustedSetupExcecution<F> {
        todo!()
    }
}
