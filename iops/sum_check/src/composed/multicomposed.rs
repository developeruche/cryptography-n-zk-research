use super::{prover::ComposedProver, utils::perform_multi_eval, ComposedSumCheckProof, RoundPoly};
use crate::{
    composed::utils::{compute_multi_composed_bytes, perform_multi_partial_eval},
    interface::{
        ComposedProverInterface, MultiComposedProverInterface, MultiComposedVerifierInterface,
    },
};
use ark_ff::{BigInteger, PrimeField};
use fiat_shamir::interface::TranscriptInterface;
use fiat_shamir::FiatShamirTranscript;
use polynomial::interface::{MultilinearPolynomialInterface, PolynomialInterface};
use polynomial::{
    composed::{interfaces::ComposedMultilinearInterface, multilinear::ComposedMultilinear},
    univariant::UnivariantPolynomial,
};

#[derive(Clone, Default, Debug)]
pub struct MultiComposedProver;

#[derive(Clone, Default, Debug)]
pub struct MultiComposedVerifier;

#[derive(Clone, Default, Debug)]
pub struct IntermidateClaimCheck<F: PrimeField> {
    pub claimed_sum: F,
    pub random_challenges: Vec<F>,
}

impl<F: PrimeField> MultiComposedProverInterface<F> for MultiComposedProver {
    fn calculate_sum(poly: &[ComposedMultilinear<F>]) -> F {
        let mut sum = F::zero();

        for p in poly.iter() {
            sum += ComposedProver::calculate_sum(p);
        }

        sum
    }

    fn sum_check_proof(
        poly: &[ComposedMultilinear<F>],
        sum: &F,
    ) -> (ComposedSumCheckProof<F>, Vec<F>) {
        let mut transcript = FiatShamirTranscript::default();

        transcript.append(compute_multi_composed_bytes(&poly));

        Self::sum_check_proof_internal(poly, &mut transcript, sum)
    }

    fn sum_check_proof_without_initial_polynomial(
        poly: &[ComposedMultilinear<F>],
        sum: &F,
    ) -> (ComposedSumCheckProof<F>, Vec<F>) {
        let mut transcript = FiatShamirTranscript::default();

        Self::sum_check_proof_internal(poly, &mut transcript, sum)
    }

    fn sum_check_proof_internal(
        poly_: &[ComposedMultilinear<F>],
        transcript: &mut FiatShamirTranscript,
        sum: &F,
    ) -> (ComposedSumCheckProof<F>, Vec<F>) {
        let mut poly = poly_.to_vec();
        let mut all_random_reponse = Vec::new();
        let mut round_polys = Vec::new();

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

impl<F: PrimeField> MultiComposedVerifierInterface<F> for MultiComposedVerifier {
    fn verify_internal(
        proof: &ComposedSumCheckProof<F>,
        transcript: &mut FiatShamirTranscript,
    ) -> Result<IntermidateClaimCheck<F>, &'static str> {
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
                return Err("untrusted_sum != proof.sum");
            }

            let sample = F::from_be_bytes_mod_order(&transcript.sample());
            mutating_sum = r_poly.evaluate(&sample);
            all_rands.push(sample);
        }

        Ok(IntermidateClaimCheck {
            claimed_sum: mutating_sum,
            random_challenges: all_rands,
        })
    }

    fn verify_except_last_check(proof: &ComposedSumCheckProof<F>) -> IntermidateClaimCheck<F> {
        let mut transcript = FiatShamirTranscript::default();
        Self::verify_internal(proof, &mut transcript).unwrap()
    }

    fn verify(proof: &ComposedSumCheckProof<F>, poly: &[ComposedMultilinear<F>]) -> bool {
        let mut transcript = FiatShamirTranscript::default();

        transcript.append(compute_multi_composed_bytes(&poly));
        let intermidate_claim = Self::verify_internal(proof, &mut transcript).unwrap();

        intermidate_claim.complete(poly)
    }
}

