#![allow(non_snake_case)]

use crate::{
    interface::PlonkProverInterface,
    utils::{apply_w_to_polynomial, split_poly_in_3},
};
use ark_ec::pairing::Pairing;
use ark_ff::PrimeField;
use fiat_shamir::{interface::TranscriptInterface, FiatShamirTranscript};
use kzg_rust::{interface::KZGUnivariateInterface, primitives::SRS, univariate::UnivariateKZG};
use plonk_compiler::utils::{root_of_unity, roots_of_unity};
use plonk_core::primitives::{
    PlonkProof, PlonkishIntermediateRepresentation, RoundFiveOutput, RoundFourOutput,
    RoundOneOutput, RoundThreeOutput, RoundTwoOutput, Witness,
};
use polynomial::{
    evaluation::{univariate::UnivariateEval, Domain},
    interface::{PolynomialInterface, UnivariantPolynomialInterface},
    univariant::UnivariantPolynomial,
};
use std::marker::PhantomData;

pub struct PlonkProver<F: PrimeField, P: Pairing> {
    transcript: FiatShamirTranscript,
    circuit_ir: PlonkishIntermediateRepresentation<F>,
    srs: SRS<P>,
    _f: PhantomData<F>,
    _p: PhantomData<P>,
}

impl<F: PrimeField, P: Pairing> PlonkProver<F, P> {
    pub fn new(
        transcript: FiatShamirTranscript,
        circuit_ir: PlonkishIntermediateRepresentation<F>,
        srs: SRS<P>,
    ) -> Self {
        Self {
            transcript,
            circuit_ir,
            srs,
            _f: PhantomData,
            _p: PhantomData,
        }
    }
}

impl<F: PrimeField, P: Pairing> PlonkProverInterface<F, P> for PlonkProver<F, P> {
    fn prove(&mut self, witness: &Witness<F>) -> PlonkProof<P, F> {
        // round one
        let round_one_output = self.round_one(witness);

        // commit round one output to the transcript
        self.transcript
            .append_with_label("round_one_output", round_one_output.to_bytes());

        // round 2
        let round_two_output = self.round_two(witness);

        // commit round two output to the transcript
        self.transcript
            .append_with_label("round_two_output", round_two_output.to_bytes());

        // round 3
        let round_three_output = self.round_three(witness, &round_one_output, &round_two_output);

        // commit round three output to the transcript
        self.transcript
            .append_with_label("round_three_output", round_three_output.to_bytes());

        // round 4
        let round_four_output = self.round_four(&round_one_output, &round_three_output);

        // commit round four output to the transcript
        self.transcript
            .append_with_label("round_four_output", round_four_output.to_bytes());

        // round 5
        let round_five_output = self.round_five(
            &witness,
            &round_one_output,
            &round_two_output,
            &round_three_output,
            &round_four_output,
        );

        // return the proof
        PlonkProof {
            a_poly_commitment: round_one_output.a_commitment,
            b_poly_commitment: round_one_output.b_commitment,
            c_poly_commitment: round_one_output.c_commitment,
            accumulator_poly_commitment: round_two_output.accumulator_commitment,
            t_lo_poly_commitment: round_three_output.t_lo_commitment,
            t_mid_poly_commitment: round_three_output.t_mid_commitment,
            t_hi_poly_commitment: round_three_output.t_hi_commitment,
            W_zeta_poly_commitment: round_five_output.W_zeta_poly_commitment,
            W_zeta_w_poly_commitment: round_five_output.W_zeta_w_poly_commitment,
            a_x_zeta: round_four_output.a_x_ploy_zeta,
            b_x_zeta: round_four_output.b_x_ploy_zeta,
            c_x_zeta: round_four_output.c_x_ploy_zeta,
            w_accumulator_poly_zeta: round_four_output.w_accumulator_poly_zeta,
            s1_poly_zeta: round_four_output.s1_poly_zeta,
            s2_poly_zeta: round_four_output.s2_poly_zeta,
        }
    }

