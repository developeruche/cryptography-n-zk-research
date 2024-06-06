use ark_ff::PrimeField;
use crate::{interfaces::GKRProtocolInterface, primitives::{Circuit, CircuitEvaluation, GKRProof}};
pub struct GKRProtocol;




impl<F: PrimeField> GKRProtocolInterface<F> for GKRProtocol {
    fn prove(
        circuit: &Circuit,
        evals: &CircuitEvaluation<F>,
    ) -> GKRProof<F> {
        unimplemented!()
    }
    
    fn verify(
        circuit: &Circuit,
        input: &[F],
        proof: &GKRProof<F>,
    ) -> bool {
        unimplemented!()
    }
}