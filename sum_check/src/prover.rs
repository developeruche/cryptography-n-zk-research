use crate::{data_structure::SumCheckProof, interface::ProverInterface};
use ark_ff::PrimeField;
use fiat_shamir::{interface::TranscriptInterface, FiatShamirTranscript};
use polynomial::{
    interface::MultilinearPolynomialInterface, multilinear::Multilinear, utils::boolean_hypercube,
};

#[derive(Clone, Default, Debug)]
pub struct Prover<F: PrimeField> {
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

impl<F: PrimeField> Prover<F> {
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

    /// This function crates a new prover instance with sum already computed
    pub fn new_with_sum(poly: Multilinear<F>, sum: F) -> Self {
        Self {
            poly,
            round_poly: Default::default(),
            round_0_poly: Default::default(),
            sum,
            transcript: Default::default(),
        }
    }

    /// This function creates a new  prover intance, computes the sum and computes the round zero polynomial
    pub fn new_with_sum_and_round_zero(poly: Multilinear<F>) -> Self {
        let mut prover = Self::new(poly);
        prover.calculate_sum();
        prover.compute_round_zero_poly();
        prover
    }
}

impl<F: PrimeField> ProverInterface<F> for Prover<F> {
    /// This function returns the sum of the multilinear polynomial evaluation over the boolean hypercube.
    fn calculate_sum(&mut self) {
        self.sum = self.poly.evaluations.iter().sum();
    }

    /// This function returns the round zero computed polynomial
    fn compute_round_zero_poly(&mut self) {
        let number_of_round = self.poly.num_vars - 1;
        let bh = boolean_hypercube(number_of_round);
        let mut bh_partials: Multilinear<F> = Multilinear::zero(1); // this is an accumulator

        for bh_i in bh {
            let current_partial = self
                .poly
                .partial_evaluations(bh_i, vec![1; number_of_round]);
            bh_partials += current_partial;
        }

        self.transcript.append(bh_partials.to_bytes());
        self.round_0_poly = bh_partials;
    }