    fn round_one(&mut self, witness: &Witness<F>) -> RoundOneOutput<P, F> {
        // generate 6 random element, these element provides some hiding properties for our polynomial
        let mut rng = rand::thread_rng();
        let rands = (0..6).map(|_| F::rand(&mut rng)).collect::<Vec<F>>();

        // create a vanishing polynomial; x^group_order - 1
        let vanishing_polynomial = UnivariantPolynomial::create_monomial(
            self.circuit_ir.group_order as usize,
            F::ONE,
            -F::ONE,
        );

        let witness_a_poly = witness.a.to_coefficient_poly();
        let witness_b_poly = witness.b.to_coefficient_poly();
        let witness_c_poly = witness.c.to_coefficient_poly();

        // create `a` polynomial with the vanishing polynomial; a(x) = a + ((r_1X + r2) * vanishing_polynomial)
        let a_x = (UnivariantPolynomial::create_monomial(1, rands[0], rands[1])
            * vanishing_polynomial.clone())
            + witness_a_poly.clone();

        // create `b` polynomial with the vanishing polynomial; b(x) = b + ((r_3X + r4) * vanishing_polynomial)
        let b_x = (UnivariantPolynomial::create_monomial(1, rands[2], rands[3])
            * vanishing_polynomial.clone())
            + witness_b_poly.clone();

        // create `c` polynomial with the vanishing polynomial; c(x) = c + ((r_5X + r6) * vanishing_polynomial)
        let c_x = (UnivariantPolynomial::create_monomial(1, rands[4], rands[5])
            * vanishing_polynomial.clone())
            + witness_c_poly.clone();

        // commit to the polynomials
        let a_commitment = <UnivariateKZG as KZGUnivariateInterface<P>>::commit(&self.srs, &a_x);
        let b_commitment = <UnivariateKZG as KZGUnivariateInterface<P>>::commit(&self.srs, &b_x);
        let c_commitment = <UnivariateKZG as KZGUnivariateInterface<P>>::commit(&self.srs, &c_x);

        #[cfg(test)]
        for i in 0..self.circuit_ir.group_order as usize {
            let root: F = root_of_unity::<F>(self.circuit_ir.group_order);
            assert_eq!(
                ((a_x.clone() * self.circuit_ir.QL.to_coefficient_poly())
                    + (b_x.clone() * self.circuit_ir.QR.to_coefficient_poly())
                    + (c_x.clone() * self.circuit_ir.QO.to_coefficient_poly())
                    + (a_x.clone() * b_x.clone() * self.circuit_ir.QM.to_coefficient_poly())
                    + witness.pi.to_coefficient_poly()
                    + self.circuit_ir.QC.to_coefficient_poly())
                .evaluate(&root.pow(&[i as u64])),
                F::ZERO
            );
        }

        RoundOneOutput {
            a_commitment,
            b_commitment,
            c_commitment,
            a_x,
            b_x,
            c_x,
        }
    }

