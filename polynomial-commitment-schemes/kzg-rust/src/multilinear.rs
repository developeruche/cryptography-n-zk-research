use ark_ec::pairing::Pairing;
use ark_ff::PrimeField;
use polynomial::utils::boolean_hypercube;

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
}
