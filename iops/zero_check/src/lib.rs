use ark_ff::PrimeField;
use fiat_shamir::FiatShamirTranscript;
use interface::ZeroCheckInterface;
use polynomial::composed::multilinear::ComposedMultilinear;
use std::marker::PhantomData;
use sum_check::composed::ComposedSumCheckProof;
pub mod interface;
pub mod primitives;

/// Struct used to create a instance of the zero check protocol.
pub struct ZeroCheck<F: PrimeField> {
    _phantom: PhantomData<F>,
}

impl<F: PrimeField> ZeroCheckInterface for ZeroCheck<F> {
    type Poly = ComposedMultilinear<F>;
    type SubClaim = primitives::ZeroCheckSubClaim<F>;
    type Proof = ComposedSumCheckProof<F>;
    type Transcript = FiatShamirTranscript;

    fn prove(
        poly: &Self::Poly,
        transcript: &mut Self::Transcript,
    ) -> Result<Self::Proof, anyhow::Error> {
        todo!()
    }

    fn verify(
        proof: &Self::Proof,
        poly: &Self::Poly,
        transcript: &mut Self::Transcript,
    ) -> Result<bool, anyhow::Error> {
        todo!()
    }
}
