pub mod multicomposed;
pub mod prover;
pub mod utils;
pub mod verifier;

use ark_ff::PrimeField;
use polynomial::{
    composed::multilinear::ComposedMultilinear,
    interface::{MultilinearPolynomialInterface, UnivariantPolynomialInterface},
    multilinear::Multilinear,
    univariant::UnivariantPolynomial,
    utils::compute_domain,
};

/// This struct is used to store the sum check proof
#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct ComposedSumCheckProof<F: PrimeField> {
    /// This vector stores the round polynomials
    pub round_poly: Vec<UnivariantPolynomial<F>>,
    /// This holds the sum of the polynomial evaluation over the boolean hypercube
    pub sum: F,
}

#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct RoundPoly<F: PrimeField> {
    /// This is a vector of points that would be interpolated against a domain of real numbers to obtain the round polynomial
    pub poly_vec: Vec<F>,
}

impl<F: PrimeField> RoundPoly<F> {
    pub fn new(poly_vec: Vec<F>) -> Self {
        Self { poly_vec }
    }

    pub fn interpolate(&self) -> UnivariantPolynomial<F> {
        let domain = compute_domain(self.poly_vec.len(), 0);
        UnivariantPolynomial::interpolate(self.poly_vec.clone(), domain)
    }

    pub fn rep_in_eval(&self) -> Multilinear<F> {
        Multilinear::new(self.poly_vec.clone(), 1)
    }
}
