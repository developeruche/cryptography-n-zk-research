use crate::{data_structure::SumCheckProof, interface::VerifierInterface};
use ark_ff::PrimeField;
use fiat_shamir::{interface::TranscriptInterface, FiatShamirTranscript};
use polynomial::interface::MultilinearPolynomialInterface;

#[derive(Clone, Default, Debug)]
pub struct Verifier;

impl<F: PrimeField> VerifierInterface<F> for Verifier {
    /// This function verifies the sum check proof
    fn verify(proof: &SumCheckProof<F>) -> bool {
        // steps
        // 1. eval poly_0 at 0 and 1 and check if the sum is equal to the sum in the proof [done]
        // 2. append poly_0 to the transcript
        // 3. eval poly_1 at rand_0 from transcript and check if the eval of poly_0 at 0 and 1 is equal to poly_1 at rand_0
        // 4. append poly_1 to the transcript
        // 5. repeat step 3 and 4 until the last round
        // 6. check if the eval of the last poly at rand(last) is equal to the eval of the main poly at all rands()

        let mut transcript = FiatShamirTranscript::default();
        let mut all_rands = Vec::new();

        // step 1
        let untrusted_sum = proof.round_0_poly.evaluate(&vec![F::one()]).unwrap()
            + proof.round_0_poly.evaluate(&vec![F::zero()]).unwrap();

        if untrusted_sum != proof.sum {
            println!(
                "untrusted_sum != proof.sum --> {} - {}",
                untrusted_sum, proof.sum
            );
            return false;
        }

        // step 2
        transcript.append(proof.round_0_poly.to_bytes());

        // step 3 and 4
        let sample_1 = F::from_be_bytes_mod_order(&transcript.sample());
        all_rands.push(sample_1);
        let eval_poly_0_at_rand = proof.round_0_poly.evaluate(&vec![sample_1]).unwrap();
        let eval_poly_1_at_0_plus_1 = proof.round_poly[0].evaluate(&vec![F::one()]).unwrap()
            + proof.round_poly[0].evaluate(&vec![F::zero()]).unwrap();

        if eval_poly_0_at_rand != eval_poly_1_at_0_plus_1 {
            println!(
                "eval_poly_0_at_rand != eval_poly_1_at_0_plus_1 --> {} - {}",
                eval_poly_0_at_rand, eval_poly_1_at_0_plus_1
            );
            return false;
        }

        transcript.append(proof.round_poly[0].to_bytes());

        // step 5
        for i in 1..proof.round_poly.len() {
            let sample_i = F::from_be_bytes_mod_order(&transcript.sample());
            all_rands.push(sample_i);
            let eval_poly_i_at_rand = proof.round_poly[i - 1].evaluate(&vec![sample_i]).unwrap();
            let eval_poly_i_plus_1_at_0_plus_1 =
                proof.round_poly[i].evaluate(&vec![F::one()]).unwrap()
                    + proof.round_poly[i].evaluate(&vec![F::zero()]).unwrap();

            if eval_poly_i_at_rand != eval_poly_i_plus_1_at_0_plus_1 {
                println!(
                    "eval_poly_i_at_rand != eval_poly_i_plus_1_at_0_plus_1 --> {} - {}",
                    eval_poly_i_at_rand, eval_poly_i_plus_1_at_0_plus_1
                );
                return false;
            }

            transcript.append(proof.round_poly[i].to_bytes());
        }

        // step 6
        let last_round_rand = F::from_be_bytes_mod_order(&transcript.sample());
        all_rands.push(last_round_rand);
        let eval_last_poly_at_rand = proof.round_poly[proof.round_poly.len() - 1]
            .evaluate(&vec![all_rands[all_rands.len() - 1]])
            .unwrap();
        let eval_main_poly_at_all_rands = proof.polynomial.evaluate(&all_rands).unwrap();

        if eval_last_poly_at_rand != eval_main_poly_at_all_rands {
            println!(
                "eval_last_poly_at_rand != eval_main_poly_at_all_rands --> {} - {}",
                eval_last_poly_at_rand, eval_main_poly_at_all_rands
            );
            return false;
        }

        true
    }
}
