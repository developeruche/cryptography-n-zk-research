use crate::interface::PlonkProverInterface;
use ark_ec::pairing::Pairing;
use ark_ff::PrimeField;
use fiat_shamir::{interface::TranscriptInterface, FiatShamirTranscript};
use kzg_rust::{interface::KZGUnivariateInterface, primitives::SRS, univariate::UnivariateKZG};
use plonk_compiler::utils::{root_of_unity, roots_of_unity};
use plonk_core::primitives::{
    PlonkProof, PlonkishIntermediateRepresentation, RoundOneOutput, RoundThreeOutput,
    RoundTwoOutput, Witness,
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
    fn prove(&mut self, witness: Witness<F>) -> PlonkProof<F> {
        // round one
        let round_one_output = self.round_one(witness);

        // commit round one output to the transcript
        self.transcript
            .append_with_label("round_one_output", round_one_output.to_bytes());

        // round 2

        todo!()
    }

    fn round_one(&mut self, witness: Witness<F>) -> RoundOneOutput<P, F> {
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

    fn round_two(&mut self, witness: Witness<F>) -> RoundTwoOutput<P, F> {
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
            beta,
            gamma,
        }
    }

    fn round_three(
        &mut self,
        raw_witness: Witness<F>,
        round_one_output: RoundOneOutput<P, F>,
        round_two_output: RoundTwoOutput<P, F>,
    ) -> RoundThreeOutput<P> {
        // sampling alpha
        let alpha = self.transcript.sample_as_field_element::<F>();
        todo!()
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
    fn test_plonk_prove_round_1() {
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

        let transcript = FiatShamirTranscript::new("test_plonk_prove_round_1".as_bytes().to_vec());
        let srs: SRS<Bls12_381> =
            UnivariateKZG::generate_srs(&Fr::from(6), program.group_order as usize * 4);
        let mut prover = PlonkProver::new(transcript, circuit_ir, srs);

        let round_one_output = prover.round_one(witness);
    }

    #[test]
    #[ignore]
    fn test_plonk_prove_round_2() {
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

        let transcript = FiatShamirTranscript::new("test_plonk_prove_round_1".as_bytes().to_vec());
        let srs: SRS<Bls12_381> =
            UnivariateKZG::generate_srs(&Fr::from(6), program.group_order as usize * 4);
        let mut prover = PlonkProver::new(transcript, circuit_ir, srs);

        let round_one_output = prover.round_one(witness);
    }
}
