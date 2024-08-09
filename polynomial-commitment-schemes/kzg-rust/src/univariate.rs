use crate::{
    interface::KZGUnivariateInterface,
    primitives::SRS,
    utils::{generate_powers_of_tau_g1, linear_combination_homomorphic_poly_eval_g1},
};
use ark_ec::{pairing::Pairing, Group};
use ark_ff::PrimeField;
use polynomial::univariant::UnivariantPolynomial;

#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct UnivariateKZG;

impl<P: Pairing> KZGUnivariateInterface<P> for UnivariateKZG {
    fn generate_srs(tau: &<P as Pairing>::ScalarField, poly_degree: usize) -> SRS<P> {
        let g1_power_of_taus = generate_powers_of_tau_g1::<P>(tau, poly_degree);
        let g2_power_of_tau = P::G2::generator().mul_bigint(tau.into_bigint());

        SRS {
            g1_power_of_taus,
            g2_power_of_tau,
        }
    }

    fn commit(
        srs: &SRS<P>,
        poly: &UnivariantPolynomial<<P as Pairing>::ScalarField>,
    ) -> <P as Pairing>::G1 {
        linear_combination_homomorphic_poly_eval_g1::<P>(poly, &srs.g1_power_of_taus)
    }

    fn open(
        srs: &SRS<P>,
        poly: &UnivariantPolynomial<<P as Pairing>::ScalarField>,
        point: &<P as Pairing>::ScalarField,
    ) -> (<P as Pairing>::ScalarField, <P as Pairing>::G1) {
        todo!()
    }

    fn verify(
        srs: &SRS<P>,
        commitment: &<P as Pairing>::G1,
        point: &<P as Pairing>::ScalarField,
        point_evaluation: &<P as Pairing>::ScalarField,
        proof: &<P as Pairing>::G1,
    ) -> bool {
        let g2_generator = P::G2::generator();
        let g1_point_evalauation =
            srs.g1_power_of_taus[0].mul_bigint(point_evaluation.into_bigint());
        let g2_point = g2_generator.mul_bigint(point.into_bigint());

        let left_pairing = P::pairing(*commitment - g1_point_evalauation, g2_generator);
        let right_pairing = P::pairing(*proof, srs.g2_power_of_tau - g2_point);

        left_pairing == right_pairing
    }
}
