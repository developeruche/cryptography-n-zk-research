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
        let n_b_vars = match &self.w_b {
            Some(w_b) => w_b.num_vars(),
            None => 0,
        };
        let n_c_vars = match &self.w_c {
            Some(w_c) => w_c.num_vars(),
            None => 0,
        };

        n_b_vars + n_c_vars
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
        unimplemented!()
    }

    fn add_distinct(&self, rhs: &Self) -> Self {
        unimplemented!()
    }

    fn mul_distinct(&self, rhs: &Self) -> Self {
        unimplemented!()
    }

    fn interpolate(y_s: &[F]) -> Self {
        todo!()
    }

    fn zero(num_vars: usize) -> Self {
        Self {
            add_i: None,
            mul_i: None,
            w_b: None,
            w_c: None,
            random_sampling: vec![],
        }
    }

    fn is_zero(&self) -> bool {
        if self.add_i.is_none() && self.mul_i.is_none() && self.w_b.is_none() && self.w_c.is_none()
        {
            return true;
        } else {
            return false;
        }
    }

    fn internal_add(&self, rhs: &Self) -> Self {
        if self.is_zero() {
            return rhs.clone();
        } else if rhs.is_zero() {
            return self.clone();
        } else {
            let random_sampling = self.random_sampling.clone();

            todo!()
            // return W {
            //     add_i,
            //     mul_i,
            //     w_b,
            //     w_c,
            //     random_sampling,
            // };
        }
    }

    fn internal_add_assign(&mut self, rhs: &Self) {
        if self.is_zero() {
            *self = rhs.clone();
        } else if rhs.is_zero() {
            return;
        } else {
            let random_sampling = self.random_sampling.clone();
            todo!()
            // *self = W {
            //     add_i,
            //     mul_i,
            //     w_b,
            //     w_c,
            //     random_sampling,
            // };
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        match &self.add_i {
            Some(add_i) => {
                bytes.extend_from_slice(&add_i.to_bytes());
            }
            None => {
                bytes.extend_from_slice(&[0u8; 32]);
            }
        }

        match &self.mul_i {
            Some(mul_i) => {
                bytes.extend_from_slice(&mul_i.to_bytes());
            }
            None => {
                bytes.extend_from_slice(&[0u8; 32]);
            }
        }

        match &self.w_b {
            Some(w_b) => {
                bytes.extend_from_slice(&w_b.to_bytes());
            }
            None => {
                bytes.extend_from_slice(&[0u8; 32]);
            }
        }

        match &self.w_c {
            Some(w_c) => {
                bytes.extend_from_slice(&w_c.to_bytes());
            }
            None => {
                bytes.extend_from_slice(&[0u8; 32]);
            }
        }

        bytes
    }
}
