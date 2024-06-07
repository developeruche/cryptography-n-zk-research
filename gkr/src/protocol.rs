use crate::{
    interfaces::GKRProtocolInterface,
    primitives::{Circuit, CircuitEvaluation, GKRProof},
};
use ark_ff::PrimeField;
use fiat_shamir::{interface::TranscriptInterface, FiatShamirTranscript};
use polynomial::{interface::MultilinearPolynomialInterface, multilinear::Multilinear};
pub struct GKRProtocol;

impl<F: PrimeField> GKRProtocolInterface<F> for GKRProtocol {
    fn prove(circuit: &Circuit, evals: &CircuitEvaluation<F>) -> GKRProof<F> {
        // let mut transcript = FiatShamirTranscript::new(vec![]);
        // // let mut sumcheck_proofs = vec![];
        // let mut q_polynomials = vec![];

        // let w_0_mle = Multilinear::new(evals.layers[0], evals.layers[0].len());
        // transcript.append(w_0_mle.to_bytes());

        // let n_r = transcript.sample_n_as_field_elements(w_0_mle.num_vars);
        // let claim = w_0_mle.evaluate(&n_r).unwrap();

        // // starting the GKR round reductions powered by sumcheck
        // for l_index in 1..evals.layers.len() {

        // }

        todo!()
    }

    fn verify(circuit: &Circuit, input: &[F], proof: &GKRProof<F>) -> bool {
        unimplemented!()
    }
}