    fn round_two(&mut self, witness: &Witness<F>) -> RoundTwoOutput<P, F> {
        let circuit_group_order = self.circuit_ir.group_order as usize;
        let mut accumulator = vec![F::ONE; circuit_group_order];
        let roots: Vec<F> = roots_of_unity(circuit_group_order as u64);

        // sample beta and gamma
        let rand_round_2 = self.transcript.sample_n_as_field_elements(2);
        let beta = rand_round_2[0];
        let gamma = rand_round_2[1];

        for i in 1..circuit_group_order {
            let last_index = i - 1;
            let acc = accumulator[last_index]
                * (((witness.a.values[last_index] + (beta * roots[last_index]) + gamma)
                    * (witness.b.values[last_index]
                        + (beta * F::from(2u8) * roots[last_index])
                        + gamma)
                    * (witness.c.values[last_index]
                        + (beta * F::from(3u8) * roots[last_index])
                        + gamma))
                    / ((witness.a.values[last_index]
                        + (beta * self.circuit_ir.S1.values[last_index])
                        + gamma)
                        * (witness.b.values[last_index]
                            + (beta * self.circuit_ir.S2.values[last_index])
                            + gamma)
                        * (witness.c.values[last_index]
                            + (beta * self.circuit_ir.S3.values[last_index])
                            + gamma)));

            accumulator[i] = acc;
        }

        #[cfg(test)]
        for i in 0..circuit_group_order {
            assert_eq!(
                ((witness.a.values[i] + (beta * roots[i]) + gamma)
                    * (witness.b.values[i] + (beta * F::from(2u8) * roots[i]) + gamma)
                    * (witness.c.values[i] + (beta * F::from(3u8) * roots[i]) + gamma))
                    * accumulator[i],
                ((witness.a.values[i] + (beta * self.circuit_ir.S1.values[i]) + gamma)
                    * (witness.b.values[i] + (beta * self.circuit_ir.S2.values[i]) + gamma)
                    * (witness.c.values[i] + (beta * self.circuit_ir.S3.values[i]) + gamma))
                    * accumulator[(i + 1) % circuit_group_order as usize]
            );
        }

        let domain = Domain::new(circuit_group_order);
        let accumulator_poly = UnivariateEval::interpolate(accumulator, domain);

        // blinding the accumulator polynomial
        let mut rng = rand::thread_rng();
        let rands = (0..3).map(|_| F::rand(&mut rng)).collect::<Vec<F>>();

        // create a vanishing polynomial; x^group_order - 1
        let vanishing_polynomial = UnivariantPolynomial::create_monomial(
            self.circuit_ir.group_order as usize,
            F::ONE,
            -F::ONE,
        );
        let blinding_factor =
            UnivariantPolynomial::from_coefficients_vec(vec![rands[2], rands[1], rands[0]]);
        let blinded_accumulator_poly = accumulator_poly + (blinding_factor * vanishing_polynomial);

        // commit to the blinded accumulator polynomial
        let accumulator_commitment = <UnivariateKZG as KZGUnivariateInterface<P>>::commit(
            &self.srs,
            &blinded_accumulator_poly,
        );

        RoundTwoOutput {
            accumulator_commitment,
            accumulator_poly: blinded_accumulator_poly,
            beta,
            gamma,
        }
    }

