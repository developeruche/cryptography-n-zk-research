#![allow(unused_assignments)]
pub mod interfaces;
pub mod primitives;

use crate::{
    interfaces::SuccinctGKRProtocolInterface,
    primitives::{SuccinctGKRMultilinearKZGOPenningProof, SuccinctGKRProof},
};
use ark_ec::pairing::Pairing;
use ark_ff::PrimeField;
use circuits::{
    interfaces::GKRProtocolCircuitInterface,
    primitives::{Circuit, CircuitEvaluation},
};
use fiat_shamir::{FiatShamirTranscript, interface::TranscriptInterface};
use gkr::utils::{gen_w_mle, perform_gkr_sumcheck_layer_one, verifiy_gkr_sumcheck_layer_one};
use pcs::{
    interface::KZGMultiLinearInterface, kzg::multilinear::MultilinearKZG,
    primitives::MultiLinearSRS,
};
use polynomial::{
    composed::multilinear::ComposedMultilinear, interface::MultilinearPolynomialInterface,
};
use sum_check::{
    composed::multicomposed::{MultiComposedProver, MultiComposedVerifier},
    interface::{MultiComposedProverInterface, MultiComposedVerifierInterface},
};

pub struct SuccinctGKRProtocol;

impl<F: PrimeField, P: Pairing> SuccinctGKRProtocolInterface<F, P> for SuccinctGKRProtocol {
    fn prove(
        circuit: &Circuit,
        evals: &CircuitEvaluation<F>,
        input_poly_commitment: &P::G1,
        srs: &MultiLinearSRS<P>,
    ) -> SuccinctGKRProof<F, P> {
        let mut transcript = FiatShamirTranscript::new(vec![]);

        let mut sum_check_proofs = vec![];
        let mut w_i_b = vec![];
        let mut w_i_c = vec![];

        let mut proof_b: SuccinctGKRMultilinearKZGOPenningProof<P, F> =
            SuccinctGKRMultilinearKZGOPenningProof::<P, F>::default();
        let mut proof_c: SuccinctGKRMultilinearKZGOPenningProof<P, F> =
            SuccinctGKRMultilinearKZGOPenningProof::<P, F>::default();

        let w_0_mle = gen_w_mle(&evals.layers, 0);
        transcript.append(w_0_mle.to_bytes());
        transcript.append(input_poly_commitment.to_string().as_bytes().to_vec()); // Appending the commitment to the poly to ensure soundness

        let n_r = transcript.sample_n_as_field_elements(w_0_mle.num_vars);
        let mut claim = w_0_mle.evaluate(&n_r).unwrap();

        let mut last_rand_b;
        let mut last_rand_c;
        let mut last_alpha;
        let mut last_beta;

        // let mut last_round_w_b_opening_proof

        // Running sumcheck on layer one
        let (add_mle_layer_one, mul_mle_layer_one) = circuit.get_add_n_mul_mle::<F>(0);
        let w_1_mle = gen_w_mle(&evals.layers, 1);
        let (layer_one_claim, layer_one_rand_b, layer_one_rand_c, layer_one_alpha, layer_one_beta) =
            perform_gkr_sumcheck_layer_one(
                claim,
                n_r.clone(),
                &add_mle_layer_one,
                &mul_mle_layer_one,
                &w_1_mle,
                &mut transcript,
                &mut sum_check_proofs,
                &mut w_i_b,
                &mut w_i_c,
            );

        claim = layer_one_claim;
        last_rand_b = layer_one_rand_b;
        last_rand_c = layer_one_rand_c;
        last_alpha = layer_one_alpha;
        last_beta = layer_one_beta;

        // starting the GKR round reductions powered by sumcheck (layer 2 to n-1(excluding the input layer))
        for l_index in 2..evals.layers.len() {
            let (add_mle, mul_mle) = circuit.get_add_n_mul_mle::<F>(l_index - 1);
            let w_i_mle = gen_w_mle(&evals.layers, l_index);

            let number_of_round = last_rand_b.len();

            // add(r_b, b, c) ---> add(b, c)
            let add_rb_b_c =
                add_mle.partial_evaluations(last_rand_b.clone(), vec![0; number_of_round]);
            // mul(r_b, b, c) ---> mul(b, c)
            let mul_rb_b_c =
                mul_mle.partial_evaluations(last_rand_b.clone(), vec![0; number_of_round]);

            // add(r_c, b, c) ---> add(b, c)
            let add_rc_b_c =
                add_mle.partial_evaluations(last_rand_c.clone(), vec![0; number_of_round]);
            // mul(r_c, b, c) ---> mul(b, c)
            let mul_rc_b_c =
                mul_mle.partial_evaluations(last_rand_c.clone(), vec![0; number_of_round]);

            // alpha * add(r_b, b, c) + beta * add(r_c, b, c)
            let alpha_beta_add_b_c = (add_rb_b_c * last_alpha) + (add_rc_b_c * last_beta);
            // alpha * mul(r_b, b, c) + beta * mul(r_c, b, c)
            let alpha_beta_mul_b_c = (mul_rb_b_c * last_alpha) + (mul_rc_b_c * last_beta);

            let wb = w_i_mle.clone();
            let wc = w_i_mle.clone();

            // w_i(b) + w_i(c)
            let wb_add_wc = wb.add_distinct(&wc);
            // w_i(b) * w_i(c)
            let wb_mul_wc = wb.mul_distinct(&wc);

            // alpha * add(r_b, b, c) + beta * add(r_c, b, c)(w_i(b) + w_i(c))
            let f_b_c_add_section = ComposedMultilinear::new(vec![alpha_beta_add_b_c, wb_add_wc]);
            // alpha * mul(r_b, b, c) + beta * mul(r_c, b, c)(w_i(b) * w_i(c))
            let f_b_c_mul_section = ComposedMultilinear::new(vec![alpha_beta_mul_b_c, wb_mul_wc]);

            // f(b, c) = alpha * add(r_b, b, c) + beta * add(r_c, b, c)(w_i(b) + w_i(c)) + alpha * mul(r_b, b, c) + beta * mul(r_c, b, c)(w_i(b) * w_i(c))
            let f_b_c = vec![f_b_c_add_section, f_b_c_mul_section];

            // this prover that the `claim` is the result of the evalution of the preivous layer
            let (sumcheck_proof, random_challenges) =
                MultiComposedProver::sum_check_proof_without_initial_polynomial(&f_b_c, &claim);

            transcript.append(sumcheck_proof.to_bytes());
            sum_check_proofs.push(sumcheck_proof);

            let (rand_b, rand_c) = random_challenges.split_at(random_challenges.len() / 2);

            let eval_w_i_b = wb.evaluate(&rand_b.to_vec()).unwrap();
            let eval_w_i_c = wc.evaluate(&rand_c.to_vec()).unwrap();

            // TODO: always make this push even if it is the last round
            w_i_b.push(eval_w_i_b);
            w_i_c.push(eval_w_i_c);

            last_alpha = transcript.sample_as_field_element();
            last_beta = transcript.sample_as_field_element();

            last_rand_b = rand_b.to_vec();
            last_rand_c = rand_c.to_vec();

            // check if this is the last round
            if l_index == evals.layers.len() - 1 {
                // opening poly and create MLE KGZ openning proof
                let (point_evaluation_w_last_b, proof_w_last_b) =
                    MultilinearKZG::open(&srs, &w_i_mle, &last_rand_b);
                let (point_evaluation_w_last_c, proof_w_last_c) =
                    MultilinearKZG::open(&srs, &w_i_mle, &last_rand_c);

                proof_b = SuccinctGKRMultilinearKZGOPenningProof {
                    opening: point_evaluation_w_last_b,
                    opening_proof: proof_w_last_b,
                };

                proof_c = SuccinctGKRMultilinearKZGOPenningProof {
                    opening: point_evaluation_w_last_c,
                    opening_proof: proof_w_last_c,
                };
            }

            claim = last_alpha * eval_w_i_b + last_beta * eval_w_i_c;
        }

        SuccinctGKRProof {
            sum_check_proofs,
            w_i_b,
            w_i_c,
            w_0_mle,
            w_i_b_last_proof: proof_b,
            w_i_c_last_proof: proof_c,
        }
    }

