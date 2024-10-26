#![allow(non_snake_case)]

use ark_ec::pairing::Pairing;
use ark_ff::PrimeField;
use kzg_rust::{interface::KZGUnivariateInterface, primitives::SRS, univariate::UnivariateKZG};
use polynomial::{evaluation::univariate::UnivariateEval, univariant::UnivariantPolynomial};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlonkSRS<P: Pairing> {
    pub g1_power_of_taus: Vec<P::G1>,
    // making this a vec also so batch kzg polynomial commitment can be used also
    pub g2_power_of_tau: Vec<P::G2>,
}

/// This is an intermediate representation of the plonk protocol circuit
/// showing how the circuit is represented in the plonk protocol
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlonkishIntermediateRepresentation<F: PrimeField> {
    // q_M(X) multiplication selector polynomial
    pub QM: UnivariateEval<F>,
    // q_L(X) left selector polynomial
    pub QL: UnivariateEval<F>,
    // q_R(X) right selector polynomial
    pub QR: UnivariateEval<F>,
    // q_O(X) output selector polynomial
    pub QO: UnivariateEval<F>,
    // q_C(X) constants selector polynomial
    pub QC: UnivariateEval<F>,
    // S_σ1(X) first permutation polynomial S_σ1(X)
    pub S1: UnivariateEval<F>,
    // S_σ2(X) second permutation polynomial S_σ2(X)
    pub S2: UnivariateEval<F>,
    // S_σ3(X) third permutation polynomial S_σ3(X)
    pub S3: UnivariateEval<F>,
    // order of group
    pub group_order: u64,
}

/// This struct is used to represent the witness of the polynomial
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Witness<F: PrimeField> {
    pub a: UnivariateEval<F>,
    pub b: UnivariateEval<F>,
    pub c: UnivariateEval<F>,
    pub pi: UnivariateEval<F>,
}

/// This is the RoundOne Output
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoundOneOutput<P: Pairing, F: PrimeField> {
    pub a_commitment: P::G1,
    pub b_commitment: P::G1,
    pub c_commitment: P::G1,
    pub a_x: UnivariantPolynomial<F>,
    pub b_x: UnivariantPolynomial<F>,
    pub c_x: UnivariantPolynomial<F>,
}

/// This is the output for round two
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoundTwoOutput<P: Pairing, F: PrimeField> {
    pub accumulator_commitment: P::G1,
    pub accumulator_poly: UnivariantPolynomial<F>,
    pub beta: F,
    pub gamma: F,
}

/// This is the output of the round 3 round
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoundThreeOutput<P: Pairing, F: PrimeField> {
    pub t_lo_commitment: P::G1,
    pub t_mid_commitment: P::G1,
    pub t_hi_commitment: P::G1,
    pub w_accumulator_poly: UnivariantPolynomial<F>,
    pub t_lo_poly: UnivariantPolynomial<F>,
    pub t_mid_poly: UnivariantPolynomial<F>,
    pub t_hi_poly: UnivariantPolynomial<F>,
    pub alpha: F,
}

/// This is the output of the round 4 round
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoundFourOutput<F: PrimeField> {
    pub a_x_ploy_zeta: F,
    pub b_x_ploy_zeta: F,
    pub c_x_ploy_zeta: F,
    pub w_accumulator_poly_zeta: F,
    pub s1_poly_zeta: F,
    pub s2_poly_zeta: F,
    pub zeta: F,
}

/// This is the output of the round 5 round
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoundFiveOutput<P: Pairing, F: PrimeField> {
    pub W_zeta_poly_commitment: P::G1,
    pub W_zeta_w_poly_commitment: P::G1,
    pub mu: F,
}

/// This is a struct representing the interface of the plonk proof
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlonkProof<P: Pairing, F: PrimeField> {
    pub a_poly_commitment: P::G1,
    pub b_poly_commitment: P::G1,
    pub c_poly_commitment: P::G1,
    pub accumulator_poly_commitment: P::G1,
    pub t_lo_poly_commitment: P::G1,
    pub t_mid_poly_commitment: P::G1,
    pub t_hi_poly_commitment: P::G1,
    pub W_zeta_poly_commitment: P::G1,
    pub W_zeta_w_poly_commitment: P::G1,
    pub a_x_zeta: F,
    pub b_x_zeta: F,
    pub c_x_zeta: F,
    pub w_accumulator_poly_zeta: F,
    pub s1_poly_zeta: F,
    pub s2_poly_zeta: F,
}

impl<P: Pairing> PlonkSRS<P> {
    pub fn new(g1_power_of_taus: Vec<P::G1>, g2_power_of_tau: Vec<P::G2>) -> Self {
        Self {
            g1_power_of_taus,
            g2_power_of_tau,
        }
    }

    pub fn run_setup(&self, tau: P::ScalarField, poly_degree: usize) -> PlonkSRS<P> {
        let kzg_srs: SRS<P> = UnivariateKZG::generate_srs(&tau, poly_degree);
        PlonkSRS {
            g1_power_of_taus: kzg_srs.g1_power_of_taus,
            g2_power_of_tau: kzg_srs.g2_power_of_tau,
        }
    }
}

impl<P: Pairing, F: PrimeField> RoundOneOutput<P, F> {
    pub fn new(
        a_commitment: P::G1,
        b_commitment: P::G1,
        c_commitment: P::G1,
        a_x: UnivariantPolynomial<F>,
        b_x: UnivariantPolynomial<F>,
        c_x: UnivariantPolynomial<F>,
    ) -> Self {
        Self {
            a_commitment,
            b_commitment,
            c_commitment,
            a_x,
            b_x,
            c_x,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.a_commitment.to_string().as_bytes());
        bytes.extend_from_slice(&self.b_commitment.to_string().as_bytes());
        bytes.extend_from_slice(&self.c_commitment.to_string().as_bytes());
        bytes
    }
}

impl<P: Pairing, F: PrimeField> RoundTwoOutput<P, F> {
    pub fn new(
        accumulator_commitment: P::G1,
        beta: F,
        gamma: F,
        accumulator_poly: UnivariantPolynomial<F>,
    ) -> Self {
        Self {
            accumulator_commitment,
            accumulator_poly,
            beta,
            gamma,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.accumulator_commitment.to_string().as_bytes());
        bytes
    }
}

impl<P: Pairing, F: PrimeField> RoundThreeOutput<P, F> {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.t_lo_poly.to_string().as_bytes());
        bytes.extend_from_slice(&self.t_mid_poly.to_string().as_bytes());
        bytes.extend_from_slice(&self.t_hi_poly.to_string().as_bytes());
        bytes
    }
}

impl<F: PrimeField> RoundFourOutput<F> {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.a_x_ploy_zeta.to_string().as_bytes());
        bytes.extend_from_slice(&self.b_x_ploy_zeta.to_string().as_bytes());
        bytes.extend_from_slice(&self.c_x_ploy_zeta.to_string().as_bytes());
        bytes
    }
}

impl<P: Pairing, F: PrimeField> RoundFiveOutput<P, F> {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.W_zeta_poly_commitment.to_string().as_bytes());
        bytes.extend_from_slice(&self.W_zeta_w_poly_commitment.to_string().as_bytes());
        bytes
    }
}

impl<F: PrimeField> Witness<F> {
    pub fn new(
        a: UnivariateEval<F>,
        b: UnivariateEval<F>,
        c: UnivariateEval<F>,
        pi: UnivariateEval<F>,
    ) -> Self {
        Self { a, b, c, pi }
    }
}
