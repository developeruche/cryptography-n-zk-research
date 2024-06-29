use crate::{
    interfaces::TrustedSetupInterface,
    primitives::{
        QAPPolys, ToxicWaste, TrustedSetup,
        TrustedSetupExcecution,
    },
    utils::{
        generate_c_tau_plus_beta_a_tau_plus_alpha_b_tau_g1_public, generate_powers_of_tau_g1, generate_powers_of_tau_g2, generate_powers_of_tau_t_poly_delta_inverse_g1, generate_t_poly
    },
};
use ark_ec::{pairing::Pairing, Group};
use ark_ff::{Field, PrimeField};
use polynomial::interface::PolynomialInterface;








impl<P: Pairing> TrustedSetupInterface<P> for TrustedSetup<P> {
    fn run_trusted_setup(
        &self,
        toxic_waste: &ToxicWaste<P::ScalarField>,
        qap_polys: &QAPPolys<P::ScalarField>,
        number_of_constraints: usize,
    ) -> TrustedSetupExcecution<P> {
        let t_poly = generate_t_poly::<P::ScalarField>(number_of_constraints);
        let powers_of_tau_g1 =
            generate_powers_of_tau_g1::<P>(toxic_waste.tau, (number_of_constraints * 2) - 1);
        let powers_of_tau_g2 =
            generate_powers_of_tau_g2::<P>(toxic_waste.tau, number_of_constraints - 1);
        let powers_of_tau_t_poly_delta_inverse_g1 =
            generate_powers_of_tau_t_poly_delta_inverse_g1::<P>(
                toxic_waste.tau,
                toxic_waste.delta.inverse().unwrap(),
                &t_poly,
                t_poly.degree(),
            );
        let beta_g2 = P::G2::generator().mul_bigint(toxic_waste.beta.into_bigint());
        let alpha_g1 = P::G1::generator().mul_bigint(toxic_waste.alpha.into_bigint());
        let gamma_g2 = P::G2::generator().mul_bigint(toxic_waste.gamma.into_bigint());
        let delta_g2 = P::G2::generator().mul_bigint(toxic_waste.delta.into_bigint());
        let beta_g1 = P::G1::generator().mul_bigint(toxic_waste.beta.into_bigint());
        let delta_g1 = P::G1::generator().mul_bigint(toxic_waste.delta.into_bigint());

        // this is done here because the phases for the trusted set is reduced to one
        // this is Ideally done when needed by the prover using linear combination
        // c(tau) + beta*a(tau) + alpha*c(tau))
        let c_tau: Vec<P::ScalarField> = qap_polys
            .c
            .iter()
            .map(|c| c.evaluate(&toxic_waste.tau))
            .collect();
        let a_tau: Vec<P::ScalarField> = qap_polys
            .a
            .iter()
            .map(|a| a.evaluate(&toxic_waste.tau) * toxic_waste.beta)
            .collect();
        let b_tau: Vec<P::ScalarField> = qap_polys
            .b
            .iter()
            .map(|b| b.evaluate(&toxic_waste.tau) * toxic_waste.alpha)
            .collect();
        let c_tau_plus_beta_a_tau_plus_alpha_b_tau: Vec<P::ScalarField> = c_tau
            .iter()
            .zip(a_tau.iter())
            .zip(b_tau.iter())
            .map(|((c, a), b)| *c + *a + b)
            .collect();
        
        let c_tau_plus_beta_a_tau_plus_alpha_b_tau_g1_public = generate_c_tau_plus_beta_a_tau_plus_alpha_b_tau_g1_public::<P>(&c_tau_plus_beta_a_tau_plus_alpha_b_tau, &toxic_waste.gamma);
        let c_tau_plus_beta_a_tau_plus_alpha_b_tau_g1_private = generate_c_tau_plus_beta_a_tau_plus_alpha_b_tau_g1_public::<P>(&c_tau_plus_beta_a_tau_plus_alpha_b_tau, &toxic_waste.gamma);
        
    

        TrustedSetupExcecution::<P>::new(
            powers_of_tau_g1,
            powers_of_tau_g2,
            beta_g2,
            alpha_g1,
            beta_g1,
            c_tau_plus_beta_a_tau_plus_alpha_b_tau_g1_public,
            c_tau_plus_beta_a_tau_plus_alpha_b_tau_g1_private,
            powers_of_tau_t_poly_delta_inverse_g1,
            gamma_g2,
            delta_g2,
            delta_g1,
        )
    }
}
