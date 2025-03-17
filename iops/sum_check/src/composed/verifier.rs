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
    fn verify(
        proof: &ComposedSumCheckProof<F>,
        poly: &ComposedMultilinear<F>,
        transcript: &mut FiatShamirTranscript,
    ) -> bool {
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
