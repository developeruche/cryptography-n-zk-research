#![allow(non_snake_case)]

use crate::interface::PlonkVerifierInterface;
use ark_ec::{pairing::Pairing, Group};
use ark_ff::PrimeField;
use pcs::primitives::SRS;
use plonk_compiler::utils::root_of_unity;
use plonk_core::primitives::{
    PlonkProof, PlonkishIntermediateRepresentation, VerifierPreprocessedInput,
};
use polynomial::{
    evaluation::{univariate::UnivariateEval, Domain},
    interface::PolynomialInterface,
};

pub struct PlonkVerifier<P: Pairing, F: PrimeField> {
    // order of group
    pub group_order: u64,
    // This is the verifier preprocessed input
    pub verifier_preprocessed_input: VerifierPreprocessedInput<P>,
    /// This is the shared reference string
    pub srs: SRS<P>,
    _p: std::marker::PhantomData<F>,
}

impl<P: Pairing, F: PrimeField> PlonkVerifier<P, F> {
    pub fn new(
        group_order: u64,
        plonkish_intermediate_representation: PlonkishIntermediateRepresentation<F>,
        srs: SRS<P>,
    ) -> Self {
        let verifier_preprocessed_input = plonkish_intermediate_representation.to_vpi(&srs);
        Self {
            group_order,
            verifier_preprocessed_input,
            srs,
            _p: std::marker::PhantomData,
        }
    }
}