    fn round_three(
        &mut self,
        witness: &Witness<F>,
        round_one_output: &RoundOneOutput<P, F>,
        round_two_output: &RoundTwoOutput<P, F>,
    ) -> RoundThreeOutput<P, F> {
        let alpha = self.transcript.sample_as_field_element::<F>();
        let vanishing_polynomial = UnivariantPolynomial::create_monomial(
            self.circuit_ir.group_order as usize,
            F::ONE,
            -F::ONE,
        );
        let root: F = root_of_unity::<F>(self.circuit_ir.group_order);
        let mut l1_values = vec![F::ZERO; self.circuit_ir.group_order as usize];
        l1_values[0] = F::ONE;
        let domain = Domain::new(self.circuit_ir.group_order as usize);
        let l1_poly_eval = UnivariateEval::new(l1_values, domain);
        let w_accumulator_poly =
            apply_w_to_polynomial(&round_two_output.accumulator_poly.clone(), &root);
        let t_x = (((round_one_output.a_x.clone()
            * round_one_output.b_x.clone()
            * self.circuit_ir.QM.to_coefficient_poly())
            + (round_one_output.a_x.clone() * self.circuit_ir.QL.to_coefficient_poly())
            + (round_one_output.b_x.clone() * self.circuit_ir.QR.to_coefficient_poly())
            + (round_one_output.c_x.clone() * self.circuit_ir.QO.to_coefficient_poly())
            + witness.pi.to_coefficient_poly()
            + self.circuit_ir.QC.to_coefficient_poly())
            / vanishing_polynomial.clone())
            + ((((round_one_output.a_x.clone()
                + UnivariantPolynomial::create_monomial(
                    1,
                    round_two_output.beta,
                    round_two_output.gamma,
                ))
                * (round_one_output.b_x.clone()
                    + UnivariantPolynomial::create_monomial(
                        1,
                        round_two_output.beta * F::from(2u32),
                        round_two_output.gamma,
                    ))
                * (round_one_output.c_x.clone()
                    + UnivariantPolynomial::create_monomial(
                        1,
                        round_two_output.beta * F::from(3u32),
                        round_two_output.gamma,
                    ))
                * round_two_output.accumulator_poly.clone())
                * alpha)
                / vanishing_polynomial.clone())
            - ((((round_one_output.a_x.clone()
                + (self.circuit_ir.S1.to_coefficient_poly() * round_two_output.beta)
                + round_two_output.gamma)
                * (round_one_output.b_x.clone()
                    + (self.circuit_ir.S2.to_coefficient_poly() * round_two_output.beta)
                    + round_two_output.gamma)
                * (round_one_output.c_x.clone()
                    + (self.circuit_ir.S3.to_coefficient_poly() * round_two_output.beta)
                    + round_two_output.gamma)
                * w_accumulator_poly.clone())
                * alpha)
                / vanishing_polynomial.clone())
            + ((((round_two_output.accumulator_poly.clone() - F::ONE)
                * (l1_poly_eval.to_coefficient_poly()))
                * alpha.pow(&[2 as u64]))
                / vanishing_polynomial);

        let (t_lo, t_mid, t_hi) = split_poly_in_3(&t_x, self.circuit_ir.group_order as usize);

        let mut rng = rand::thread_rng();
        let rands = (0..2).map(|_| F::rand(&mut rng)).collect::<Vec<F>>();
        let b_10 = rands[0];
        let b_11 = rands[1];

        let mut x_n_coeffs = vec![F::ZERO; self.circuit_ir.group_order as usize + 1];
        x_n_coeffs[self.circuit_ir.group_order as usize] = F::ONE;

        let mut x_2n_coeffs = vec![F::ZERO; self.circuit_ir.group_order as usize * 2 + 1];
        x_2n_coeffs[self.circuit_ir.group_order as usize * 2] = F::ONE;

        #[cfg(test)]
        assert_eq!(
            t_x.clone(),
            t_lo.clone()
                + (UnivariantPolynomial::new(x_n_coeffs.clone()) * t_mid.clone())
                + (UnivariantPolynomial::new(x_2n_coeffs) * t_hi.clone()),
        );

        let t_lo_blinded = t_lo.clone() + (UnivariantPolynomial::new(x_n_coeffs.clone()) * b_10);
        let t_mid_blinded =
            t_mid.clone() + (UnivariantPolynomial::new(x_n_coeffs.clone()) * b_11 - b_10);
        let t_hi_blinding = t_hi.clone() + b_11.neg();

        let t_lo_commitment =
            <UnivariateKZG as KZGUnivariateInterface<P>>::commit(&self.srs, &t_lo_blinded);
        let t_mid_commitment =
            <UnivariateKZG as KZGUnivariateInterface<P>>::commit(&self.srs, &t_mid_blinded);
        let t_hi_commitment =
            <UnivariateKZG as KZGUnivariateInterface<P>>::commit(&self.srs, &t_hi_blinding);

        RoundThreeOutput {
            t_lo_commitment,
            t_mid_commitment,
            t_hi_commitment,
            w_accumulator_poly,
            t_lo_poly: t_lo_blinded,
            t_mid_poly: t_mid_blinded,
            t_hi_poly: t_hi_blinding,
            alpha,
        }
    }

