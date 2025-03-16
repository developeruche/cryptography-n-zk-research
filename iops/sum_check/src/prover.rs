use crate::{data_structure::SumCheckProof, interface::ProverInterface};
use ark_ff::PrimeField;
use fiat_shamir::{interface::TranscriptInterface, FiatShamirTranscript};
use polynomial::{
    interface::MultilinearPolynomialInterface, multilinear::Multilinear, utils::boolean_hypercube,
};

#[derive(Clone, Default, Debug)]
pub struct Prover;

impl<F: PrimeField> ProverInterface<F> for Prover {
    /// This function returns the sum of the multilinear polynomial evaluation over the boolean hypercube.
    fn calculate_sum(poly: &Multilinear<F>) -> F {
        poly.evaluations.iter().sum()
    }

    /// This function returns the round zero computed polynomial
    fn compute_round_zero_poly<P: MultilinearPolynomialInterface<F>>(
        poly: &P,
        transcript: &mut FiatShamirTranscript,
    ) -> P {
        let number_of_round = poly.num_vars() - 1;
        let bh = boolean_hypercube(number_of_round);
        let mut bh_partials = P::zero(1); // this is an accumulator

        for bh_i in bh {
            let current_partial = poly.partial_evaluations(bh_i, vec![1; number_of_round]);
            bh_partials.internal_add_assign(&current_partial);
        }

        transcript.append(bh_partials.to_bytes());
        bh_partials
    }

    /// This function computes sum check proof
    fn sum_check_proof<P: MultilinearPolynomialInterface<F> + Clone>(
        poly: &P,
        transcript: &mut FiatShamirTranscript,
        sum: &F,
    ) -> SumCheckProof<F, P> {
        let round_0_poly = Self::compute_round_zero_poly(poly, transcript);
        let mut all_random_reponse = Vec::new();
        let mut round_poly = Vec::new();

        for i in 1..poly.num_vars() {
            let number_of_round = poly.num_vars() - i - 1;
            let bh = boolean_hypercube::<F>(number_of_round);
            let mut bh_partials = P::zero(1);

            let verifier_random_reponse_f = F::from_be_bytes_mod_order(&transcript.sample());
            all_random_reponse.push(verifier_random_reponse_f);

            for bh_i in bh {
                let bh_len = bh_i.len();
                let mut eval_vector = all_random_reponse.clone();
                eval_vector.extend(bh_i);
                let mut eval_index = vec![0; all_random_reponse.len()];
                let suffix_eval_index = vec![1; bh_len];
                eval_index.extend(suffix_eval_index);

                let current_partial = poly.partial_evaluations(eval_vector, eval_index);
                bh_partials.internal_add_assign(&current_partial);
            }

            transcript.append(bh_partials.to_bytes());
            round_poly.push(bh_partials);
        }

        SumCheckProof {
            polynomial: poly.clone(),
            round_poly: round_poly.clone(),
            round_0_poly: round_0_poly.clone(),
            sum: sum.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{interface::VerifierInterface, verifier::Verifier};

    use super::*;
    use ark_test_curves::bls12_381::Fr;

    #[test]
    fn test_sum_calculation() {
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

        let sum = Prover::calculate_sum(&poly);
        assert_eq!(sum, Fr::from(12));
    }

    #[test]
    fn test_compute_round_zero_poly() {
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
        let mut transcript = FiatShamirTranscript::default();
        let round_0_poly = Prover::compute_round_zero_poly(&poly, &mut transcript);
        assert_eq!(round_0_poly.evaluations, vec![Fr::from(2), Fr::from(10)]);
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
        let mut transcript = FiatShamirTranscript::default();
        let round_0_poly = Prover::compute_round_zero_poly(&poly, &mut transcript);
        let sum = round_0_poly.evaluate(&vec![Fr::from(1)]).unwrap()
            + round_0_poly.evaluate(&vec![Fr::from(0)]).unwrap();

        assert_eq!(sum, Fr::from(12));
    }

    #[test]
    fn test_compute_round_zero_poly_3() {
        let poly = Multilinear::new(
            vec![
                Fr::from(0),
                Fr::from(0),
                Fr::from(2),
                Fr::from(7),
                Fr::from(3),
                Fr::from(3),
                Fr::from(5),
                Fr::from(11),
            ],
            3,
        );

        let mut transcript = FiatShamirTranscript::default();
        let round_0_poly = Prover::compute_round_zero_poly(&poly, &mut transcript);

        assert_eq!(round_0_poly.evaluations, vec![Fr::from(9), Fr::from(22)]);
    }

    #[test]
    fn test_compute_round_zero_poly_4() {
        let poly = Multilinear::new(
            vec![
                Fr::from(0),
                Fr::from(0),
                Fr::from(2),
                Fr::from(7),
                Fr::from(3),
                Fr::from(3),
                Fr::from(5),
                Fr::from(11),
            ],
            3,
        );

        let mut transcript = FiatShamirTranscript::default();
        let round_0_poly = Prover::compute_round_zero_poly(&poly, &mut transcript);
        let sum = round_0_poly.evaluate(&vec![Fr::from(1)]).unwrap()
            + round_0_poly.evaluate(&vec![Fr::from(0)]).unwrap();

        assert_eq!(sum, Fr::from(31));
    }

    #[test]
    fn test_compute_round_zero_poly_5() {
        let poly = Multilinear::new(
            vec![
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(1),
                Fr::from(1),
                Fr::from(1),
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
            ],
            4,
        );

        let mut transcript = FiatShamirTranscript::default();
        let round_0_poly = Prover::compute_round_zero_poly(&poly, &mut transcript);

        let sum = round_0_poly.evaluate(&vec![Fr::from(1)]).unwrap()
            + round_0_poly.evaluate(&vec![Fr::from(0)]).unwrap();
        assert_eq!(sum, Fr::from(3));
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
        let mut transcript = FiatShamirTranscript::default();
        let sum = Prover::calculate_sum(&poly);
        let proof = Prover::sum_check_proof(&poly, &mut transcript, &sum);

        assert!(Verifier::verify(&proof));
    }

    #[test]
    fn test_sum_check_proof_2() {
        let poly = Multilinear::new(
            vec![
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(1),
                Fr::from(1),
                Fr::from(1),
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
            ],
            4,
        );
        let mut transcript = FiatShamirTranscript::default();
        let sum = Prover::calculate_sum(&poly);
        let proof = Prover::sum_check_proof(&poly, &mut transcript, &sum);

        assert!(Verifier::verify(&proof));
    }

    #[test]
    fn test_sum_check_proof_3() {
        let poly = Multilinear::new(
            vec![
                Fr::from(1),
                Fr::from(3),
                Fr::from(5),
                Fr::from(7),
                Fr::from(2),
                Fr::from(4),
                Fr::from(6),
                Fr::from(8),
                Fr::from(3),
                Fr::from(5),
                Fr::from(7),
                Fr::from(9),
                Fr::from(4),
                Fr::from(6),
                Fr::from(8),
                Fr::from(10),
            ],
            4,
        );
        let mut transcript = FiatShamirTranscript::default();
        let sum = Prover::calculate_sum(&poly);
        let proof = Prover::sum_check_proof(&poly, &mut transcript, &sum);

        assert!(Verifier::verify(&proof));
    }
}
