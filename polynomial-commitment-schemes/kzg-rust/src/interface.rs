use crate::primitives::{MultiLinearSRS, SRS};
use ark_ec::pairing::Pairing;
use ark_ff::PrimeField;
use polynomial::{multilinear::Multilinear, univariant::UnivariantPolynomial};

/// This trait is used by it implementing struct to create a new SRS, taking in the string representation of the SRS or the fs-path to the SRS file.
pub trait FromStringToSRS<P: Pairing> {
    fn from_string_to_srs(&self, srs: String) -> SRS<P>;
}

pub trait KZGUnivariateInterface<P: Pairing> {
    /// This function is used to generate a new SRS.
    fn generate_srs(tau: &P::ScalarField, poly_degree: usize) -> SRS<P>;
    /// Commit to a polynomial would degree is less than the degree of the SRS
    fn commit(srs: &SRS<P>, poly: &UnivariantPolynomial<P::ScalarField>) -> P::G1;
    /// Open a polynomial at a point
    fn open<F: PrimeField>(srs: &SRS<P>, poly: &UnivariantPolynomial<F>, point: &F) -> (F, P::G1);
    /// Verify polynomial evaluation
    fn verify<F: PrimeField>(
        srs: &SRS<P>,
        commitment: &P::G1,
        point: &F,
        point_evaluation: &F,
        proof: &P::G1,
    ) -> bool;
}

pub trait KZGMultiLinearInterface<P: Pairing> {
    /// This function is used to generate a new SRS.
    fn generate_srs<F: PrimeField>(taus: &[F]) -> MultiLinearSRS<P>;
    /// This function is used to commit to the poly using a provided SRS
    fn commit<F: PrimeField>(srs: &MultiLinearSRS<P>, poly: &Multilinear<F>) -> P::G1;
    /// This function is used to open the polynomial at various a point
    fn open<F: PrimeField>(
        srs: &MultiLinearSRS<P>,
        poly: &Multilinear<F>,
        point: &[F],
    ) -> (F, Vec<P::G1>);
    /// This function is used to verify the polynomial evaluation
    fn verify<F: PrimeField>(
        srs: &MultiLinearSRS<P>,
        commitment: &P::G1,
        point: &[F],
        point_evaluation: &F,
        proof: &Vec<P::G1>,
    ) -> bool;
}

pub trait BatchKZGUnivariateInterface<P: Pairing> {
    /// This function is used to generate a new SRS.
    fn generate_srs(tau: &P::ScalarField, poly_degree: usize) -> SRS<P>;
    /// Commit to a polynomial would degree is less than the degree of the SRS
    fn commit(srs: &SRS<P>, poly: &UnivariantPolynomial<P::ScalarField>) -> P::G1;
    /// Open a polynomial at a point
    fn open<F: PrimeField>(srs: &SRS<P>, poly: &UnivariantPolynomial<F>, point: &F) -> (F, P::G1);
    /// Verify polynomial evaluation
    fn verify<F: PrimeField>(
        srs: &SRS<P>,
        commitment: &P::G1,
        point: &F,
        point_evaluation: &F,
        proof: &P::G1,
    ) -> bool;
}