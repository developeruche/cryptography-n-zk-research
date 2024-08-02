use super::{prover::ComposedProver, ComposedSumCheckProof, RoundPoly};
use crate::{
    composed::utils::{compute_multi_composed_bytes, perform_multi_partial_eval},
    interface::{ComposedProverInterface, MultiComposedProverInterface},
};
use ark_ff::{BigInteger, PrimeField};
use fiat_shamir::interface::TranscriptInterface;
use fiat_shamir::FiatShamirTranscript;
use polynomial::interface::MultilinearPolynomialInterface;
use polynomial::{
    composed::{interfaces::ComposedMultilinearInterface, multilinear::ComposedMultilinear},
    univariant::UnivariantPolynomial,
};

#[derive(Clone, Default, Debug)]
pub struct MultiComposedProver;

impl<F: PrimeField> MultiComposedProverInterface<F> for MultiComposedProver {
    fn calculate_sum(poly: &[ComposedMultilinear<F>]) -> F {
        let mut sum = F::zero();

        for p in poly.iter() {
            sum += ComposedProver::calculate_sum(p);
        }

        sum
    }

    fn sum_check_proof(
        poly_: &[ComposedMultilinear<F>],
        transcript: &mut FiatShamirTranscript,
        sum: &F,
    ) -> (ComposedSumCheckProof<F>, Vec<F>) {
        let mut poly = poly_.to_vec();
        let mut all_random_reponse = Vec::new();
        let mut round_polys = Vec::new();

        transcript.append(compute_multi_composed_bytes(&poly));
        transcript.append(sum.into_bigint().to_bytes_be());

        for _ in 0..poly[0].num_vars() {
            let mut round_poly = UnivariantPolynomial::zero();

            for poly_i in poly.iter() {
                let mut round_i_poly_vec = Vec::new();

                for i in 0..=poly_i.max_degree() {
                    let instance = poly_i
                        .partial_evaluation(F::from(i as u128), 0)
                        .elementwise_product()
                        .iter()
                        .sum();
                    round_i_poly_vec.push(instance);
                }

                let round_i_poly = RoundPoly::new(round_i_poly_vec).interpolate();
                round_poly += round_i_poly;
            }

            transcript.append(round_poly.to_bytes());

            let random_response = F::from_be_bytes_mod_order(&transcript.sample());
            poly = perform_multi_partial_eval(&poly, random_response, 0);

            all_random_reponse.push(random_response);
            round_polys.push(round_poly);
        }

        (
            ComposedSumCheckProof {
                round_poly: round_polys,
                sum: *sum,
            },
            all_random_reponse,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_test_curves::bls12_381::Fr;
    use polynomial::multilinear::Multilinear;

    #[test]
    fn test_calculate_sum_1() {
        let poly1 = Multilinear::new(vec![Fr::from(0), Fr::from(1), Fr::from(2), Fr::from(3)], 2);
        let poly2 = Multilinear::new(vec![Fr::from(0), Fr::from(0), Fr::from(0), Fr::from(1)], 2);

        let composed_1 = ComposedMultilinear::new(vec![poly1]);
        let composed_2 = ComposedMultilinear::new(vec![poly2]);

        let multi_composed = vec![composed_1, composed_2];

        let sum = MultiComposedProver::calculate_sum(&multi_composed);

        assert_eq!(sum, Fr::from(7u32));
    }

    #[test]
    fn test_calculate_sum_2() {
        let poly1 = Multilinear::new(vec![Fr::from(0), Fr::from(0), Fr::from(0), Fr::from(2)], 2);
        let poly2 = Multilinear::new(vec![Fr::from(0), Fr::from(3), Fr::from(0), Fr::from(3)], 2);

        let composed_1 = ComposedMultilinear::new(vec![poly1]);
        let composed_2 = ComposedMultilinear::new(vec![poly2]);

        let multi_composed = vec![composed_1, composed_2];

        let sum = MultiComposedProver::calculate_sum(&multi_composed);

        assert_eq!(sum, Fr::from(8u32));
    }
}
