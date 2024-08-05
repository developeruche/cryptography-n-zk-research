use crate::{interfaces::GKRProtocolInterface, primitives::GKRProof, utils::gen_w_mle};
use ark_ff::PrimeField;
use circuits::{
    interfaces::GKRProtocolCircuitInterface,
    primitives::{Circuit, CircuitEvaluation},
};
use fiat_shamir::{interface::TranscriptInterface, FiatShamirTranscript};
use polynomial::{
    composed::multilinear::ComposedMultilinear, interface::MultilinearPolynomialInterface,
};
use sum_check::{
    composed::multicomposed::MultiComposedProver, interface::MultiComposedProverInterface,
};

pub struct GKRProtocol;

impl<F: PrimeField> GKRProtocolInterface<F> for GKRProtocol {
    fn prove<P: MultilinearPolynomialInterface<F> + Clone>(
        circuit: &Circuit,
        evals: &CircuitEvaluation<F>,
    ) -> GKRProof<F, P> {
        let mut transcript = FiatShamirTranscript::new(vec![]);
        let mut sumcheck_proofs = vec![];
        // let mut q_polynomials = vec![];

        let w_0_mle = gen_w_mle(&evals.layers, 0);
        transcript.append(w_0_mle.to_bytes());

        let mut n_r = transcript.sample_n_as_field_elements(w_0_mle.num_vars);
        let claim = w_0_mle.evaluate(&n_r).unwrap();

        // starting the GKR round reductions powered by sumcheck
        for l_index in 1..evals.layers.len() {
            let n_r_internal = n_r.clone();
            let number_of_round = n_r_internal.len();

            let (add_mle, mul_mle) = circuit.get_add_n_mul_mle::<F>(l_index - 1);
            let w_i_mle = gen_w_mle(&evals.layers, l_index);
            // f(b, c) = add(r, b, c)(w_i(b) + w_i(c)) + mul(r, b, c)(w_i(b) * w_i(c))
            // add(r, b, c) ---> add(b, c)
            let add_b_c =
                add_mle.partial_evaluations(n_r_internal.clone(), vec![0; number_of_round]);
            // mul(r, b, c) ---> mul(b, c)
            let mul_b_c = mul_mle.partial_evaluations(n_r_internal, vec![0; number_of_round]);

            let wb = w_i_mle.clone();
            let wc = w_i_mle.clone();

            // w_i(b) + w_i(c)
            let wb_add_wc = wb.add_distinct(&wc);
            // w_i(b) * w_i(c)
            let wb_mul_wc = wb.mul_distinct(&wc);

            //  add(b, c)(w_i(b) + w_i(c))
            let f_b_c_add_section = ComposedMultilinear::new(vec![add_b_c, wb_add_wc]);
            // mul(b, c)(w_i(b) * w_i(c))
            let f_b_c_mul_section = ComposedMultilinear::new(vec![mul_b_c, wb_mul_wc]);

            // f(b, c) = add(r, b, c)(w_i(b) + w_i(c)) + mul(r, b, c)(w_i(b) * w_i(c))
            let f_b_c = vec![f_b_c_add_section, f_b_c_mul_section];

            // this prover that the `claim` is the result of the evalution of the preivous layer
            let (sumcheck_proof, random_challenges) =
                MultiComposedProver::sum_check_proof_without_initial_polynomial(
                    &f_b_c,
                    &mut transcript,
                    &claim,
                );

            transcript.append(sumcheck_proof.to_bytes());
            sumcheck_proofs.push(sumcheck_proof);
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
