use crate::{data_structure::SumCheckProof, interface::ProverInterface};
use ark_ff::Field;
use fiat_shamir::FiatShamirTranscript;
use polynomial::{interface::MultivariantPolynomialInterface, multilinear::Multilinear, utils::boolean_hypercube};

#[derive(Clone, Default, Debug)]
pub struct Prover<F: Field> {
    /// This is the polynomial to calculate the sum check proof
    pub poly: Multilinear<F>,
    /// This struct is used to store the sum check proof
    pub round_poly: Vec<Multilinear<F>>,
    /// This vectors store the polynomial from the first round
    pub round_0_poly: Multilinear<F>,
    /// This holds the sum of the polynomial evaluation over the boolean hypercube
    pub sum: F,
    /// This is this fiat-shamir challenge transcript
    pub transcript: FiatShamirTranscript,
}

impl<F: Field> Prover<F> {
    /// This function creates a new prover instance
    pub fn new(poly: Multilinear<F>) -> Self {
        Self {
            poly,
            round_poly: Default::default(),
            round_0_poly: Default::default(),
            sum: Default::default(),
            transcript: Default::default(),
        }
    }
    
    /// This function crates a new prover instance with sum 
    pub fn new_with_sum(poly: Multilinear<F>, sum: F) -> Self {
        Self {
            poly,
            round_poly: Default::default(),
            round_0_poly: Default::default(),
            sum,
            transcript: Default::default(),
        }
    }
}

impl<F: Field> ProverInterface<F> for Prover<F> {
    /// This function returns the sum of the multilinear polynomial evaluation over the boolean hypercube.
    fn calculate_sum(&mut self) {
        self.sum = self.poly.evaluations.iter().sum();
    }

    /// This function returns the round zero computed polynomial
    fn compute_round_zero_poly(&mut self) {
        let number_of_round = self.poly.num_vars - 1;
        let bh = boolean_hypercube(number_of_round);
        let mut bh_partials: Multilinear<F> = Multilinear::zero(1);
        
        for bh_i in bh {
            let current_partial =  self.poly.partial_evaluations(bh_i, vec![1; number_of_round]);
            println!("Current partial {:?}", current_partial);
            bh_partials += current_partial;
        }
        
        self.round_0_poly = bh_partials;
    }

    /// This function returns poly cimouted in round j
    fn compute_round_j_poly(&mut self, j: usize) -> Multilinear<F> {
        unimplemented!("Implement this function")
    }

    /// This function computes sum check proof
    fn sum_check_proof(&self) -> SumCheckProof<F> {
        unimplemented!("Implement this function");

        // SumCheckProof {
        //     round_poly: self.round_poly.clone(),
        //     round_0_poly: self.round_0_poly.clone(),
        //     sum: self.sum,
        // }
    }
}










#[cfg(test)]
mod tests {
    use super::*;
    use ark_test_curves::bls12_381::Fr;

    #[test]
    fn test_sum_calculation() {
        let poly = Multilinear::new(vec![Fr::from(0), Fr::from(0), Fr::from(0), Fr::from(2), Fr::from(2), Fr::from(2), Fr::from(2), Fr::from(4)], 3);
        let mut prover = Prover::new(poly);
        prover.calculate_sum();
        assert_eq!(prover.sum, Fr::from(12));
    }
    
    #[test]
    fn test_compute_round_zero_poly() {
        let poly = Multilinear::new(vec![Fr::from(0), Fr::from(0), Fr::from(0), Fr::from(2), Fr::from(2), Fr::from(2), Fr::from(2), Fr::from(4)], 3);
        let mut prover = Prover::new(poly);
        prover.compute_round_zero_poly();
        assert_eq!(prover.round_0_poly.evaluations, vec![Fr::from(2), Fr::from(10)]);
    }
    
    #[test]
    fn test_compute_round_zero_poly_2() {
        let poly = Multilinear::new(vec![Fr::from(0), Fr::from(0), Fr::from(0), Fr::from(2), Fr::from(2), Fr::from(2), Fr::from(2), Fr::from(4)], 3);
        let mut prover = Prover::new(poly);
        prover.compute_round_zero_poly();
        let sum = prover.round_0_poly.evaluate(&vec![Fr::from(1)]).unwrap() + prover.round_0_poly.evaluate(&vec![Fr::from(0)]).unwrap();
        assert_eq!(sum, Fr::from(12));
    }
    
    #[test]
    fn test_compute_round_zero_poly_3() {
        let poly = Multilinear::new(vec![Fr::from(0), Fr::from(0), Fr::from(2), Fr::from(7), Fr::from(3), Fr::from(3), Fr::from(5), Fr::from(11)], 3);
        let mut prover = Prover::new(poly);
        prover.compute_round_zero_poly();
        assert_eq!(prover.round_0_poly.evaluations, vec![Fr::from(9), Fr::from(22)]);
    }
    
    #[test]
    fn test_compute_round_zero_poly_4() {
        let poly = Multilinear::new(vec![Fr::from(0), Fr::from(0), Fr::from(2), Fr::from(7), Fr::from(3), Fr::from(3), Fr::from(5), Fr::from(11)], 3);
        let mut prover = Prover::new(poly);
        prover.compute_round_zero_poly();
        let sum = prover.round_0_poly.evaluate(&vec![Fr::from(1)]).unwrap() + prover.round_0_poly.evaluate(&vec![Fr::from(0)]).unwrap();
        assert_eq!(sum, Fr::from(31));
    }
}