impl<F: PrimeField, P: Pairing> PlonkVerifierInterface<F, P> for PlonkVerifier<P, F> {
    fn verify(&self, proof: &PlonkProof<P, F>, public_input: UnivariateEval<F>) -> bool {
        let (beta, gamma, alpha, zeta, vinculum, mu) = proof.output_transcript_challenges();
        let Z_h_zeta = zeta.pow(&[self.group_order]) - F::ONE;

        let mut l1_values = vec![F::ZERO; self.group_order as usize];
        l1_values[0] = F::ONE;
        let domain = Domain::new(self.group_order as usize);
        let l1_poly_eval = UnivariateEval::new(l1_values, domain);
        let l1_poly = l1_poly_eval.to_coefficient_poly();
        let l1_eval_zeta = l1_poly.evaluate(&zeta);

        let root: F = root_of_unity::<F>(self.group_order);

        // let l1_eval_zeta = (root * Z_h_zeta) / (F::from(self.group_order) * (zeta - root)); // Remove after testing
        let public_params_poly_at_zeta = public_input.to_coefficient_poly().evaluate(&zeta);

        let a_x_zeta = proof.a_x_zeta;
        let b_x_zeta = proof.b_x_zeta;
        let c_x_zeta = proof.c_x_zeta;
        let w_accumulator_poly_zeta = proof.w_accumulator_poly_zeta;
        let s1_poly_zeta = proof.s1_poly_zeta;
        let s2_poly_zeta = proof.s2_poly_zeta;

        let r_0 = public_params_poly_at_zeta
            - l1_eval_zeta * alpha.pow(&[2u64])
            - alpha
                * ((a_x_zeta + s1_poly_zeta * beta + gamma)
                    * (b_x_zeta + s2_poly_zeta * beta + gamma)
                    * (c_x_zeta + gamma)
                    * w_accumulator_poly_zeta);

        let qm_1 = self.verifier_preprocessed_input.qm_1_commitment;
        let ql_1 = self.verifier_preprocessed_input.ql_1_commitment;
        let qr_1 = self.verifier_preprocessed_input.qr_1_commitment;
        let qo_1 = self.verifier_preprocessed_input.qo_1_commitment;
        let qc_1 = self.verifier_preprocessed_input.qc_1_commitment;
        let z_1 = proof.accumulator_poly_commitment;
        let s3_1 = self.verifier_preprocessed_input.s3_1_commitment;
        let t_lo_1 = proof.t_lo_poly_commitment;
        let t_mid_1 = proof.t_mid_poly_commitment;
        let t_hi_1 = proof.t_hi_poly_commitment;

        let d_1_1 = qm_1.mul_bigint(&(a_x_zeta * b_x_zeta).into_bigint())
            + ql_1.mul_bigint(&a_x_zeta.into_bigint())
            + qr_1.mul_bigint(&b_x_zeta.into_bigint())
            + qo_1.mul_bigint(&c_x_zeta.into_bigint())
            + qc_1;

        let d_1_2 = z_1.mul_bigint(
            ((a_x_zeta + zeta * beta + gamma)
                * (b_x_zeta + (F::from(2u8) * zeta * beta) + gamma)
                * (c_x_zeta + (F::from(3u8) * zeta * beta) + gamma)
                * alpha
                + l1_eval_zeta * alpha.pow(&[2u64])
                + mu)
                .into_bigint(),
        );

        let d_1_3 = s3_1.mul_bigint(
            ((a_x_zeta + s1_poly_zeta * beta + gamma)
                * (b_x_zeta + s2_poly_zeta * beta + gamma)
                * alpha
                * beta
                * w_accumulator_poly_zeta)
                .into_bigint(),
        );

        let d_1_4 = (t_lo_1
            + (t_mid_1.mul_bigint(zeta.pow(&[self.group_order]).into_bigint()))
            + (t_hi_1.mul_bigint(zeta.pow(&[2 * self.group_order]).into_bigint())))
        .mul_bigint(Z_h_zeta.into_bigint());

        let d_1 = d_1_1 + d_1_2 - d_1_3 - d_1_4;

        let a_1 = proof.a_poly_commitment;
        let b_1 = proof.b_poly_commitment;
        let c_1 = proof.c_poly_commitment;
        let s1_1 = self.verifier_preprocessed_input.s1_1_commitment;
        let s2_1 = self.verifier_preprocessed_input.s2_1_commitment;

        let f_1 = d_1
            + a_1.mul_bigint(vinculum.into_bigint())
            + b_1.mul_bigint(vinculum.pow(&[2u64]).into_bigint())
            + c_1.mul_bigint(vinculum.pow(&[3u64]).into_bigint())
            + s1_1.mul_bigint(vinculum.pow(&[4u64]).into_bigint())
            + s2_1.mul_bigint(vinculum.pow(&[5u64]).into_bigint());

        let e_1 = P::G1::generator().mul_bigint(
            (vinculum * a_x_zeta
                + vinculum.pow(&[2, 0, 0, 0]) * b_x_zeta
                + vinculum.pow(&[3, 0, 0, 0]) * c_x_zeta
                + vinculum.pow(&[4, 0, 0, 0]) * s1_poly_zeta
                + vinculum.pow(&[5, 0, 0, 0]) * s2_poly_zeta
                + mu * w_accumulator_poly_zeta
                - r_0)
                .into_bigint(),
        );
        let w_zeta_1 = proof.W_zeta_poly_commitment;
        let w_zeta_omega_1 = proof.W_zeta_w_poly_commitment;
        let x_2 = self.verifier_preprocessed_input.x_2;

        let left = P::pairing(w_zeta_1 + w_zeta_omega_1.mul_bigint(mu.into_bigint()), x_2);
        let right = P::pairing(
            w_zeta_1.mul_bigint(zeta.into_bigint())
                + w_zeta_omega_1.mul_bigint((mu * zeta * root).into_bigint())
                + f_1
                - e_1,
            P::G2::generator(),
        );

        left == right
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{interface::PlonkProverInterface, prover::PlonkProver};
    use ark_test_curves::bls12_381::{Bls12_381, Fr};
    use fiat_shamir::FiatShamirTranscript;
    use pcs::{interface::KZGUnivariateInterface, kzg::univariate::UnivariateKZG};
    use plonk_compiler::{assembly::eq_to_assembly, program::Program};
    use std::collections::HashMap;

    #[test]
    fn test_plonk_complete_prove_n_verify() {
        let original_constriants = ["e public"];
        let mut assembly_eqns = Vec::new();
        for eq in original_constriants.iter() {
            let assembly_eqn = eq_to_assembly::<Fr>(eq.to_string());
            assembly_eqns.push(assembly_eqn);
        }
        let program = Program::new(assembly_eqns, 8);

        let mut variable_assignment = HashMap::new();
        variable_assignment.insert(Some("e".to_string()), Fr::from(3));

        let witness = program.compute_witness_and_public_parameter(variable_assignment);
        let circuit_ir = program.common_preproccessed_input();

        let transcript = FiatShamirTranscript::new("plonk-protocol".as_bytes().to_vec());
        let srs: SRS<Bls12_381> =
            UnivariateKZG::generate_srs(&Fr::from(6), program.group_order as usize * 4);
        let mut prover = PlonkProver::new(transcript, circuit_ir.clone(), srs.clone());
        let proof = prover.prove(&witness);
        let verifer = PlonkVerifier::new(program.group_order, circuit_ir.clone(), srs);
        let is_valid = verifer.verify(&proof, witness.pi);
        assert_eq!(is_valid, true);
    }

    #[test]
    fn test_plonk_complete_prove_n_verify_1() {
        let original_constriants = [
            "x public",
            "c <== a * b",
            "f <== d * e",
            "g <== c + f",
            "x <== g * y",
        ];
        let mut assembly_eqns = Vec::new();
        for eq in original_constriants.iter() {
            let assembly_eqn = eq_to_assembly::<Fr>(eq.to_string());
            assembly_eqns.push(assembly_eqn);
        }
        let program = Program::new(assembly_eqns, 8);

        let mut variable_assignment = HashMap::new();
        variable_assignment.insert(Some("x".to_string()), Fr::from(258));
        variable_assignment.insert(Some("a".to_string()), Fr::from(2));
        variable_assignment.insert(Some("b".to_string()), Fr::from(4));
        variable_assignment.insert(Some("d".to_string()), Fr::from(5));
        variable_assignment.insert(Some("e".to_string()), Fr::from(7));
        variable_assignment.insert(Some("y".to_string()), Fr::from(6));

        let witness = program.compute_witness_and_public_parameter(variable_assignment);
        let circuit_ir = program.common_preproccessed_input();

        let transcript = FiatShamirTranscript::new("plonk-protocol".as_bytes().to_vec());
        let srs: SRS<Bls12_381> =
            UnivariateKZG::generate_srs(&Fr::from(6), program.group_order as usize * 4);
        let mut prover = PlonkProver::new(transcript, circuit_ir.clone(), srs.clone());
        let proof = prover.prove(&witness);
        let verifer = PlonkVerifier::new(program.group_order, circuit_ir.clone(), srs);
        let is_valid = verifer.verify(&proof, witness.pi);
        assert_eq!(is_valid, true);
    }
}
