//! This module contains traits discribing the product check protocol
use std::fmt::Debug;

pub trait ProductCheckInterface {
    type Poly: Clone + Debug + Default + PartialEq;
    type KZGSRS: Clone + Debug + PartialEq;
    type Transcript: fiat_shamir::TranscriptInterface;
    type Proof: Clone + Debug + PartialEq;

    /// Prove the product check protocol
    /// Employing KZG to make the protocol succinct, the oracle should not be set to the verifier
    fn prove(
        &self,
        poly_1: &Self::Poly,
        poly_2: &Self::Poly,
        kzg_srs: &Self::KZGSRS,
        transcript: &mut Self::Transcript,
    ) -> Result<Self::Proof, anyhow::Error>;
}
