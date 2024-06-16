use ark_ff::PrimeField;
use crate::primitives::{QAPPolysCoefficients, ToxicWaste, TrustedSetupExcecution, QAP};

pub trait R1CSInterface<F: PrimeField> {
    /// This function take the columns from the R1CS matrix and returns the QAP polynomial coefficients
    fn to_qap_poly_coefficients(&self) -> QAPPolysCoefficients<F>;
}


pub trait QAPPolysCoefficientsInterface<F: PrimeField> {
    /// This fuction takes the QAP polynomial coefficients with the witness and returns the QAP polynomials
    fn to_qap_polynomials(&self, witness: Vec<F>) -> QAP<F>;
}


pub trait QAPInterface<F: PrimeField> {
    /// This is function is used to check if the QAP is satisfied
    fn is_satisfied(&self) -> bool;
}


pub trait TrustedSetupInterface<F: PrimeField> {
    /// This function is used to run the trusted setup
    fn run_trusted_setup(&self) -> TrustedSetupExcecution<F>;
}