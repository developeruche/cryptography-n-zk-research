use crate::primitives::{
    ProvingKey, QAPPolys, QAPPolysCoefficients, TrustedSetupExcecution, VerificationKey, QAP,
};
use ark_ff::PrimeField;

pub trait R1CSProcessingInterface<F: PrimeField> {
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
    /// parameters:
    /// circuit_details: The QAPPolys struct that contains the QAP polynomial coefficients.\
    /// this is used for the circuit specific trusted setup
    fn run_trusted_setup(&self, circuit_details: &QAPPolys<F>) -> TrustedSetupExcecution<F>;
    /// This function is used to run the trusted setup with toxic variables been know
    fn run_trusted_setup_toxic_variables(
        &self,
        circuit_details: &QAPPolys<F>,
    ) -> TrustedSetupExcecution<F>;
    /// This function is used to obtain verification key
    fn get_verification_key(&self) -> VerificationKey<F>;
    /// This function is used to obtain the proving key
    fn get_proving_key(&self) -> ProvingKey<F>;
}

pub trait PreProcessorInterface<F: PrimeField> {
    /// This function is used to preprocess the R1CS
    fn preprocess(&self) -> QAP<F>;
}