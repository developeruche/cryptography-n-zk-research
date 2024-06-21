use crate::{
    interfaces::TrustedSetupInterface,
    primitives::{QAPPolys, TrustedSetup, TrustedSetupExcecution, VerificationKey},
};
use ark_ec::pairing::Pairing;

impl<P: Pairing> TrustedSetupInterface<P> for TrustedSetup<P> {
    fn run_trusted_setup(&self, circuit_details: &QAPPolys<P::ScalarField>) -> TrustedSetupExcecution<P::ScalarField> {
        unimplemented!()
    }

    fn get_verification_key(&self) -> VerificationKey<P::ScalarField> {
        todo!()
    }

    fn get_proving_key(&self) -> crate::primitives::ProvingKey<P::ScalarField> {
        todo!()
    }

    fn run_trusted_setup_toxic_variables(
        &self,
        circuit_details: &QAPPolys<P::ScalarField>,
    ) -> TrustedSetupExcecution<P::ScalarField> {
        todo!()
    }
}
