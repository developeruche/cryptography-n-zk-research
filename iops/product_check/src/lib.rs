//! This module contains implementations of product check protocol.

use ark_ec::pairing::Pairing;
use fiat_shamir::FiatShamirTranscript;
use interface::ProductCheckInterface;
use pcs::primitives::MultiLinearSRS;
use polynomial::composed::multilinear::ComposedMultilinear;
use primitives::ProductCheckProof;
pub mod interface;
pub mod multilinear;
pub mod primitives;
pub mod utils;

/// Struct used to instantiate product check protocol.
pub struct ProductCheck<P: Pairing> {
    _marker: std::marker::PhantomData<P>,
}

impl<P: Pairing> ProductCheckInterface for ProductCheck<P> {
    type Poly = ComposedMultilinear<P::ScalarField>;
    type KZGSRS = MultiLinearSRS<P>;
    type Transcript = FiatShamirTranscript;
    type Proof = ProductCheckProof<P>;

    fn prove(
        &self,
        poly_1: &Self::Poly,
        poly_2: &Self::Poly,
        kzg_srs: &Self::KZGSRS,
        transcript: &mut Self::Transcript,
    ) -> Result<Self::Proof, anyhow::Error> {
        // performing some sanity checks
        // Dimension of the composed polynomials is same
        if poly_1.polys.len() != poly_2.polys.len() {
            return Err(anyhow::anyhow!("Polynomials have different lengths"));
        }

        let fractional_poly = ();

        todo!()
    }
}
