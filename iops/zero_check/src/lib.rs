//! This module contains the implementation of the zero check protocol.
//! this protocol would be perfromed over the `ComposedMultilinear` polynomial variant
//! this is done so the zero check piop can run on product of multilinear polynomials
use ark_ff::PrimeField;
use fiat_shamir::{FiatShamirTranscript, TranscriptInterface};
use interface::ZeroCheckInterface;
use polynomial::{
    composed::{interfaces::ComposedMultilinearInterface, multilinear::ComposedMultilinear},
    interface::MultilinearPolynomialInterface,
};
use std::marker::PhantomData;
use sum_check::{
    composed::{ComposedSumCheckProof, prover::ComposedProver, verifier::ComposedVerifier},
    interface::{ComposedProverInterface, ComposedVerifierInterface},
};
use utils::generate_eq_poly;
pub mod interface;
pub mod primitives;
pub mod utils;

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
        let r_s = transcript.sample_n_as_field_elements(poly.num_vars());
        let eq_poly = generate_eq_poly(&r_s);

        // f(x) = poly(x) * eq_poly(x)
        let f = poly.mul_by_mle(&eq_poly);

        let (proof, _) = ComposedProver::sum_check_proof(&f, transcript, &F::ZERO);

        Ok(proof)
    }

    fn verify(
        proof: &Self::Proof,
        poly: &Self::Poly,
        transcript: &mut Self::Transcript,
    ) -> Result<bool, anyhow::Error> {
        if !ComposedVerifier::verify(proof, poly, transcript) {
            return Err(anyhow::anyhow!("Zero check Verification failed"));
        }

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_test_curves::bls12_381::Fr;
    use polynomial::multilinear::Multilinear;

    #[test]
    fn test_zero_check() {
        let zero_poly = ComposedMultilinear::new(vec![
            Multilinear::new(vec![Fr::from(0), Fr::from(0)], 1),
            Multilinear::new(vec![Fr::from(0), Fr::from(0)], 1),
        ]);
        let mut transcript = FiatShamirTranscript::default();

        let proof = ZeroCheck::prove(&zero_poly, &mut transcript).unwrap();
        let mut transcript_ = FiatShamirTranscript::default();
        let result = ZeroCheck::verify(&proof, &zero_poly, &mut transcript_).unwrap();

        assert!(result);
    }

    #[test]
    fn test_zero_check_non_zero() {
        let zero_poly = ComposedMultilinear::new(vec![
            Multilinear::new(vec![Fr::from(1), Fr::from(0)], 1),
            Multilinear::new(vec![Fr::from(1), Fr::from(0)], 1),
        ]);
        let mut transcript = FiatShamirTranscript::default();

        let proof = ZeroCheck::prove(&zero_poly, &mut transcript).unwrap();
        let mut transcript_ = FiatShamirTranscript::default();

        assert!(ZeroCheck::verify(&proof, &zero_poly, &mut transcript_).is_err());
    }
}
