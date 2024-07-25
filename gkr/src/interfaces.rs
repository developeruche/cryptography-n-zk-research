use crate::primitives::GKRProof;
use ark_ff::PrimeField;
use circuits::primitives::{Circuit, CircuitEvaluation};
use polynomial::interface::MultilinearPolynomialInterface;

/// This is the interface for the GKR protocol
pub trait GKRProtocolInterface<F: PrimeField> {
    /// This function is used to create GKR proofs
    fn prove<P: MultilinearPolynomialInterface<F> + Clone>(
        circuit: &Circuit,
        evals: &CircuitEvaluation<F>,
    ) -> GKRProof<F, P>;

    /// This function is used to verify GKR proofs
    fn verify<P: MultilinearPolynomialInterface<F> + Clone>(
        circuit: &Circuit,
        input: &[F],
        proof: &GKRProof<F, P>,
    ) -> bool;
}
