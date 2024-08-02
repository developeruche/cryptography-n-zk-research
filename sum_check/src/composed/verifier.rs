use super::ComposedSumCheckProof;
use crate::interface::ComposedVerifierInterface;
use ark_ff::{BigInteger, PrimeField};
use fiat_shamir::interface::TranscriptInterface;
use fiat_shamir::FiatShamirTranscript;
use polynomial::composed::multilinear::ComposedMultilinear;
use polynomial::interface::MultilinearPolynomialInterface;
use polynomial::interface::PolynomialInterface;

#[derive(Clone, Default, Debug)]
pub struct ComposedVerifier;

impl<F: PrimeField> ComposedVerifierInterface<F> for ComposedVerifier {
    fn verify(proof: &ComposedSumCheckProof<F>, poly: &ComposedMultilinear<F>) -> bool {
        // steps
        // 1. eval poly_0 at 0 and 1 and check if the sum is equal to the sum in the proof [done]
        // 2. append poly_0 to the transcript
        // 3. eval poly_1 at rand_0 from transcript and check if the eval of poly_0 at 0 and 1 is equal to poly_1 at rand_0
        // 4. append poly_1 to the transcript
        // 5. repeat step 3 and 4 until the last round
        // 6. check if the eval of the last poly at rand(last) is equal to the eval of the main poly at all rands()

        let mut transcript = FiatShamirTranscript::default();

        transcript.append(poly.to_bytes());
        transcript.append(proof.sum.into_bigint().to_bytes_be());

        let mut all_rands = Vec::new();

        let mut mutating_sum = proof.sum;

        for r_poly in proof.round_poly.iter() {
            transcript.append(r_poly.to_bytes());

            // stage one assertion (see if current mutating sum was influenced by the passed mutating sum)
            let untrusted_sum = r_poly.evaluate(&F::zero()) + r_poly.evaluate(&F::one());

            if untrusted_sum != mutating_sum {
                println!(
                    "untrusted_sum != proof.sum --> {} - {}",
                    untrusted_sum, proof.sum
                );
                return false;
            }

            let sample = F::from_be_bytes_mod_order(&transcript.sample());
            mutating_sum = r_poly.evaluate(&sample);
            all_rands.push(sample);
        }

        // last round check
        let last_mutating_sum = poly.evaluate(&all_rands).unwrap();

        if last_mutating_sum != mutating_sum {
            println!(
                "last_mutating_sum != mutating_sum --> {} - {}",
                last_mutating_sum, mutating_sum
            );
            return false;
        }

        true
    }
}