    fn round_four(
        &mut self,
        round_one_output: &RoundOneOutput<P, F>,
        round_three_output: &RoundThreeOutput<P, F>,
    ) -> RoundFourOutput<F> {
        let zeta = self.transcript.sample_as_field_element::<F>();

        let a_x_poly = round_one_output.a_x.clone();
        let b_x_poly = round_one_output.b_x.clone();
        let c_x_poly = round_one_output.c_x.clone();
        let w_accumulator_poly = round_three_output.w_accumulator_poly.clone();
        let s1_poly = self.circuit_ir.S1.to_coefficient_poly();
        let s2_poly = self.circuit_ir.S2.to_coefficient_poly();

        let a_x_ploy_zeta = a_x_poly.evaluate(&zeta);
        let b_x_ploy_zeta = b_x_poly.evaluate(&zeta);
        let c_x_ploy_zeta = c_x_poly.evaluate(&zeta);
        let w_accumulator_poly_zeta = w_accumulator_poly.evaluate(&zeta);
        let s1_poly_zeta = s1_poly.evaluate(&zeta);
        let s2_poly_zeta = s2_poly.evaluate(&zeta);

        RoundFourOutput {
            a_x_ploy_zeta,
            b_x_ploy_zeta,
            c_x_ploy_zeta,
            w_accumulator_poly_zeta,
            s1_poly_zeta,
            s2_poly_zeta,
            zeta,
        }
    }

