use ark_ff::Field;
use polynomial::multilinear::Multilinear;
use fiat_shamir::FiatShamirTranscript;

use crate::{data_structure::SumCheckProof, interface::ProverInterface};




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
}




impl<F: Field> ProverInterface<F> for Prover<F> {
    /// This function returns the sum of the multilinear polynomial evaluation over the boolean hypercube.
    fn calculate_sum(&self, poly: &Multilinear<F>) -> F {
        let mut sum = F::zero();
        for eval in poly.evaluations.iter() {
            sum += *eval;
        }
        sum
    }

    /// This function returns the round zero computed polynomial
    fn compute_round_zero_poly(&self) -> Multilinear<F> {
        unimplemented!("Implement this function")
    }

    /// This function returns poly cimouted in round j
    fn compute_round_j_poly(&self, j: usize) -> Multilinear<F> {
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