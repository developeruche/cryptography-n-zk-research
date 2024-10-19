#![allow(non_snake_case)]

use ark_ec::pairing::Pairing;
use ark_ff::PrimeField;
use kzg_rust::{interface::KZGUnivariateInterface, primitives::SRS, univariate::UnivariateKZG};
use polynomial::evaluation::univariate::UnivariateEval;

pub struct PlonkSRS<P: Pairing> {
    pub g1_power_of_taus: Vec<P::G1>,
    // making this a vec also so batch kzg polynomial commitment can be used also
    pub g2_power_of_tau: Vec<P::G2>,
}


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
