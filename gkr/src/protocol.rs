use crate::{interfaces::GKRProtocolInterface, primitives::GKRProof};
use ark_ff::PrimeField;
use circuits::{
    interfaces::GKRProtocolCircuitInterface,
    primitives::{Circuit, CircuitEvaluation},
};
use fiat_shamir::{interface::TranscriptInterface, FiatShamirTranscript};
use polynomial::{interface::MultilinearPolynomialInterface, multilinear::Multilinear};

pub struct GKRProtocol;

impl<F: PrimeField> GKRProtocolInterface<F> for GKRProtocol {
    fn prove(circuit: &Circuit, evals: &CircuitEvaluation<F>) -> GKRProof<F> {
        let mut transcript = FiatShamirTranscript::new(vec![]);
        // let mut sumcheck_proofs = vec![];
        // let mut q_polynomials = vec![];

        let w_0_mle = Multilinear::new(evals.layers[0].clone(), evals.layers[0].len());
        transcript.append(w_0_mle.to_bytes());

        let n_r = transcript.sample_n_as_field_elements(w_0_mle.num_vars);
        let claim = w_0_mle.evaluate(&n_r).unwrap();

        // starting the GKR round reductions powered by sumcheck
        for l_index in 1..evals.layers.len() {
            let (add_mle, mul_mle) = circuit.get_add_n_mul_mle::<F>(l_index - 1);
            let log_2_num_vars = (evals.layers[l_index].len() as f64).log2().ceil() as usize;
            let w_i_mle = Multilinear::new(evals.layers[l_index].clone(), log_2_num_vars);
        }

        todo!()
    }

    fn verify(circuit: &Circuit, input: &[F], proof: &GKRProof<F>) -> bool {
        unimplemented!()
    }
}
