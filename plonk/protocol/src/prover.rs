use crate::interface::{PlonkProverInterface, PlonkTranscriptInterface};
use ark_ec::pairing::Pairing;
use ark_ff::PrimeField;
use kzg_rust::{interface::KZGUnivariateInterface, primitives::SRS, univariate::UnivariateKZG};
use plonk_compiler::utils::roots_of_unity;
use plonk_core::primitives::{
    PlonkProof, PlonkishIntermediateRepresentation, RoundOneOutput, Witness, WitnessRaw,
};
use polynomial::{
    evaluation::{univariate::UnivariateEval, Domain},
    interface::UnivariantPolynomialInterface,
    univariant::UnivariantPolynomial,
};
use std::marker::PhantomData;

pub struct PlonkProver<
    T: PlonkTranscriptInterface<F>,
    F: PrimeField,
    P: Pairing,
    PC: KZGUnivariateInterface<P>,
> {
    transcript: T,
    pcs: PC,
    circuit_ir: PlonkishIntermediateRepresentation<F>,
    srs: SRS<P>,
    _f: PhantomData<F>,
    _p: PhantomData<P>,
}

impl<T: PlonkTranscriptInterface<F>, F: PrimeField, P: Pairing, PC: KZGUnivariateInterface<P>>
    PlonkProver<T, F, P, PC>
{
    pub fn new(
        transcript: T,
        pcs: PC,
        circuit_ir: PlonkishIntermediateRepresentation<F>,
        srs: SRS<P>,
    ) -> Self {
        Self {
            transcript,
            pcs,
            circuit_ir,
            srs,
            _f: PhantomData,
            _p: PhantomData,
        }
    }
}

impl<T: PlonkTranscriptInterface<F>, F: PrimeField, P: Pairing, PC: KZGUnivariateInterface<P>>
    PlonkProverInterface<F, P> for PlonkProver<T, F, P, PC>
{
    fn prove(&mut self, witness: Witness<F>) -> PlonkProof<F> {
        // round one
        let round_one_output = self.round_one(witness);

        // commit round one output to the transcript
        self.transcript
            .append("round_one_output", round_one_output.to_bytes());

        // round 2

        todo!()
    }

    fn round_one(&mut self, witness: Witness<F>) -> RoundOneOutput<P> {
        // generate 6 random element, these element provides some hiding properties for our polynomial
        let mut rng = rand::thread_rng();
        let rands = [0..6].iter().map(|_| F::rand(&mut rng)).collect::<Vec<F>>();

        // create a vanishing polynomial; x^group_order - 1
        let vanishing_polynomial = UnivariantPolynomial::create_monomial(
            self.circuit_ir.group_order as usize,
            F::ONE,
            -F::ONE,
        );

        // create `a` polynomial with the vanishing polynomial; a(x) = a + ((r_1X + r2) * vanishing_polynomial)
        let a_x = (UnivariantPolynomial::create_monomial(1, rands[0], rands[1])
            * vanishing_polynomial.clone())
            + witness.a;

        // create `b` polynomial with the vanishing polynomial; b(x) = b + ((r_3X + r4) * vanishing_polynomial)
        let b_x = (UnivariantPolynomial::create_monomial(1, rands[2], rands[3])
            * vanishing_polynomial.clone())
            + witness.b;

        // create `c` polynomial with the vanishing polynomial; c(x) = c + ((r_5X + r6) * vanishing_polynomial)
        let c_x = (UnivariantPolynomial::create_monomial(1, rands[4], rands[5])
            * vanishing_polynomial.clone())
            + witness.c;

        // commit to the polynomials
        let a_commitment = <UnivariateKZG as KZGUnivariateInterface<P>>::commit(&self.srs, &a_x);
        let b_commitment = <UnivariateKZG as KZGUnivariateInterface<P>>::commit(&self.srs, &b_x);
        let c_commitment = <UnivariateKZG as KZGUnivariateInterface<P>>::commit(&self.srs, &c_x);

        RoundOneOutput {
            a_commitment,
            b_commitment,
            c_commitment,
        }
    }

    fn round_two(&mut self, raw_witness: WitnessRaw<F>) -> P::G1 {
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
                * (((raw_witness.a[last_index] + (beta * roots[last_index]) + gamma)
                    * (raw_witness.b[last_index]
                        + (beta * F::from(2u8) * roots[last_index])
                        + gamma)
                    * (raw_witness.c[last_index]
                        + (beta * F::from(3u8) * roots[last_index])
                        + gamma))
                    / ((raw_witness.a[last_index]
                        + (beta * self.circuit_ir.S1.values[last_index])
                        + gamma)
                        * (raw_witness.b[last_index]
                            + (beta * self.circuit_ir.S2.values[last_index])
                            + gamma)
                        * (raw_witness.c[last_index]
                            + (beta * self.circuit_ir.S3.values[last_index])
                            + gamma)));

            accumulator[i] = acc;
        }

        let domain = Domain::new(circuit_group_order);
        let accumulator_poly = UnivariateEval::interpolate(accumulator, domain);

        // blinding the accumulator polynomial
        let mut rng = rand::thread_rng();
        let rands = [0..3].iter().map(|_| F::rand(&mut rng)).collect::<Vec<F>>();

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

        accumulator_commitment
    }
}