impl<F: PrimeField> IntermidateClaimCheck<F> {
    pub fn complete(&self, poly: &[ComposedMultilinear<F>]) -> bool {
        let eval = perform_multi_eval(poly, &self.random_challenges);

        eval.iter().sum::<F>() == self.claimed_sum
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

    #[test]
    fn test_multi_composed_sum_check_proof() {
        let poly1 = Multilinear::new(vec![Fr::from(0), Fr::from(0), Fr::from(0), Fr::from(2)], 2);
        let poly2 = Multilinear::new(vec![Fr::from(0), Fr::from(3), Fr::from(0), Fr::from(3)], 2);

        let composed_1 = ComposedMultilinear::new(vec![poly1]);
        let composed_2 = ComposedMultilinear::new(vec![poly2]);

        let multi_composed = vec![composed_1, composed_2];
        let sum = MultiComposedProver::calculate_sum(&multi_composed);

        let (proof, _) = MultiComposedProver::sum_check_proof(&multi_composed, &sum);

        assert!(MultiComposedVerifier::verify(&proof, &multi_composed));
    }

    #[test]
    fn test_multi_composed_sum_check_proof_1() {
        let poly1 = Multilinear::new(vec![Fr::from(0), Fr::from(0), Fr::from(0), Fr::from(2)], 2);
        let poly2 = Multilinear::new(vec![Fr::from(0), Fr::from(3), Fr::from(0), Fr::from(3)], 2);

        let composed_1 = ComposedMultilinear::new(vec![poly1]);
        let composed_2 = ComposedMultilinear::new(vec![poly2.clone()]);
        let composed_3 = ComposedMultilinear::new(vec![poly2]);

        let multi_composed = vec![composed_1, composed_2, composed_3];
        let sum = MultiComposedProver::calculate_sum(&multi_composed);

        let (proof, _) = MultiComposedProver::sum_check_proof(&multi_composed, &sum);

        assert!(MultiComposedVerifier::verify(&proof, &multi_composed));
    }

    #[test]
    fn test_multi_composed_sum_check_proof_2() {
        let poly1 = Multilinear::new(vec![Fr::from(0), Fr::from(0), Fr::from(0), Fr::from(2)], 2);
        let poly2 = Multilinear::new(vec![Fr::from(0), Fr::from(3), Fr::from(0), Fr::from(3)], 2);

        let composed_1 = ComposedMultilinear::new(vec![poly1.clone(), poly2.clone()]);
        let composed_2 = ComposedMultilinear::new(vec![poly2.clone(), poly1.clone()]);

        let multi_composed = vec![composed_1, composed_2];
        let sum = MultiComposedProver::calculate_sum(&multi_composed);

        let (proof, _) = MultiComposedProver::sum_check_proof(&multi_composed, &sum);

        assert!(MultiComposedVerifier::verify(&proof, &multi_composed));
    }

    #[test]
    fn test_multi_composed_sum_check_proof_3() {
        let poly1 = Multilinear::new(vec![Fr::from(0), Fr::from(0), Fr::from(0), Fr::from(2)], 2);
        let poly2 = Multilinear::new(vec![Fr::from(0), Fr::from(3), Fr::from(0), Fr::from(3)], 2);

        let composed_1 = ComposedMultilinear::new(vec![poly1.clone(), poly2.clone()]);
        let composed_2 = ComposedMultilinear::new(vec![poly2.clone(), poly1.clone()]);

        let multi_composed = vec![composed_1.clone(), composed_2.clone(), composed_1];
        let sum = MultiComposedProver::calculate_sum(&multi_composed);

        let (proof, _) = MultiComposedProver::sum_check_proof(&multi_composed, &sum);

        assert!(MultiComposedVerifier::verify(&proof, &multi_composed));
    }

    #[test]
    fn test_multi_composed_sum_check_proof_2_on_gkr_example() {
        // f(a,b,c) = 2abc + 3b + 4
        let add_i = Multilinear::<Fr>::new(
            vec![
                Fr::from(4),
                Fr::from(4),
                Fr::from(7),
                Fr::from(7),
                Fr::from(4),
                Fr::from(4),
                Fr::from(7),
                Fr::from(9),
            ],
            3,
        );
        // f(b) = 4b
        let w_b = Multilinear::<Fr>::new(vec![Fr::from(0), Fr::from(4)], 1);
        // f(c) = 3c
        let w_c = Multilinear::<Fr>::new(vec![Fr::from(0), Fr::from(3)], 1);
        // f(a,b,c) = 2ab + bc + 3
        let mul_i = Multilinear::<Fr>::new(
            vec![
                Fr::from(3),
                Fr::from(3),
                Fr::from(3),
                Fr::from(4),
                Fr::from(3),
                Fr::from(3),
                Fr::from(5),
                Fr::from(6),
            ],
            3,
        );

        let lhs_poly = ComposedMultilinear::new(vec![
            add_i.partial_evaluation(Fr::from(2), 0),
            w_b.add_distinct(&w_c),
        ]);
        let rhs_poly = ComposedMultilinear::new(vec![
            mul_i.partial_evaluation(Fr::from(2), 0),
            w_b.mul_distinct(&w_c),
        ]);

        let multi_composed = vec![lhs_poly, rhs_poly];
        let sum = MultiComposedProver::calculate_sum(&multi_composed);

        let (proof, _) = MultiComposedProver::sum_check_proof(&multi_composed, &sum);

        assert!(MultiComposedVerifier::verify(&proof, &multi_composed));
    }

    #[test]
    fn test_multi_composed_sum_check_proof_verify_except_last_check() {
        let poly1 = Multilinear::new(vec![Fr::from(0), Fr::from(0), Fr::from(0), Fr::from(2)], 2);
        let poly2 = Multilinear::new(vec![Fr::from(0), Fr::from(3), Fr::from(0), Fr::from(3)], 2);

        let composed_1 = ComposedMultilinear::new(vec![poly1]);
        let composed_2 = ComposedMultilinear::new(vec![poly2]);

        let multi_composed = vec![composed_1, composed_2];
        let sum = MultiComposedProver::calculate_sum(&multi_composed);

        let (proof, _) =
            MultiComposedProver::sum_check_proof_without_initial_polynomial(&multi_composed, &sum);

        let intermidate_claim_check = MultiComposedVerifier::verify_except_last_check(&proof);

        assert!(intermidate_claim_check.complete(&multi_composed));
    }
}