    fn round_five(
        &mut self,
        witness: &Witness<F>,
        round_one_output: &RoundOneOutput<P, F>,
        round_two_output: &RoundTwoOutput<P, F>,
        round_three_output: &RoundThreeOutput<P, F>,
        round_four_output: &RoundFourOutput<F>,
    ) -> RoundFiveOutput<P, F> {
        let vinculum = self.transcript.sample_as_field_element::<F>();

        let a_x_ploy_zeta = round_four_output.a_x_ploy_zeta;
        let b_x_ploy_zeta = round_four_output.b_x_ploy_zeta;
        let c_x_ploy_zeta = round_four_output.c_x_ploy_zeta;
        let w_accumulator_poly_zeta = round_four_output.w_accumulator_poly_zeta;
        let s1_poly_zeta = round_four_output.s1_poly_zeta;
        let s2_poly_zeta = round_four_output.s2_poly_zeta;

        let alpha = round_three_output.alpha;
        let beta = round_two_output.beta;
        let gamma = round_two_output.gamma;
        let zeta = round_four_output.zeta;

        let a_x_poly = round_one_output.a_x.clone();
        let b_x_poly = round_one_output.b_x.clone();
        let c_x_poly = round_one_output.c_x.clone();
        let accumulator_poly = round_two_output.accumulator_poly.clone();
        let s1_poly = self.circuit_ir.S1.to_coefficient_poly();
        let s2_poly = self.circuit_ir.S2.to_coefficient_poly();

        let mut l1_values = vec![F::ZERO; self.circuit_ir.group_order as usize];
        l1_values[0] = F::ONE;
        let domain = Domain::new(self.circuit_ir.group_order as usize);
        let l1_poly_eval = UnivariateEval::new(l1_values, domain);

        let vanishing_polynomial = UnivariantPolynomial::create_monomial(
            self.circuit_ir.group_order as usize,
            F::ONE,
            -F::ONE,
        );
        let root: F = root_of_unity::<F>(self.circuit_ir.group_order);

        let r_poly = ((self.circuit_ir.QM.to_coefficient_poly() * a_x_ploy_zeta * b_x_ploy_zeta)
            + (self.circuit_ir.QL.to_coefficient_poly() * a_x_ploy_zeta)
            + (self.circuit_ir.QR.to_coefficient_poly() * b_x_ploy_zeta)
            + (self.circuit_ir.QO.to_coefficient_poly() * c_x_ploy_zeta)
            + witness.pi.to_coefficient_poly().evaluate(&zeta)
            + self.circuit_ir.QC.to_coefficient_poly())
            + (((accumulator_poly.clone()
                * (a_x_ploy_zeta + (beta * zeta) + gamma)
                * (b_x_ploy_zeta + (beta * F::from(2u8) * zeta) + gamma)
                * (c_x_ploy_zeta + (beta * F::from(3u8) * zeta) + gamma))
                - (((self.circuit_ir.S3.to_coefficient_poly() * beta) + c_x_ploy_zeta + gamma)
                    * (a_x_ploy_zeta + (beta * s1_poly_zeta) + gamma)
                    * (b_x_ploy_zeta + (beta * s2_poly_zeta) + gamma)
                    * w_accumulator_poly_zeta))
                * alpha)
            + (((accumulator_poly.clone() - F::ONE)
                * (l1_poly_eval.to_coefficient_poly().evaluate(&zeta)))
                * alpha.pow(&[2 as u64]))
            - ((round_three_output.t_lo_poly.clone()
                + (round_three_output.t_mid_poly.clone()
                    * zeta.pow(&[self.circuit_ir.group_order]))
                + (round_three_output.t_hi_poly.clone()
                    * zeta.pow(&[2 * self.circuit_ir.group_order])))
                * vanishing_polynomial.evaluate(&zeta));

        let x_minus_zeta = UnivariantPolynomial::new(vec![-zeta, F::ONE]);
        let W_zeta_poly = (r_poly
            + ((a_x_poly.clone() - a_x_ploy_zeta) * vinculum)
            + ((b_x_poly.clone() - b_x_ploy_zeta) * vinculum.pow(&[2 as u64]))
            + ((c_x_poly.clone() - c_x_ploy_zeta) * vinculum.pow(&[3 as u64]))
            + ((s1_poly.clone() - s1_poly_zeta) * vinculum.pow(&[4 as u64]))
            + ((s2_poly.clone() - s2_poly_zeta) * vinculum.pow(&[5 as u64])))
            / x_minus_zeta;

        let x_minus_w_zeta = UnivariantPolynomial::new(vec![-(root * zeta), F::ONE]);
        let W_zeta_w_poly = (accumulator_poly - w_accumulator_poly_zeta) / x_minus_w_zeta;

        let W_zeta_poly_commitment =
            <UnivariateKZG as KZGUnivariateInterface<P>>::commit(&self.srs, &W_zeta_poly);
        let W_zeta_w_poly_commitment =
            <UnivariateKZG as KZGUnivariateInterface<P>>::commit(&self.srs, &W_zeta_w_poly);

        let mu = self.transcript.sample_as_field_element::<F>();

        RoundFiveOutput {
            W_zeta_poly_commitment,
            W_zeta_w_poly_commitment,
            mu,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_test_curves::bls12_381::{Bls12_381, Fr};
    use fiat_shamir::FiatShamirTranscript;
    use plonk_compiler::{assembly::eq_to_assembly, program::Program};
    use std::collections::HashMap;

    #[test]
    fn test_plonk_complete_prove() {
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
        let mut prover = PlonkProver::new(transcript, circuit_ir, srs);

        let _ = prover.prove(&witness);
    }

    #[test]
    fn test_plonk_prove_round_1() {
        let original_constriants = ["e public"];
        let mut assembly_eqns = Vec::new();
        for eq in original_constriants.iter() {
            let assembly_eqn = eq_to_assembly::<Fr>(eq.to_string());
            assembly_eqns.push(assembly_eqn);
        }
        let program = Program::new(assembly_eqns, 8);

        let mut variable_assignment = HashMap::new();
        variable_assignment.insert(Some("a".to_string()), Fr::from(1));
        variable_assignment.insert(Some("b".to_string()), Fr::from(2));
        variable_assignment.insert(Some("c".to_string()), Fr::from(2));
        variable_assignment.insert(Some("d".to_string()), Fr::from(4));
        variable_assignment.insert(Some("e".to_string()), Fr::from(8));

        let witness = program.compute_witness_and_public_parameter(variable_assignment);
        let circuit_ir = program.common_preproccessed_input();

        let transcript = FiatShamirTranscript::new("plonk-protocol".as_bytes().to_vec());
        let srs: SRS<Bls12_381> =
            UnivariateKZG::generate_srs(&Fr::from(6), program.group_order as usize * 4);
        let mut prover = PlonkProver::new(transcript, circuit_ir, srs);

        let _ = prover.round_one(&witness);
    }
}
