use super::{ComposedSumCheckProof, RoundPoly};
use crate::interface::ComposedProverInterface;
use ark_ff::{BigInteger, PrimeField};
use fiat_shamir::{interface::TranscriptInterface, FiatShamirTranscript};
use polynomial::{
    composed::{interfaces::ComposedMultilinearInterface, multilinear::ComposedMultilinear},
    interface::MultilinearPolynomialInterface,
    univariant::UnivariantPolynomial,
};

#[derive(Clone, Default, Debug)]
pub struct ComposedProver;

impl<F: PrimeField> ComposedProverInterface<F> for ComposedProver {
    /// This function returns the sum of the multilinear polynomial evaluation over the boolean hypercube.
    fn calculate_sum(poly: &ComposedMultilinear<F>) -> F {
        poly.elementwise_product().iter().sum()
    }

    /// This function returns the round zero computed polynomial
    fn compute_round_zero_poly(
        poly: &ComposedMultilinear<F>,
        transcript: &mut FiatShamirTranscript,
    ) -> UnivariantPolynomial<F> {
        let composed_poly_max_degree = poly.max_degree();
        let mut round_0_poly_vec = Vec::new();

        for i in 0..=composed_poly_max_degree {
            let instance = poly
                .partial_evaluation(F::from(i as u128), 0)
                .elementwise_product()
                .iter()
                .sum();
            round_0_poly_vec.push(instance);
        }

        let round_0_poly = RoundPoly::new(round_0_poly_vec).interpolate();

        transcript.append(round_0_poly.to_bytes());

        round_0_poly
    }

