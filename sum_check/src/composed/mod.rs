pub mod prover;
pub mod verifier;

use ark_ff::PrimeField;
use polynomial::{
    interface::{MultilinearPolynomialInterface, UnivariantPolynomialInterface},
    multilinear::Multilinear,
    univariant::UnivariantPolynomial,
    utils::compute_domain,
};

/// This struct is used to store the sum check proof
#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct SumCheckProof<F: PrimeField, P: MultilinearPolynomialInterface<F>> {
    /// This is the polynomial that is used to generate the sum check proof
    pub polynomial: P,
    /// This vector stores the round polynomials
    pub round_poly: Vec<RoundPoly<F>>,
    /// This vectors store the polynomial from the first round
    pub round_0_poly: P,
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

    pub fn interpolate(&self, domain: Vec<F>) -> UnivariantPolynomial<F> {
        let domain = compute_domain(self.poly_vec.len(), 0);
        UnivariantPolynomial::interpolate(domain, self.poly_vec.clone())
    }
}
