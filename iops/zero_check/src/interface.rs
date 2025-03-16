//! This interface defines the ZeroCheck trait.
use std::fmt::Debug;

/// Trait for the zero check poip
pub trait ZeroCheckInterface {
    type Poly: Clone + Debug + Default + PartialEq;
    type SubClaim: Clone + Debug + Default + PartialEq;
    type Proof: Clone + Debug + Default + PartialEq;
    type Transcript: fiat_shamir::TranscriptInterface;

    /// Prove the zero check
    fn prove(
        poly: &Self::Poly,
        transcript: &mut Self::Transcript,
    ) -> Result<Self::Proof, anyhow::Error>;
    /// Verify the zero check proof
    fn verify(
        proof: &Self::Proof,
        poly: &Self::Poly,
        transcript: &mut Self::Transcript,
    ) -> Result<bool, anyhow::Error>;
}
