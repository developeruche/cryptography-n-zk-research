use ark_ff::PrimeField;
use polynomial::composed::multilinear::ComposedMultilinear;

use crate::interface::{ComposedProverInterface, MultiComposedProverInterface};

use super::prover::ComposedProver;

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

    fn compute_round_zero_poly(
        poly: &[ComposedMultilinear<F>],
        transcript: &mut fiat_shamir::FiatShamirTranscript,
    ) -> polynomial::univariant::UnivariantPolynomial<F> {
        todo!()
    }

    fn sum_check_proof(
        poly: &[ComposedMultilinear<F>],
        transcript: &mut fiat_shamir::FiatShamirTranscript,
        sum: &F,
    ) -> (super::ComposedSumCheckProof<F>, Vec<F>) {
        todo!()
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
