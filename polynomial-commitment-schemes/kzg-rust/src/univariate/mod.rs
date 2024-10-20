use crate::{
    interface::KZGUnivariateInterface,
    primitives::SRS,
    utils::{
        generate_powers_of_tau_g1, generate_powers_of_tau_g2,
        linear_combination_homomorphic_poly_eval_g1,
        linear_combination_homomorphic_poly_eval_g1_primefield,
    },
};
use ark_ec::{pairing::Pairing, Group};
use ark_ff::PrimeField;
use polynomial::interface::PolynomialInterface;
use polynomial::univariant::UnivariantPolynomial;

pub mod batch;

#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct UnivariateKZG;

impl<P: Pairing> KZGUnivariateInterface<P> for UnivariateKZG {
    fn generate_srs(tau: &P::ScalarField, poly_degree: usize) -> SRS<P> {
        let g1_power_of_taus = generate_powers_of_tau_g1::<P>(tau, poly_degree);
        let g2_power_of_tau = generate_powers_of_tau_g2::<P>(tau, poly_degree);

        SRS {
            g1_power_of_taus,
            g2_power_of_tau,
        }
    }

    fn commit<F: PrimeField>(srs: &SRS<P>, poly: &UnivariantPolynomial<F>) -> P::G1 {
        linear_combination_homomorphic_poly_eval_g1::<P, F>(poly, &srs.g1_power_of_taus)
    }

    fn open<F: PrimeField>(srs: &SRS<P>, poly: &UnivariantPolynomial<F>, point: &F) -> (F, P::G1) {
        let point_evaluation = poly.evaluate(point);
        let divisor = UnivariantPolynomial::new(vec![-*point, F::one()]);
        let numerator = poly - point_evaluation;
        let quotient = numerator / divisor;

        let proof = linear_combination_homomorphic_poly_eval_g1_primefield::<P, F>(
            &quotient,
            &srs.g1_power_of_taus,
        );

        (point_evaluation, proof)
    }

    fn verify<F: PrimeField>(
        srs: &SRS<P>,
        commitment: &P::G1,
        point: &F,
        point_evaluation: &F,
        proof: &P::G1,
    ) -> bool {
        let g2_generator = P::G2::generator();
        let g1_point_evalauation =
            srs.g1_power_of_taus[0].mul_bigint(point_evaluation.into_bigint());
        let g2_point = g2_generator.mul_bigint(point.into_bigint());

        let left_pairing = P::pairing(*commitment - g1_point_evalauation, g2_generator);
        let right_pairing = P::pairing(*proof, srs.g2_power_of_tau[1] - g2_point);

        left_pairing == right_pairing
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::generate_vanishing_polynomial;

    use super::*;
    use ark_test_curves::bls12_381::{Bls12_381, Fr};
    use polynomial::interface::UnivariantPolynomialInterface;

    #[test]
    fn test_univariate_kzg() {
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
        let (point_evaluation, proof) = UnivariateKZG::open::<Fr>(&srs, &poly, &Fr::from(2u64));
        let is_valid = UnivariateKZG::verify::<Fr>(
            &srs,
            &commitment,
            &Fr::from(2u64),
            &point_evaluation,
            &proof,
        );

        assert!(is_valid);
    }

    #[test]
    fn test_univariate_kzg_invalid_opening() {
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
        let (point_evaluation, proof) = UnivariateKZG::open::<Fr>(&srs, &poly, &Fr::from(2u64));
        let is_valid = UnivariateKZG::verify::<Fr>(
            &srs,
            &commitment,
            &Fr::from(4u64),
            &point_evaluation,
            &proof,
        );

        assert_eq!(is_valid, false);
    }

    #[test]
    fn exp_test_univariate_kzg_invalid_opening() {
        let poly = UnivariantPolynomial::new(vec![Fr::from(3u64), Fr::from(2u64), Fr::from(4u64)]);
        let opening = vec![Fr::from(1u64), Fr::from(5u64), Fr::from(3u64)];
        let opening_evaluautions = vec![Fr::from(9u64), Fr::from(113u64), Fr::from(45u64)];

        let vanishing_poly = generate_vanishing_polynomial(&opening);

        let (q, r) = poly.divide_with_q_and_r(&vanishing_poly).unwrap();

        let t_eval_0 = r.evaluate(&opening[0]);
        let t_eval_1 = r.evaluate(&opening[1]);
        let t_eval_2 = r.evaluate(&opening[2]);

        let f_eval_opening_0 = opening_evaluautions[0];
        let f_eval_opening_1 = opening_evaluautions[1];
        let f_eval_opening_2 = opening_evaluautions[2];

        assert_eq!(t_eval_0, f_eval_opening_0);
        assert_eq!(t_eval_1, f_eval_opening_1);
        assert_eq!(t_eval_2, f_eval_opening_2);

        let expected_r = UnivariantPolynomial::interpolate(opening_evaluautions, opening);

        assert_eq!(r, expected_r);
    }
}
