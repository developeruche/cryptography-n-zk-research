use crate::{interfaces::GKRProtocolInterface, primitives::GKRProof, utils::gen_w_mle};
use ark_ff::PrimeField;
use circuits::{
    interfaces::GKRProtocolCircuitInterface,
    primitives::{Circuit, CircuitEvaluation},
};
use fiat_shamir::{interface::TranscriptInterface, FiatShamirTranscript};
use polynomial::interface::MultilinearPolynomialInterface;

pub struct GKRProtocol;

impl<F: PrimeField> GKRProtocolInterface<F> for GKRProtocol {
    fn prove<P: MultilinearPolynomialInterface<F> + Clone>(
        circuit: &Circuit,
        evals: &CircuitEvaluation<F>,
    ) -> GKRProof<F, P> {
        let mut transcript = FiatShamirTranscript::new(vec![]);
        // let mut sumcheck_proofs = vec![];
        // let mut q_polynomials = vec![];

        let w_0_mle = gen_w_mle(&evals.layers, 0);
        transcript.append(w_0_mle.to_bytes());

        let n_r = transcript.sample_n_as_field_elements(w_0_mle.num_vars);
        let claim = w_0_mle.evaluate(&n_r).unwrap();

        // starting the GKR round reductions powered by sumcheck
        for l_index in 1..evals.layers.len() {
            let (add_mle, mul_mle) = circuit.get_add_n_mul_mle::<F>(l_index - 1);
            let w_i_mle = gen_w_mle(&evals.layers, l_index);
        }

        todo!()
    }

    fn verify<P: MultilinearPolynomialInterface<F> + Clone>(
        circuit: &Circuit,
        input: &[F],
        proof: &GKRProof<F, P>,
    ) -> bool {
        unimplemented!()
    }
}
