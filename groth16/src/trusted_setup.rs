use crate::{
    interfaces::TrustedSetupInterface,
    primitives::{ProvingKey, QAPPolys, TrustedSetup, TrustedSetupExcecution, VerificationKey},
    utils::{
        generate_powers_of_tau_g1, generate_powers_of_tau_g1_alpha_or_beta,
        generate_powers_of_tau_g2,
    },
};
use ark_ec::{pairing::Pairing, Group};
use ark_ff::PrimeField;

impl<P: Pairing> TrustedSetupInterface<P> for TrustedSetup<P> {
    fn run_trusted_setup(
        &self,
        circuit_details: &QAPPolys<P::ScalarField>,
    ) -> TrustedSetupExcecution<P> {
        let powers_of_tau_g1 = generate_powers_of_tau_g1::<P>(
            self.toxic_waste.tau,
            (self.number_of_constraints * 2) - 1,
        );
        let powers_of_tau_g2 =
            generate_powers_of_tau_g2::<P>(self.toxic_waste.tau, self.number_of_constraints - 1);
        let powers_of_tau_g1_alpha = generate_powers_of_tau_g1_alpha_or_beta::<P>(
            self.toxic_waste.tau,
            self.toxic_waste.alpha,
            self.number_of_constraints - 1,
        );
        let powers_of_tau_g1_beta = generate_powers_of_tau_g1_alpha_or_beta::<P>(
            self.toxic_waste.tau,
            self.toxic_waste.beta,
            self.number_of_constraints - 1,
        );
        let beta_g2 = P::G2::generator().mul_bigint(self.toxic_waste.beta.into_bigint());

        TrustedSetupExcecution::<P>::new(
            powers_of_tau_g1,
            powers_of_tau_g2,
            powers_of_tau_g1_alpha,
            powers_of_tau_g1_beta,
            beta_g2,
        )
    }

    fn get_verification_key(
        &self,
        trusted_setup_exec: &TrustedSetupExcecution<P>,
    ) -> VerificationKey<P> {
        todo!()
    }

    fn get_proving_key(&self) -> ProvingKey<P> {
        todo!()
    }
}
