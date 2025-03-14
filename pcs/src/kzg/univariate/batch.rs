use crate::{
    interface::BatchKZGUnivariateInterface,
    primitives::SRS,
    utils::{
        generate_vanishing_polynomial, linear_combination_homomorphic_poly_eval_g1_primefield,
        linear_combination_homomorphic_poly_eval_g2_primefield,
    },
};
use ark_ec::{pairing::Pairing, Group};
use ark_ff::PrimeField;
use polynomial::interface::{PolynomialInterface, UnivariantPolynomialInterface};
use polynomial::univariant::UnivariantPolynomial;
use std::ops::Sub;

use super::UnivariateKZG;

impl<P: Pairing> BatchKZGUnivariateInterface<P> for UnivariateKZG {
    fn open<F: PrimeField>(
        srs: &SRS<P>,
        poly: &UnivariantPolynomial<F>,
        point: &Vec<F>,
    ) -> (Vec<F>, <P as Pairing>::G1) {
        let point_evaluations: Vec<F> = point.iter().map(|point| poly.evaluate(point)).collect();
        let i_poly = UnivariantPolynomial::interpolate(point_evaluations.clone(), point.clone());
        let vanishing_polynomial = generate_vanishing_polynomial(&point);
        let quotient =
            <UnivariantPolynomial<F> as Sub>::sub(poly.clone(), i_poly) / vanishing_polynomial;

        let proof = linear_combination_homomorphic_poly_eval_g1_primefield::<P, F>(
            &quotient,
            &srs.g1_power_of_taus,
        );

        (point_evaluations, proof)
    }

    fn verify<F: PrimeField>(
        srs: &SRS<P>,
        commitment: &<P as Pairing>::G1,
        point: &Vec<F>,
        point_evaluation: &Vec<F>,
        proof: &<P as Pairing>::G1,
    ) -> bool {
        let r = UnivariantPolynomial::interpolate(point_evaluation.clone(), point.clone());

        // first check: f(x) = r(x)
        for (i, p) in point.iter().enumerate() {
            if r.evaluate(p) != point_evaluation[i] {
                return false;
            }
        }

        // second check: e(commitment - reminder_poly_commitment, g) = e(proof, vanishing_polynomial_commitment)
        let g2_generator = P::G2::generator();
        let vanishing_polynomial = generate_vanishing_polynomial(&point);
        let r_commitment = linear_combination_homomorphic_poly_eval_g1_primefield::<P, F>(
            &r,
            &srs.g1_power_of_taus,
        );
        let vanishing_poly_commitment = linear_combination_homomorphic_poly_eval_g2_primefield::<
            P,
            F,
        >(&vanishing_polynomial, &srs.g2_power_of_tau);

        let lhs = P::pairing(*commitment - r_commitment, g2_generator);
        let rhs = P::pairing(*proof, vanishing_poly_commitment);

        lhs == rhs
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interface::KZGUnivariateInterface;
    use ark_test_curves::bls12_381::{Bls12_381, Fr};

    #[test]
    fn test_batch_univariate_kzg() {
        let tau = Fr::from(10u64);
        let poly_degree = 4;
        let srs: SRS<Bls12_381> = UnivariateKZG::generate_srs(&tau, poly_degree);

        let poly = UnivariantPolynomial::new(vec![
            Fr::from(1u64),
            Fr::from(2u64),
            Fr::from(3u64),
            Fr::from(4u64),
            Fr::from(5u64),
        ]);
        let commitment = UnivariateKZG::commit(&srs, &poly);
        let (point_evaluation, proof) =
            <UnivariateKZG as BatchKZGUnivariateInterface<Bls12_381>>::open::<Fr>(
                &srs,
                &poly,
                &vec![Fr::from(2u64), Fr::from(3u64)],
            );
        let is_valid = <UnivariateKZG as BatchKZGUnivariateInterface<Bls12_381>>::verify::<Fr>(
            &srs,
            &commitment,
            &vec![Fr::from(2u64), Fr::from(3u64)],
            &point_evaluation,
            &proof,
        );

        assert!(is_valid);
    }

    #[test]
    fn test_batch_univariate_kzg_invalid() {
        let tau = Fr::from(10u64);
        let poly_degree = 4;
        let srs: SRS<Bls12_381> = UnivariateKZG::generate_srs(&tau, poly_degree);

        let poly = UnivariantPolynomial::new(vec![
            Fr::from(1u64),
            Fr::from(2u64),
            Fr::from(3u64),
            Fr::from(4u64),
            Fr::from(5u64),
        ]);
        let commitment = UnivariateKZG::commit(&srs, &poly);
        let (point_evaluation, proof) =
            <UnivariateKZG as BatchKZGUnivariateInterface<Bls12_381>>::open::<Fr>(
                &srs,
                &poly,
                &vec![Fr::from(2u64), Fr::from(3u64)],
            );
        let is_valid = <UnivariateKZG as BatchKZGUnivariateInterface<Bls12_381>>::verify::<Fr>(
            &srs,
            &commitment,
            &vec![Fr::from(2u64), Fr::from(5u64)],
            &point_evaluation,
            &proof,
        );

        assert!(!is_valid);
    }

    #[test]
    fn test_batch_univariate_kzg_nunber_of_opening_higher_than_degree() {
        let tau = Fr::from(10u64);
        let poly_degree = 6;
        let srs: SRS<Bls12_381> = UnivariateKZG::generate_srs(&tau, poly_degree);

        let poly = UnivariantPolynomial::new(vec![
            Fr::from(1u64),
            Fr::from(2u64),
            Fr::from(3u64),
            Fr::from(4u64),
            Fr::from(5u64),
        ]);
        let commitment = UnivariateKZG::commit(&srs, &poly);
        let (point_evaluation, proof) =
            <UnivariateKZG as BatchKZGUnivariateInterface<Bls12_381>>::open::<Fr>(
                &srs,
                &poly,
                &vec![
                    Fr::from(2u64),
                    Fr::from(3u64),
                    Fr::from(4u64),
                    Fr::from(5u64),
                    Fr::from(6u64),
                ],
            );
        let is_valid = <UnivariateKZG as BatchKZGUnivariateInterface<Bls12_381>>::verify::<Fr>(
            &srs,
            &commitment,
            &vec![
                Fr::from(2u64),
                Fr::from(3u64),
                Fr::from(4u64),
                Fr::from(5u64),
                Fr::from(6u64),
            ],
            &point_evaluation,
            &proof,
        );

        assert!(is_valid);
    }
}
