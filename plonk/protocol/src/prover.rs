use std::marker::PhantomData;

use crate::interface::{PlonkPCSInterface, PlonkProverInterface, PlonkTranscriptInterface};
use ark_ec::pairing::Pairing;
use ark_ff::PrimeField;
use plonk_core::primitives::{PlonkProof, Witness};

pub struct PlonkProver<
    T: PlonkTranscriptInterface<F>,
    F: PrimeField,
    P: Pairing,
    PC: PlonkPCSInterface<P>,
> {
    transcript: T,
    pcs: PC,
    _f: PhantomData<F>,
    _p: PhantomData<P>,
}

impl<T: PlonkTranscriptInterface<F>, F: PrimeField, P: Pairing, PC: PlonkPCSInterface<P>>
    PlonkProver<T, F, P, PC>
{
    pub fn new(transcript: T, pcs: PC) -> Self {
        Self {
            transcript,
            pcs,
            _f: PhantomData,
            _p: PhantomData,
        }
    }
}

impl<T: PlonkTranscriptInterface<F>, F: PrimeField, P: Pairing, PC: PlonkPCSInterface<P>>
    PlonkProverInterface<F, P> for PlonkProver<T, F, P, PC>
{
    fn prove(&self, witness: &Witness<F>) -> PlonkProof<F> {
        todo!()
    }
}
