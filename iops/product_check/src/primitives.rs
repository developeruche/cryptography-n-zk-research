//! This module holds primitives used in the product check protocol.

use ark_ec::pairing::Pairing;
use sum_check::composed::ComposedSumCheckProof;

/// Represents a product check proof.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ProductCheckProof<P: Pairing> {
    pub zero_check_proof: ComposedSumCheckProof<P::ScalarField>,
    pub product_poly_commitment: P::G1,
    pub fractional_poly_commitment: P::G1,
}