    fn verify(
        circuit: &Circuit,
        proof: &SuccinctGKRProof<F, P>,
        input_poly_commitment: &P::G1,
        srs: &MultiLinearSRS<P>,
    ) -> bool {
        // performing some sanity checks
        if proof.sum_check_proofs.len() != proof.w_i_b.len()
            || proof.sum_check_proofs.len() != proof.w_i_c.len()
        {
            println!("Invalid GKR proof");
            return false;
        }

        let mut transcript = FiatShamirTranscript::default();
        transcript.append(proof.w_0_mle.to_bytes());
        transcript.append(input_poly_commitment.to_string().as_bytes().to_vec()); // Appending the commitment to the poly to ensure soundness

        let n_r = transcript.sample_n_as_field_elements(proof.w_0_mle.num_vars);
        let mut claim = proof.w_0_mle.evaluate(&n_r).unwrap();

        let mut last_rand_b = vec![];
        let mut last_rand_c = vec![];

        let mut last_alpha = F::ZERO;
        let mut last_beta = F::ZERO;

        // layer one verification logic
        let (add_mle, mul_mle) = circuit.get_add_n_mul_mle::<F>(0);
        let (layer_one_verification_status, layer_one_sum) = verifiy_gkr_sumcheck_layer_one(
            &claim,
            &proof.sum_check_proofs[0],
            &mut transcript,
            proof.w_i_b[0],
            proof.w_i_c[0],
            n_r.clone(),
            &add_mle,
            &mul_mle,
        );

        if !layer_one_verification_status {
            return false;
        }

        claim = layer_one_sum;

        // running GKR verification logic excluding the first layer
        for i in 1..proof.sum_check_proofs.len() {
            if proof.sum_check_proofs[i].sum != claim {
                println!("Invalid sumcheck proof");
                return false;
            }

            transcript.append(proof.sum_check_proofs[i].to_bytes());
            let intermidate_claim_check =
                MultiComposedVerifier::verify_except_last_check(&proof.sum_check_proofs[i]);

            // performing sum check last check
            let (rand_b, rand_c) = intermidate_claim_check
                .random_challenges
                .split_at(intermidate_claim_check.random_challenges.len() / 2);

            last_rand_b = rand_b.to_vec();
            last_rand_c = rand_c.to_vec();

            let w_b = proof.w_i_b[i];
            let w_c = proof.w_i_c[i];

            let alpha: F = transcript.sample_as_field_element();
            let beta: F = transcript.sample_as_field_element();

            claim = alpha * w_b + beta * w_c;

            last_alpha = alpha;
            last_beta = beta;
        }

        // performing verification for the input layer
        let w_in_b_opening_verification = MultilinearKZG::verify(
            &srs,
            &input_poly_commitment,
            &last_rand_b,
            &proof.w_i_b_last_proof.opening,
            &proof.w_i_b_last_proof.opening_proof,
        );
        let w_in_c_opening_verification = MultilinearKZG::verify(
            &srs,
            &input_poly_commitment,
            &last_rand_c,
            &proof.w_i_c_last_proof.opening,
            &proof.w_i_c_last_proof.opening_proof,
        );

        if !w_in_b_opening_verification || !w_in_c_opening_verification {
            println!(
                "Invalid sumcheck proof (w_in_b_opening_verification || w_in_c_opening_verification)"
            );
            return false;
        }

        let expected_claim = last_alpha * proof.w_i_b_last_proof.opening
            + last_beta * proof.w_i_c_last_proof.opening;

        if expected_claim != claim {
            println!("Invalid sumcheck proof (expected_claim != claim)");
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_test_curves::bls12_381::{Bls12_381, Fr};
    use circuits::{
        interfaces::CircuitInterface,
        primitives::{CircuitLayer, Gate, GateType},
    };
    use gkr::utils::gen_random_taus;
    use polynomial::multilinear::Multilinear;

    #[test]
    fn test_succinct_gkr_protocol() {
        let layer_0 = CircuitLayer::new(vec![Gate::new(GateType::Add, [0, 1])]);
        let layer_1 = CircuitLayer::new(vec![
            Gate::new(GateType::Mul, [0, 1]),
            Gate::new(GateType::Add, [2, 3]),
        ]);
        let layer_3 = CircuitLayer::new(vec![
            Gate::new(GateType::Add, [0, 1]),
            Gate::new(GateType::Mul, [2, 3]),
            Gate::new(GateType::Mul, [4, 5]),
            Gate::new(GateType::Mul, [6, 7]),
        ]);
        let layer_4 = CircuitLayer::new(vec![
            Gate::new(GateType::Mul, [0, 1]),
            Gate::new(GateType::Mul, [2, 3]),
            Gate::new(GateType::Mul, [4, 5]),
            Gate::new(GateType::Add, [6, 7]),
            Gate::new(GateType::Mul, [8, 9]),
            Gate::new(GateType::Add, [10, 11]),
            Gate::new(GateType::Mul, [12, 13]),
            Gate::new(GateType::Mul, [14, 15]),
        ]);

        let circuit = Circuit::new(vec![layer_0, layer_1, layer_3, layer_4]);
        let input = [
            Fr::from(2u32),
            Fr::from(1u32),
            Fr::from(3u32),
            Fr::from(1u32),
            Fr::from(4u32),
            Fr::from(1u32),
            Fr::from(2u32),
            Fr::from(2u32),
            Fr::from(3u32),
            Fr::from(3u32),
            Fr::from(4u32),
            Fr::from(4u32),
            Fr::from(2u32),
            Fr::from(3u32),
            Fr::from(3u32),
            Fr::from(4u32),
        ];
        let input_in_poly_form = Multilinear::interpolate(&input);
        let tau_s = gen_random_taus::<Fr>(input_in_poly_form.num_vars);
        let srs: MultiLinearSRS<Bls12_381> = MultilinearKZG::generate_srs(&tau_s);
        let commitment = MultilinearKZG::commit(&srs, &input_in_poly_form);

        let evaluation = circuit.evaluate(&input);

        assert_eq!(evaluation.layers[0][0], Fr::from(224u32));

        let proof = SuccinctGKRProtocol::prove(&circuit, &evaluation, &commitment, &srs);

        assert!(SuccinctGKRProtocol::verify(
            &circuit,
            &proof,
            &commitment,
            &srs
        ));
    }

    #[test]
    #[ignore]
    fn test_succinct_gkr_protocol_random_circuit() {
        let circuit = Circuit::random(8);
        let input = (0u64..256)
            .into_iter()
            .map(|x| Fr::from(x))
            .collect::<Vec<Fr>>();

        let input_in_poly_form = Multilinear::interpolate(&input);
        let tau_s = gen_random_taus::<Fr>(input_in_poly_form.num_vars);
        let srs: MultiLinearSRS<Bls12_381> = MultilinearKZG::generate_srs(&tau_s);
        let commitment = MultilinearKZG::commit(&srs, &input_in_poly_form);

        let evaluation = circuit.evaluate(&input);

        let proof = SuccinctGKRProtocol::prove(&circuit, &evaluation, &commitment, &srs);

        assert!(SuccinctGKRProtocol::verify(
            &circuit,
            &proof,
            &commitment,
            &srs
        ));
    }
}