    fn sum_check_proof(
        poly_: &ComposedMultilinear<F>,
        transcript: &mut FiatShamirTranscript,
        sum: &F,
    ) -> (ComposedSumCheckProof<F>, Vec<F>) {
        let mut poly = poly_.clone();
        let mut all_random_reponse = Vec::new();
        let mut round_polys = Vec::new();

        transcript.append(poly.to_bytes());
        transcript.append(sum.into_bigint().to_bytes_be());

        for _ in 0..poly.num_vars() {
            let mut round_poly_vec = Vec::new();

            for i in 0..=poly.max_degree() {
                let instance = poly
                    .partial_evaluation(F::from(i as u128), 0)
                    .elementwise_product()
                    .iter()
                    .sum();

                round_poly_vec.push(instance);
            }

            let round_poly = RoundPoly::new(round_poly_vec).interpolate();
            transcript.append(round_poly.to_bytes());

            let random_response = F::from_be_bytes_mod_order(&transcript.sample());
            poly = poly.partial_evaluation(random_response, 0);

            all_random_reponse.push(random_response);
            round_polys.push(round_poly);
        }

        (
            ComposedSumCheckProof {
                round_poly: round_polys,
                sum: *sum,
            },
            all_random_reponse, // this comes in handle in the GKR protocol
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composed::verifier::ComposedVerifier;
    use crate::interface::ComposedVerifierInterface;
    use ark_test_curves::bls12_381::Fr;
    use polynomial::interface::PolynomialInterface;
    use polynomial::multilinear::Multilinear;

    #[test]
    fn test_calculate_sum() {
        let poly1 = Multilinear::new(vec![Fr::from(0), Fr::from(1), Fr::from(2), Fr::from(3)], 2);
        let poly2 = Multilinear::new(vec![Fr::from(0), Fr::from(0), Fr::from(0), Fr::from(1)], 2);

        let composed = ComposedMultilinear::new(vec![poly1, poly2]);

        let sum = ComposedProver::calculate_sum(&composed);
        assert_eq!(sum, Fr::from(3));
    }

    #[test]
    fn test_calculate_sum_2() {
        let poly1 = Multilinear::new(vec![Fr::from(0), Fr::from(1), Fr::from(2), Fr::from(3)], 2);

        let composed = ComposedMultilinear::new(vec![poly1]);

        let sum = ComposedProver::calculate_sum(&composed);
        assert_eq!(sum, Fr::from(6));
    }

    #[test]
    fn test_calculate_sum_3() {
        let poly = Multilinear::new(
            vec![
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(2),
                Fr::from(2),
                Fr::from(2),
                Fr::from(2),
                Fr::from(4),
            ],
            3,
        );

        let composed = ComposedMultilinear::new(vec![poly]);

        let sum = ComposedProver::calculate_sum(&composed);
        assert_eq!(sum, Fr::from(12));
    }

    #[test]
    fn test_compute_round_zero_poly() {
        let poly1 = Multilinear::new(vec![Fr::from(0), Fr::from(1), Fr::from(2), Fr::from(3)], 2);

        let composed = ComposedMultilinear::new(vec![poly1]);

        let sum = ComposedProver::calculate_sum(&composed);

        let mut transcript = FiatShamirTranscript::default();
        let round_0_poly = ComposedProver::compute_round_zero_poly(&composed, &mut transcript);

        let sum_half_0 = round_0_poly.evaluate(&Fr::from(0));
        let sum_half_1 = round_0_poly.evaluate(&Fr::from(1));

        assert_eq!(sum, sum_half_0 + sum_half_1);
    }

    #[test]
    fn test_compute_round_zero_poly_2() {
        let poly = Multilinear::new(
            vec![
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(2),
                Fr::from(2),
                Fr::from(2),
                Fr::from(2),
                Fr::from(4),
            ],
            3,
        );

        let composed = ComposedMultilinear::new(vec![poly]);

        let sum = ComposedProver::calculate_sum(&composed);

        let mut transcript = FiatShamirTranscript::default();
        let round_0_poly = ComposedProver::compute_round_zero_poly(&composed, &mut transcript);

        let sum_half_0 = round_0_poly.evaluate(&Fr::from(0));
        let sum_half_1 = round_0_poly.evaluate(&Fr::from(1));

        assert_eq!(sum, sum_half_0 + sum_half_1);
    }

    #[test]
    fn test_compute_round_zero_poly_3() {
        let poly = Multilinear::new(
            vec![
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(2),
                Fr::from(2),
                Fr::from(2),
                Fr::from(2),
                Fr::from(4),
            ],
            3,
        );

        let composed = ComposedMultilinear::new(vec![poly]);

        let sum = ComposedProver::calculate_sum(&composed);

        let mut transcript = FiatShamirTranscript::default();
        let round_0_poly = ComposedProver::compute_round_zero_poly(&composed, &mut transcript);

        let sum_half_0 = round_0_poly.evaluate(&Fr::from(0));
        let sum_half_1 = round_0_poly.evaluate(&Fr::from(1));

        println!("round_0_poly_vec: {:?}", round_0_poly);

        assert_eq!(sum, sum_half_0 + sum_half_1);
    }

    #[test]
    fn test_sum_check_proof() {
        let poly = Multilinear::new(
            vec![
                Fr::from(0),
                Fr::from(0),
                Fr::from(2),
                Fr::from(7),
                Fr::from(3),
                Fr::from(3),
                Fr::from(6),
                Fr::from(11),
            ],
            3,
        );
        let composed = ComposedMultilinear::new(vec![poly]);
        let sum = ComposedProver::calculate_sum(&composed);

        let mut transcript = FiatShamirTranscript::default();
        let (proof, _) = ComposedProver::sum_check_proof(&composed, &mut transcript, &sum);

        let mut transcript_ = FiatShamirTranscript::default();
        assert!(ComposedVerifier::verify(
            &proof,
            &composed,
            &mut transcript_
        ));
    }

    #[test]
    fn test_sum_check_proof_2() {
        let poly1 = Multilinear::new(vec![Fr::from(0), Fr::from(1), Fr::from(2), Fr::from(3)], 2);
        let poly2 = Multilinear::new(vec![Fr::from(0), Fr::from(0), Fr::from(0), Fr::from(1)], 2);

        let composed = ComposedMultilinear::new(vec![poly1, poly2]);
        let sum = ComposedProver::calculate_sum(&composed);

        let mut transcript = FiatShamirTranscript::default();
        let (proof, _) = ComposedProver::sum_check_proof(&composed, &mut transcript, &sum);
        let mut transcript_ = FiatShamirTranscript::default();

        assert!(ComposedVerifier::verify(
            &proof,
            &composed,
            &mut transcript_
        ));
    }

    #[test]
    fn test_sum_check_proof_3() {
        let poly1 = Multilinear::new(vec![Fr::from(0), Fr::from(1), Fr::from(2), Fr::from(3)], 2);
        let poly2 = Multilinear::new(vec![Fr::from(0), Fr::from(0), Fr::from(0), Fr::from(1)], 2);
        let poly3 = Multilinear::new(vec![Fr::from(0), Fr::from(2), Fr::from(0), Fr::from(3)], 2);

        let composed = ComposedMultilinear::new(vec![poly1.clone(), poly2, poly3, poly1]);
        let sum = ComposedProver::calculate_sum(&composed);

        let mut transcript = FiatShamirTranscript::default();
        let (proof, _) = ComposedProver::sum_check_proof(&composed, &mut transcript, &sum);
        let mut transcript_ = FiatShamirTranscript::default();

        assert!(ComposedVerifier::verify(
            &proof,
            &composed,
            &mut transcript_
        ));
    }
}
