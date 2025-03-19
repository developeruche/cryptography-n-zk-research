//! This module contains traits discribing the product check protocol
use std::fmt::Debug;

pub trait ProductCheckInterface {
    type Poly: Clone + Debug + Default + PartialEq;
    type KZGSRS: Clone + Debug + PartialEq;
    type Transcript: fiat_shamir::TranscriptInterface;
    type Proof: Clone + Debug + PartialEq;
    type Multilinear: Clone + Debug + PartialEq;
    type FinalQueryAndEval: Clone + Debug + PartialEq;

    /// Prove the product check protocol
    /// Employing KZG to make the protocol succinct, the oracle should not be set to the verifier
    fn prove(
        poly_1: &Self::Poly,
        poly_2: &Self::Poly,
        kzg_srs: &Self::KZGSRS,
        transcript: &mut Self::Transcript,
    ) -> Result<
        (
            Self::Proof,
            Self::Multilinear,
            Self::Multilinear,
            Self::Poly,
        ),
        anyhow::Error,
    >;

    /// Verify the product check protocol
    fn verify(
        proof: &Self::Proof,
        q_x: &Self::Poly,
        transcript: &mut Self::Transcript,
    ) -> Result<Self::FinalQueryAndEval, anyhow::Error>;
}