    /// This function computes sum check proof
    fn sum_check_proof(&mut self) -> SumCheckProof<F> {
        self.compute_round_zero_poly();
        let mut all_random_reponse = Vec::new();

        for i in 1..self.poly.num_vars {
            let number_of_round = self.poly.num_vars - i - 1;
            let bh = boolean_hypercube::<F>(number_of_round);

            let mut bh_partials: Multilinear<F> = Multilinear::zero(1);
            let verifier_random_reponse_f = F::from_be_bytes_mod_order(&self.transcript.sample());
            all_random_reponse.push(verifier_random_reponse_f);

            for bh_i in bh {
                let bh_len = bh_i.len();
                let mut eval_vector = all_random_reponse.clone();
                eval_vector.extend(bh_i);
                let mut eval_index = vec![0; all_random_reponse.len()];
                let suffix_eval_index = vec![1; bh_len];
                eval_index.extend(suffix_eval_index);

                let current_partial = self.poly.partial_evaluations(eval_vector, eval_index);

                bh_partials += current_partial;
            }

            self.transcript.append(bh_partials.to_bytes());
            self.round_poly.push(bh_partials);
        }

        SumCheckProof {
            polynomial: self.poly.clone(),
            round_poly: self.round_poly.clone(),
            round_0_poly: self.round_0_poly.clone(),
            sum: self.sum,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{interface::VerifierInterface, verifier::Verifier};

    use super::*;
    use ark_test_curves::bls12_381::Fr;

    #[test]
    fn test_sum_calculation() {
        let poly = Multilinear::new(
            vec![
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(2),
                Fr::from(2),
                Fr::from(2),
                Fr::from(2),
                Fr::from(4),
            ],
            3,
        );
        let mut prover = Prover::new(poly);
        prover.calculate_sum();
        assert_eq!(prover.sum, Fr::from(12));
    }

    #[test]
    fn test_compute_round_zero_poly() {
        let poly = Multilinear::new(
            vec![
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(2),
                Fr::from(2),
                Fr::from(2),
                Fr::from(2),
                Fr::from(4),
            ],
            3,
        );
        let mut prover = Prover::new(poly);
        prover.compute_round_zero_poly();
        assert_eq!(
            prover.round_0_poly.evaluations,
            vec![Fr::from(2), Fr::from(10)]
        );
    }

    #[test]
    fn test_compute_round_zero_poly_2() {
        let poly = Multilinear::new(
            vec![
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(2),
                Fr::from(2),
                Fr::from(2),
                Fr::from(2),
                Fr::from(4),
            ],
            3,
        );
        let mut prover = Prover::new(poly);
        prover.compute_round_zero_poly();
        let sum = prover.round_0_poly.evaluate(&vec![Fr::from(1)]).unwrap()
            + prover.round_0_poly.evaluate(&vec![Fr::from(0)]).unwrap();
        assert_eq!(sum, Fr::from(12));
    }

    #[test]
    fn test_compute_round_zero_poly_3() {
        let poly = Multilinear::new(
            vec![
                Fr::from(0),
                Fr::from(0),
                Fr::from(2),
                Fr::from(7),
                Fr::from(3),
                Fr::from(3),
                Fr::from(5),
                Fr::from(11),
            ],
            3,
        );
        let mut prover = Prover::new(poly);
        prover.compute_round_zero_poly();
        assert_eq!(
            prover.round_0_poly.evaluations,
            vec![Fr::from(9), Fr::from(22)]
        );
    }

    #[test]
    fn test_compute_round_zero_poly_4() {
        let poly = Multilinear::new(
            vec![
                Fr::from(0),
                Fr::from(0),
                Fr::from(2),
                Fr::from(7),
                Fr::from(3),
                Fr::from(3),
                Fr::from(5),
                Fr::from(11),
            ],
            3,
        );
        let mut prover = Prover::new(poly);
        prover.compute_round_zero_poly();
        let sum = prover.round_0_poly.evaluate(&vec![Fr::from(1)]).unwrap()
            + prover.round_0_poly.evaluate(&vec![Fr::from(0)]).unwrap();
        assert_eq!(sum, Fr::from(31));
    }

    #[test]
    fn test_compute_round_zero_poly_5() {
        let poly = Multilinear::new(
            vec![
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(1),
                Fr::from(1),
                Fr::from(1),
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
            ],
            4,
        );
        let mut prover = Prover::new(poly);
        prover.compute_round_zero_poly();
        let sum = prover.round_0_poly.evaluate(&vec![Fr::from(1)]).unwrap()
            + prover.round_0_poly.evaluate(&vec![Fr::from(0)]).unwrap();
        assert_eq!(sum, Fr::from(3));
    }

    #[test]
    #[ignore] // un-disable this test when all random response is F::from(3)
    fn test_sum_check_proof_ignored() {
        let poly = Multilinear::new(
            vec![
                Fr::from(0),
                Fr::from(0),
                Fr::from(2),
                Fr::from(7),
                Fr::from(3),
                Fr::from(3),
                Fr::from(6),
                Fr::from(11),
            ],
            3,
        );
        let mut prover = Prover::new(poly);
        prover.calculate_sum();
        let proof = prover.sum_check_proof();

        let sum = prover.round_0_poly.evaluate(&vec![Fr::from(1)]).unwrap()
            + prover.round_0_poly.evaluate(&vec![Fr::from(0)]).unwrap();
        assert_eq!(sum, Fr::from(32));

        let test_eval_round_0 = proof.round_0_poly.evaluate(&vec![Fr::from(3)]).unwrap();
        let test_eval_round_1 = prover.round_poly[0].evaluate(&vec![Fr::from(1)]).unwrap()
            + prover.round_poly[0].evaluate(&vec![Fr::from(0)]).unwrap();

        assert_eq!(test_eval_round_0, test_eval_round_1);

        for i in 1..proof.round_poly.len() {
            let sum = proof.round_poly[i].evaluate(&vec![Fr::from(1)]).unwrap()
                + proof.round_poly[i].evaluate(&vec![Fr::from(0)]).unwrap();
            let pre_eval = proof.round_poly[i - 1]
                .evaluate(&vec![Fr::from(3)])
                .unwrap();

            println!("{:?} - {:?}", sum, pre_eval);

            assert_eq!(sum, pre_eval);
        }
    }

    #[test]
    fn test_sum_check_proof() {
        let poly = Multilinear::new(
            vec![
                Fr::from(0),
                Fr::from(0),
                Fr::from(2),
                Fr::from(7),
                Fr::from(3),
                Fr::from(3),
                Fr::from(6),
                Fr::from(11),
            ],
            3,
        );
        let mut prover = Prover::new(poly);
        prover.calculate_sum();
        let proof = prover.sum_check_proof();
        let mut verifer = Verifier::new();

        assert!(verifer.verify(&proof));
    }

    #[test]
    fn test_sum_check_proof_2() {
        let poly = Multilinear::new(
            vec![
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(1),
                Fr::from(1),
                Fr::from(1),
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
            ],
            4,
        );
        let mut prover = Prover::new(poly);
        prover.calculate_sum();
        let proof = prover.sum_check_proof();
        let mut verifer = Verifier::new();

        assert!(verifer.verify(&proof));
    }

    #[test]
    fn test_sum_check_proof_3() {
        let poly = Multilinear::new(
            vec![
                Fr::from(1),
                Fr::from(3),
                Fr::from(5),
                Fr::from(7),
                Fr::from(2),
                Fr::from(4),
                Fr::from(6),
                Fr::from(8),
                Fr::from(3),
                Fr::from(5),
                Fr::from(7),
                Fr::from(9),
                Fr::from(4),
                Fr::from(6),
                Fr::from(8),
                Fr::from(10),
            ],
            4,
        );
        let mut prover = Prover::new(poly);
        prover.calculate_sum();

        println!("Sum: {:?}", prover.sum);

        let proof = prover.sum_check_proof();
        let mut verifer = Verifier::new();

        assert!(verifer.verify(&proof));
    }
}
