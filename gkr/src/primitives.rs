use ark_ff::PrimeField;
use polynomial::{
    interface::MultilinearPolynomialInterface, multilinear::Multilinear,
    univariant::UnivariantPolynomial,
};
use sum_check::data_structure::SumCheckProof;

#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct W<F: PrimeField> {
    /// This is the addition multilinear extension
    add_i: Option<Multilinear<F>>,
    /// This is the multiplication multilinear extension
    mul_i: Option<Multilinear<F>>,
    /// This is the w_b equation
    w_b: Option<Multilinear<F>>,
    /// This is the w_c equation
    w_c: Option<Multilinear<F>>,
    /// this is a vector of all random sampling
    random_sampling: Vec<F>,
}

#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct GKRProof<F: PrimeField, P: MultilinearPolynomialInterface<F>> {
    /// This is the output of the Circuit evaluation
    pub output: Vec<F>,
    /// This is the list of sum check proofs gotten during this protocol
    pub sum_check_proofs: Vec<SumCheckProof<F, P>>,
    /// This is the list of q polynomials
    pub q_polynomials: Vec<UnivariantPolynomial<F>>,
}

impl<F: PrimeField> W<F> {
    pub fn new(
        add_i: Option<Multilinear<F>>,
        mul_i: Option<Multilinear<F>>,
        w_b: Option<Multilinear<F>>,
        w_c: Option<Multilinear<F>>,
        r: Vec<F>,
    ) -> Self {
        W {
            add_i,
            mul_i,
            w_b,
            w_c,
            random_sampling: r,
        }
    }
}

impl<F: PrimeField> MultilinearPolynomialInterface<F> for W<F> {
    fn num_vars(&self) -> usize {
        todo!()
    }

    fn partial_evaluation(&self, evaluation_point: F, variable_index: usize) -> Self {
        todo!()
    }

    fn partial_evaluations(&self, evaluation_points: Vec<F>, variable_indices: Vec<usize>) -> Self {
        todo!()
    }

    fn evaluate(&self, point: &Vec<F>) -> Option<F> {
        todo!()
    }

    fn extend_with_new_variables(&self, num_of_new_variables: usize) -> Self {
        todo!()
    }

    fn add_distinct(&self, rhs: &Self) -> Self {
        todo!()
    }

    fn mul_distinct(&self, rhs: &Self) -> Self {
        todo!()
    }

    fn interpolate(y_s: &[F]) -> Self {
        todo!()
    }

    fn zero(num_vars: usize) -> Self {
        todo!()
    }

    fn is_zero(&self) -> bool {
        todo!()
    }

    fn internal_add(&self, rhs: &Self) -> Self {
        todo!()
    }

    fn internal_add_assign(&mut self, rhs: &Self) {
        todo!()
    }

    fn to_bytes(&self) -> Vec<u8> {
        todo!()
    }
}
