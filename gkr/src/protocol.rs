use crate::{
    interfaces::GKRProtocolInterface,
    primitives::{Circuit, CircuitEvaluation, GKRProof},
};
use ark_ff::PrimeField;
use fiat_shamir::{interface::TranscriptInterface, FiatShamirTranscript};
use polynomial::multilinear::Multilinear;
pub struct GKRProtocol;

impl<F: PrimeField> GKRProtocolInterface<F> for GKRProtocol {
    fn prove(circuit: &Circuit, evals: &CircuitEvaluation<F>) -> GKRProof<F> {
        let mut transcript = FiatShamirTranscript::new(vec![]);
        let mut sumcheck_proofs = vec![];
        let mut q_polynomials = vec![];

        let w_0_mle = Multilinear::new(evals.layers[0], evals.layers[0].len());
        transcript.append(w_0_mle.to_bytes());
    }

    fn verify(circuit: &Circuit, input: &[F], proof: &GKRProof<F>) -> bool {
        unimplemented!()
    }
}
