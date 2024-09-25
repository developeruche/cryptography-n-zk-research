use ark_ec::{
    pairing::Pairing,
    Group,
};
use ark_ff::PrimeField;
use polynomial::{
    interface::MultilinearPolynomialInterface, multilinear::Multilinear, utils::boolean_hypercube,
};

use crate::{
    interface::KZGMultiLinearInterface,
    primitives::MultiLinearSRS,
    utils::{bh_to_g1_srs, g2_operation},
};

#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct MultilinearKZG;

impl<P: Pairing> KZGMultiLinearInterface<P> for MultilinearKZG {
    fn generate_srs<F: PrimeField>(taus: &[F]) -> MultiLinearSRS<P> {
        let number_of_variables = taus.len();
        let boolean_hypercube = boolean_hypercube::<F>(number_of_variables);
        let g1_power_of_taus = bh_to_g1_srs::<F, P>(&boolean_hypercube, taus);
        let g2_power_of_taus = g2_operation::<F, P>(taus);

        MultiLinearSRS {
            g1_power_of_taus,
            g2_power_of_taus,
        }
    }

    fn commit<F: PrimeField>(srs: &MultiLinearSRS<P>, poly: &Multilinear<F>) -> P::G1 {
        poly.evaluations
            .iter()
            .zip(srs.g1_power_of_taus.iter())
            .map(|(eval, p1)| p1.mul_bigint(eval.into_bigint()))
            .sum()
    }

    fn open<F: PrimeField>(
        srs: &MultiLinearSRS<P>,
        poly: &Multilinear<F>,
        point: &[F],
    ) -> (F, Vec<<P as Pairing>::G1>) {
        let points_evaluations = poly.evaluate(&point.to_vec());
        let mut quotient_proofs = Vec::new();
        let mut last_reminder = poly.clone();

        for i in 0..poly.num_vars() {
            let (q, r) = last_reminder.divide_by_single_variable_linear(&point[i], 0);
            last_reminder = r;

            // commit to the quotient polynomial
            let scaled_quotient = q.leftappend_with_new_variables(1 + i);
            let quotient_commitment = Self::commit(srs, &scaled_quotient);
            quotient_proofs.push(quotient_commitment);
        }

        (points_evaluations.unwrap(), quotient_proofs)
    }

