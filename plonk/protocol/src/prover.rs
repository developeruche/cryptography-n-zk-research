use crate::interface::{PlonkProverInterface, PlonkTranscriptInterface};
use ark_ec::pairing::Pairing;
use ark_ff::PrimeField;
use kzg_rust::{interface::KZGUnivariateInterface, primitives::SRS, univariate::UnivariateKZG};
use plonk_core::primitives::{
    PlonkProof, PlonkishIntermediateRepresentation, RoundOneOutput, Witness,
};
use polynomial::univariant::UnivariantPolynomial;
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
    fn prove(&self, witness: Witness<F>) -> PlonkProof<F> {
        let round_one_output = self.round_one(witness);

        todo!()
    }

    fn round_one(&self, witness: Witness<F>) -> RoundOneOutput<P> {
        // generate 6 random element, these element provides some hiding properties for our polynomial
        let mut rng = rand::thread_rng();
        let rands = [0..6].iter().map(|_| F::rand(&mut rng)).collect::<Vec<F>>();

        let vanishing_polynomial = UnivariantPolynomial::create_monomial(
            self.circuit_ir.group_order as usize,
            F::ONE,
            -F::ONE,
        );

        let a_x = (UnivariantPolynomial::create_monomial(1, rands[0], rands[1])
            * vanishing_polynomial.clone())
            + witness.a;

        let b_x = (UnivariantPolynomial::create_monomial(1, rands[2], rands[3])
            * vanishing_polynomial.clone())
            + witness.b;

        let c_x = (UnivariantPolynomial::create_monomial(1, rands[4], rands[5])
            * vanishing_polynomial.clone())
            + witness.c;

        let a_commitment = <UnivariateKZG as KZGUnivariateInterface<P>>::commit(&self.srs, &a_x);
        let b_commitment = <UnivariateKZG as KZGUnivariateInterface<P>>::commit(&self.srs, &b_x);
        let c_commitment = <UnivariateKZG as KZGUnivariateInterface<P>>::commit(&self.srs, &c_x);

        RoundOneOutput {
            a_commitment,
            b_commitment,
            c_commitment,
        }
    }
}
