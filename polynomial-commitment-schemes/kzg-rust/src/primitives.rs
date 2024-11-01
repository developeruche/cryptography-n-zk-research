use ark_ec::pairing::Pairing;

#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct SRS<P: Pairing> {
    pub g1_power_of_taus: Vec<P::G1>,
    // this is a vector not just an element so it would support batch kzg
    pub g2_power_of_tau: Vec<P::G2>,
}

#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct MultiLinearSRS<P: Pairing> {
    pub g1_power_of_taus: Vec<P::G1>, // this is an expression of g1^tau^i over the boolean hypercube (every possible combination of each monomial)
    pub g2_power_of_taus: Vec<P::G2>, // this is a vector of g2^tau^i
}
