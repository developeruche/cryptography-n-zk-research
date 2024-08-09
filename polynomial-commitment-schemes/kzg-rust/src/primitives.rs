use ark_ec::{pairing::Pairing, Group};

#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct SRS<P: Pairing> {
    pub g1_power_of_taus: Vec<P::G1>,
    pub g2_power_of_tau: P::G2,
}
