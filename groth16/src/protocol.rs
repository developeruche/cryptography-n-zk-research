use crate::{
    interfaces::ProtocolInterface,
    primitives::{Proof, ProofRands, TrustedSetupExcecution, Witness, QAP},
    utils::{
        internal_product_g1, linear_combination_homomorphic_poly_eval_g1,
        linear_combination_homomorphic_poly_eval_g2, PRIVATE_VARIABLES_INDEX,
    },
};
use ark_ec::{pairing::Pairing, Group};
use ark_ff::PrimeField;

/// This is the Groth16 protocol implementation (Struct binding to the ProtocolInterface)
pub struct Groth16Protocol<P: Pairing> {
    // just binding this struct to this type...
    phantom: std::marker::PhantomData<P>,
}

impl<P: Pairing> ProtocolInterface<P> for Groth16Protocol<P> {
    /// This function is used to generate a groth16 proof
    /// parameters:
    /// proof_rands: The random values used to generate the proof
    ///
    /// this proofing would be done in this manner;
    /// A is calculated, then B, then C
    /// proof = ([A]_1, [B]_2, [C]_1)
    fn generate_proof(
        proof_rands: ProofRands<P::ScalarField>,
        trusted_setup: &TrustedSetupExcecution<P>,
        qap: &QAP<P::ScalarField>,
        witness: &Witness<P::ScalarField>,
    ) -> Proof<P> {
        // generating A (g)
        let r_delta_g1 = trusted_setup
            .delta_g1
            .mul_bigint(proof_rands.r.into_bigint());
        let qap_a_at_tau: P::G1 = linear_combination_homomorphic_poly_eval_g1::<P>(
            &qap.ax,
            &trusted_setup.powers_of_tau_g1,
        );
        let a_g1 = r_delta_g1 + trusted_setup.alpha_g1 + qap_a_at_tau;

        // generating B (g2)
        let s_delta_g2 = trusted_setup
            .delta_g2
            .mul_bigint(proof_rands.s.into_bigint());
        let qap_b_at_tau = linear_combination_homomorphic_poly_eval_g2::<P>(
            &qap.bx,
            &trusted_setup.powers_of_tau_g2,
        );
        let b_g2 = s_delta_g2 + trusted_setup.beta_g2 + qap_b_at_tau;

        // generate B (g1)
        let s_delta_g1 = trusted_setup
            .delta_g1
            .mul_bigint(proof_rands.s.into_bigint());
        let qap_b_at_tau = linear_combination_homomorphic_poly_eval_g1::<P>(
            &qap.bx,
            &trusted_setup.powers_of_tau_g1,
        );
        let b_g1 = s_delta_g1 + trusted_setup.beta_g1 + qap_b_at_tau;

        let witness_vec = witness.render();
        let x_g1 = internal_product_g1::<P>(
            &trusted_setup.c_tau_plus_beta_a_tau_plus_alpha_b_tau_g1_public,
            &witness_vec[..PRIVATE_VARIABLES_INDEX].to_vec(),
        );

        let c_prime_g1 = internal_product_g1::<P>(
            &trusted_setup.c_tau_plus_beta_a_tau_plus_alpha_b_tau_g1_private,
            &witness_vec[PRIVATE_VARIABLES_INDEX..].to_vec(),
        );
        let ht_g1 = linear_combination_homomorphic_poly_eval_g1::<P>(
            &qap.h,
            &trusted_setup.powers_of_tau_t_poly_delta_inverse_g1,
        );

        let s_a_g1 = a_g1.mul_bigint(proof_rands.s.into_bigint());
        let r_b_g1 = b_g1.mul_bigint(proof_rands.r.into_bigint());
        let r_mul_s_delta_g1 = trusted_setup
            .delta_g1
            .mul_bigint((proof_rands.r * proof_rands.s).into_bigint());
        let c_g1 = c_prime_g1 + ht_g1 + s_a_g1 + r_b_g1 + (-r_mul_s_delta_g1);

        Proof {
            a: a_g1,
            b: b_g2,
            c: c_g1,
        }
    }

    /// This function is used to verify a groth16 proof
    fn verify_proof(
        proof: &Proof<P>,
        trusted_setup: &TrustedSetupExcecution<P>,
        public_input: &Vec<P::ScalarField>,
    ) -> bool {
        let lhs = P::pairing(proof.a, proof.b);
        let rhs_1 = P::pairing(trusted_setup.alpha_g1, trusted_setup.beta_g2);
        let rhs_2 = P::pairing(
            internal_product_g1::<P>(
                &trusted_setup.c_tau_plus_beta_a_tau_plus_alpha_b_tau_g1_public,
                public_input,
            ),
            trusted_setup.gamma_g2,
        );
        let rhs_3 = P::pairing(proof.c, trusted_setup.delta_g2);

        lhs == (rhs_1 + rhs_2 + rhs_3)
    }
}