    fn verify<F: PrimeField>(
        srs: &MultiLinearSRS<P>,
        commitment: &<P as Pairing>::G1,
        point: &[F],
        point_evaluation: &F,
        proof: &Vec<<P as Pairing>::G1>,
    ) -> bool {
        let g2_generator = P::G2::generator();
        let g1_point_evalauation = P::G1::generator().mul_bigint(point_evaluation.into_bigint());

        let left_pairing = P::pairing(*commitment - g1_point_evalauation, g2_generator);
        let mut right_pairing = P::pairing(
            proof[0],
            srs.g2_power_of_taus[0] - g2_generator.mul_bigint(point[0].into_bigint()),
        );

        for i in 1..proof.len() {
            let g2_point = g2_generator.mul_bigint(point[i].into_bigint());
            right_pairing += P::pairing(proof[i], srs.g2_power_of_taus[i] - g2_point);
        }

        left_pairing == right_pairing
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_test_curves::bls12_381::{Bls12_381, Fr};

    fn group_unit_operation_g1<P: Pairing>(oprand: P::ScalarField) -> P::G1 {
        P::G1::generator().mul_bigint(oprand.into_bigint())
    }

    #[test]
    fn test_multilinear_kzg() {}

    #[test]
    fn test_multilinear_kzg_commit_one_variable_poly() {
        let srs: MultiLinearSRS<Bls12_381> = MultilinearKZG::generate_srs(&[Fr::from(5u32)]);
        let poly = Multilinear::new(vec![Fr::from(5), Fr::from(7)], 1);
        let commitment = MultilinearKZG::commit(&srs, &poly);

        let expected_commitment = group_unit_operation_g1::<Bls12_381>(Fr::from(15));

        assert_eq!(commitment, expected_commitment);
    }

    #[test]
    fn test_multilinear_kzg_commit_two_variable_poly() {
        let srs: MultiLinearSRS<Bls12_381> =
            MultilinearKZG::generate_srs(&[Fr::from(5u32), Fr::from(7u32)]);
        let poly = Multilinear::new(vec![Fr::from(3), Fr::from(3), Fr::from(7), Fr::from(10)], 2);
        let commitment = MultilinearKZG::commit(&srs, &poly);

        let expected_commitment = group_unit_operation_g1::<Bls12_381>(Fr::from(128));

        assert_eq!(commitment, expected_commitment);
    }

    #[test]
    fn test_multilinear_kzg_open_two_variable_poly() {
        let srs: MultiLinearSRS<Bls12_381> =
            MultilinearKZG::generate_srs(&[Fr::from(5u32), Fr::from(7u32)]);
        let poly = Multilinear::new(vec![Fr::from(3), Fr::from(3), Fr::from(7), Fr::from(10)], 2);
        let point = vec![Fr::from(2), Fr::from(3)];
        let (_, _) = MultilinearKZG::open(&srs, &poly, &point);
    }

    #[test]
    fn test_multilinear_kzg_verify_two_variable_poly() {
        let srs: MultiLinearSRS<Bls12_381> =
            MultilinearKZG::generate_srs(&[Fr::from(5u32), Fr::from(7u32)]);
        let poly = Multilinear::new(vec![Fr::from(3), Fr::from(3), Fr::from(7), Fr::from(10)], 2);
        let point = vec![Fr::from(2), Fr::from(3)];
        let (point_evaluation, proof) = MultilinearKZG::open(&srs, &poly, &point);
        let commitment = MultilinearKZG::commit(&srs, &poly);
        let result = MultilinearKZG::verify(&srs, &commitment, &point, &point_evaluation, &proof);
        assert_eq!(result, true);
    }

    #[test]
    fn test_multilinear_kzg_verify_three_variable_poly() {
        let srs: MultiLinearSRS<Bls12_381> =
            MultilinearKZG::generate_srs(&[Fr::from(5u32), Fr::from(7u32), Fr::from(11u32)]);
        let poly = Multilinear::new(
            vec![
                Fr::from(4),
                Fr::from(5),
                Fr::from(4),
                Fr::from(5),
                Fr::from(6),
                Fr::from(7),
                Fr::from(9),
                Fr::from(10),
            ],
            3,
        );
        let point = vec![Fr::from(2), Fr::from(3), Fr::from(4)];
        let (point_evaluation, proof) = MultilinearKZG::open(&srs, &poly, &point);
        let commitment = MultilinearKZG::commit(&srs, &poly);
        let result = MultilinearKZG::verify(&srs, &commitment, &point, &point_evaluation, &proof);
        assert_eq!(result, true);
    }

    #[test]
    #[ignore]
    fn test_multilinear_kzg_verify_10_variable_poly() {
        let srs: MultiLinearSRS<Bls12_381> = MultilinearKZG::generate_srs(&[
            Fr::from(5u32),
            Fr::from(7u32),
            Fr::from(11u32),
            Fr::from(13u32),
            Fr::from(17u32),
            Fr::from(19u32),
            Fr::from(23u32),
            Fr::from(29u32),
            Fr::from(31u32),
            Fr::from(37u32),
        ]);
        let poly = Multilinear::random(10);
        let point = vec![
            Fr::from(2),
            Fr::from(3),
            Fr::from(4),
            Fr::from(5),
            Fr::from(6),
            Fr::from(7),
            Fr::from(8),
            Fr::from(9),
            Fr::from(10),
            Fr::from(11),
        ];
        let (point_evaluation, proof) = MultilinearKZG::open(&srs, &poly, &point);
        let commitment = MultilinearKZG::commit(&srs, &poly);
        let result = MultilinearKZG::verify(&srs, &commitment, &point, &point_evaluation, &proof);
        assert_eq!(result, true);
    }
}
