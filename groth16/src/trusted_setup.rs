use crate::{
    interfaces::TrustedSetupInterface,
    primitives::{
        ProvingKey, QAPPolys, ToxicWaste, ToxicWastePhase2, TrustedSetup, TrustedSetupExcecution,
        VerificationKey,
    },
    utils::{
        compute_delta_inverse_l_tau_g1, compute_t_of_tau_delta_inverse_g1,
        generate_powers_of_tau_g1, generate_powers_of_tau_g1_alpha_or_beta,
        generate_powers_of_tau_g2,
    },
};
use ark_ec::{pairing::Pairing, Group};
use ark_ff::{Field, PrimeField};
use polynomial::univariant::UnivariantPolynomial;

impl<P: Pairing> TrustedSetupInterface<P> for TrustedSetup<P> {
    fn run_trusted_setup(
        &self,
        toxic_waste: &ToxicWaste<P::ScalarField>,
        number_of_constraints: usize,
    ) -> TrustedSetupExcecution<P> {
        let powers_of_tau_g1 =
            generate_powers_of_tau_g1::<P>(toxic_waste.tau, (number_of_constraints * 2) - 1);
        let powers_of_tau_g2 =
            generate_powers_of_tau_g2::<P>(toxic_waste.tau, number_of_constraints - 1);
        let powers_of_tau_g1_alpha = generate_powers_of_tau_g1_alpha_or_beta::<P>(
            toxic_waste.tau,
            toxic_waste.alpha,
            number_of_constraints - 1,
        );
        let powers_of_tau_g1_beta = generate_powers_of_tau_g1_alpha_or_beta::<P>(
            toxic_waste.tau,
            toxic_waste.beta,
            number_of_constraints - 1,
        );
        let beta_g2 = P::G2::generator().mul_bigint(toxic_waste.beta.into_bigint());

        let alpha_g1 = P::G1::generator().mul_bigint(toxic_waste.alpha.into_bigint());
        let beta_g1 = P::G1::generator().mul_bigint(toxic_waste.beta.into_bigint());

        TrustedSetupExcecution::<P>::new(
            powers_of_tau_g1,
            powers_of_tau_g2,
            powers_of_tau_g1_alpha,
            powers_of_tau_g1_beta,
            beta_g2,
            alpha_g1,
            beta_g1,
        )
    }

    fn get_proving_key(
        &self,
        trusted_setup_exec: &TrustedSetupExcecution<P>,
        circuit_details: &QAPPolys<P::ScalarField>,
        t_poly: &UnivariantPolynomial<P::ScalarField>,
        toxic_waste: &ToxicWastePhase2<P::ScalarField>,
        number_of_constraints: usize,
    ) -> ProvingKey<P> {
        let public_variables_size: usize = 1; // this is a constant for groth16
        let delta_g1 = P::G1::generator().mul_bigint(toxic_waste.delta.into_bigint());
        let delta_inverse_l_tau_g1 = compute_delta_inverse_l_tau_g1::<P>(
            &circuit_details.a,
            &circuit_details.b,
            &circuit_details.c,
            &trusted_setup_exec.powers_of_tau_g1_alpha,
            &trusted_setup_exec.powers_of_tau_g1_beta,
            &trusted_setup_exec.powers_of_tau_g1,
            &toxic_waste.delta.inverse().unwrap(),
            public_variables_size,
            number_of_constraints - 1,
        );
        let delta_inverse_l_t_of_tau_g1 = compute_t_of_tau_delta_inverse_g1::<P>(
            &trusted_setup_exec.powers_of_tau_g1,
            &t_poly,
            &toxic_waste.delta.inverse().unwrap(),
            number_of_constraints - 1,
        );

        todo!()
    }

    fn get_verification_key(
        &self,
        trusted_setup_exec: &TrustedSetupExcecution<P>,
        circuit_details: &QAPPolys<P::ScalarField>,
    ) -> VerificationKey<P> {
        todo!()
    }
}
