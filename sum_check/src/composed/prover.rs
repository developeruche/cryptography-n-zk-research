use crate::{data_structure::SumCheckProof, interface::ComposedProverInterface};
use ark_ff::PrimeField;
use fiat_shamir::{interface::TranscriptInterface, FiatShamirTranscript};
use polynomial::{
    composed::{interfaces::ComposedMultilinearInterface, multilinear::ComposedMultilinear},
    interface::MultilinearPolynomialInterface,
    multilinear::Multilinear,
    utils::boolean_hypercube,
};

#[derive(Clone, Default, Debug)]
pub struct ComposedProver;

impl<F: PrimeField> ComposedProverInterface<F> for ComposedProver {
    /// This function returns the sum of the multilinear polynomial evaluation over the boolean hypercube.
    fn calculate_sum(poly: &ComposedMultilinear<F>) -> F {
        poly.elementwise_product().iter().sum()
    }

    // /// This function returns the round zero computed polynomial
    // fn compute_round_zero_poly<P: MultilinearPolynomialInterface<F>>(
    //     poly: &P,
    //     transcript: &mut FiatShamirTranscript,
    // ) -> P {
    //     let number_of_round = poly.num_vars() - 1;
    //     let bh = boolean_hypercube(number_of_round);
    //     let mut bh_partials = P::zero(1); // this is an accumulator

    //     for bh_i in bh {
    //         let current_partial = poly.partial_evaluations(bh_i, vec![1; number_of_round]);
    //         bh_partials.internal_add_assign(&current_partial);
    //     }

    //     transcript.append(bh_partials.to_bytes());
    //     bh_partials
    // }

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

    #[test]
    fn test_calculate_sum() {
        let poly1 = Multilinear::new(vec![Fr::from(0), Fr::from(1), Fr::from(2), Fr::from(3)], 2);
        let poly2 = Multilinear::new(vec![Fr::from(0), Fr::from(0), Fr::from(0), Fr::from(1)], 2);

        let composed = ComposedMultilinear::new(vec![poly1, poly2]);

        let sum = ComposedProver::calculate_sum(&composed);
        assert_eq!(sum, Fr::from(3));
    }
}
