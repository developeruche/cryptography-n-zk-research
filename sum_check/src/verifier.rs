use crate::{data_structure::SumCheckProof, interface::VerifierInterface};
use ark_ff::Field;
use fiat_shamir::FiatShamirTranscript;

#[derive(Clone, Default, Debug)]
pub struct Verifier<F: Field> {
    /// This is this fiat-shamir challenge transcript
    pub transcript: FiatShamirTranscript,
    phantom: std::marker::PhantomData<F>,
}

impl<F: Field> Verifier<F> {
    /// This function creates a new verifier instance
    pub fn new() -> Self {
        Self {
            transcript: FiatShamirTranscript::default(),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<F: Field> VerifierInterface<F> for Verifier<F> {
    /// This function verifies the sum check proof
    fn verify(&self, proof: &SumCheckProof<F>) -> bool {
        unimplemented!("Implement this function")
    }
}
