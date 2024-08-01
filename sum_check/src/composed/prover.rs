use crate::{data_structure::SumCheckProof, interface::ComposedProverInterface};
use ark_ff::PrimeField;
use fiat_shamir::{interface::TranscriptInterface, FiatShamirTranscript};
use polynomial::{
    composed::{interfaces::ComposedMultilinearInterface, multilinear::ComposedMultilinear},
    interface::{MultilinearPolynomialInterface, UnivariantPolynomialInterface},
    multilinear::Multilinear,
    utils::boolean_hypercube,
};

use super::RoundPoly;

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
    ) -> Vec<F> {
        let composed_poly_max_degree = poly.max_degree();
        let mut round_0_poly = Vec::new();

        for i in 0..=composed_poly_max_degree {
            let instance = poly
                .partial_evaluation(F::from(i as u128), 0)
                .elementwise_product()
                .iter()
                .sum();
            round_0_poly.push(instance);
        }

        round_0_poly
    }

    // /// This function computes sum check proof
    // fn sum_check_proof<P: MultilinearPolynomialInterface<F> + Clone>(
    //     poly: &P,
    //     transcript: &mut FiatShamirTranscript,
    //     sum: &F,
    // ) -> SumCheckProof<F, P> {
    //     let round_0_poly = Self::compute_round_zero_poly(poly, transcript);
    //     let mut all_random_reponse = Vec::new();
    //     let mut round_poly = Vec::new();

    //     for i in 1..poly.num_vars() {
    //         let number_of_round = poly.num_vars() - i - 1;
    //         let bh = boolean_hypercube::<F>(number_of_round);

    //         let mut bh_partials = P::zero(1);
    //         let verifier_random_reponse_f = F::from_be_bytes_mod_order(&transcript.sample());
    //         all_random_reponse.push(verifier_random_reponse_f);

    //         for bh_i in bh {
    //             let bh_len = bh_i.len();
    //             let mut eval_vector = all_random_reponse.clone();
    //             eval_vector.extend(bh_i);
    //             let mut eval_index = vec![0; all_random_reponse.len()];
    //             let suffix_eval_index = vec![1; bh_len];
    //             eval_index.extend(suffix_eval_index);

    //             let current_partial = poly.partial_evaluations(eval_vector, eval_index);

    //             bh_partials.internal_add_assign(&current_partial);
    //         }

    //         transcript.append(bh_partials.to_bytes());
    //         round_poly.push(bh_partials);
    //     }

    //     SumCheckProof {
    //         polynomial: poly.clone(),
    //         round_poly: round_poly.clone(),
    //         round_0_poly: round_0_poly.clone(),
    //         sum: sum.clone(),
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_test_curves::bls12_381::Fr;
    use polynomial::interface::{MultilinearPolynomialInterface, PolynomialInterface};
    use polynomial::univariant::UnivariantPolynomial;

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
        let round_0_poly_vec = ComposedProver::compute_round_zero_poly(&composed, &mut transcript);
        println!("round_0_poly_vec: {:?}", round_0_poly_vec);

        let round_0_poly_instance = RoundPoly::new(round_0_poly_vec);
        let round_0_poly = round_0_poly_instance.interpolate();

        let sum_half_0 = round_0_poly.evaluate(&Fr::from(0));
        let sum_half_1 = round_0_poly.evaluate(&Fr::from(1));

        println!("round_0_poly: {:?}", round_0_poly);

        assert_eq!(sum, sum_half_0 + sum_half_1);
    }

    // #[test]
    // fn test_compute_round_zero_poly_2() {
    //     let poly = Multilinear::new(
    //         vec![
    //             Fr::from(0),
    //             Fr::from(0),
    //             Fr::from(0),
    //             Fr::from(2),
    //             Fr::from(2),
    //             Fr::from(2),
    //             Fr::from(2),
    //             Fr::from(4),
    //         ],
    //         3,
    //     );

    //     let composed = ComposedMultilinear::new(vec![poly]);

    //     let sum = ComposedProver::calculate_sum(&composed);

    //     let mut transcript = FiatShamirTranscript::default();
    //     let round_0_poly = ComposedProver::compute_round_zero_poly(&composed, &mut transcript);

    //     let sum_half_0 = round_0_poly.evaluate(&vec![Fr::from(0)]).unwrap();
    //     let sum_half_1 = round_0_poly.evaluate(&vec![Fr::from(1)]).unwrap();

    //     println!("round_0_poly_vec: {:?}", round_0_poly);

    //     assert_eq!(sum, sum_half_0 + sum_half_1);
    // }

    // #[test]
    // fn test_compute_round_zero_poly_3() {
    //     let poly = Multilinear::new(
    //         vec![
    //             Fr::from(0),
    //             Fr::from(0),
    //             Fr::from(0),
    //             Fr::from(2),
    //             Fr::from(2),
    //             Fr::from(2),
    //             Fr::from(2),
    //             Fr::from(4),
    //         ],
    //         3,
    //     );

    //     let composed = ComposedMultilinear::new(vec![poly]);

    //     let sum = ComposedProver::calculate_sum(&composed);

    //     let mut transcript = FiatShamirTranscript::default();
    //     let round_0_poly = ComposedProver::compute_round_zero_poly(&composed, &mut transcript);

    //     let sum_half_0 = round_0_poly.evaluate(&vec![Fr::from(0)]).unwrap();
    //     let sum_half_1 = round_0_poly.evaluate(&vec![Fr::from(1)]).unwrap();

    //     println!("round_0_poly_vec: {:?}", round_0_poly);

    //     assert_eq!(sum, sum_half_0 + sum_half_1);
    // }
}
