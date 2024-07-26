use ark_ff::PrimeField;
use polynomial::{
    interface::MultilinearPolynomialInterface, multilinear::Multilinear,
    univariant::UnivariantPolynomial,
};
use sum_check::data_structure::SumCheckProof;

/// This is the W polynomial that is used in the GKR protocol
/// this is how that algebraic equation is represented;
/// W(a, b, c) = add_i(a, b, c) * (w_b(b) + w_c(c)) + mul_i(a, b, c) * (w_b(b) * w_c(c))
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
        add_i: Option<Multilinear<F>>, //  add_i(a,b,c)
        mul_i: Option<Multilinear<F>>, //  mul_i(a,b,c)
        w_b: Option<Multilinear<F>>,   // w_b(b)
        w_c: Option<Multilinear<F>>,   // w_c(c)
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
        let mut eval_structure = self.clone();

        if evaluation_points.len() != variable_indices.len() {
            panic!(
                "The length of evaluation_points and variable_indices should be the same: {}, {}",
                evaluation_points.len(),
                variable_indices.len()
            );
        }

        for i in 0..evaluation_points.len() {
            eval_structure =
                eval_structure.partial_evaluation(evaluation_points[i], variable_indices[i]);
        }

        eval_structure
    }

    // The parameter `Point` in this case is waht is expressed in the text ar `b, c`
    fn evaluate(&self, point: &Vec<F>) -> Option<F> {
        let r_b_c = [self.random_sampling.clone(), point.clone()].concat();

        let w_b_eval = match &self.w_b {
            Some(w_b) => w_b.evaluate(&point[0..w_b.num_vars()].to_vec()),
            None => None,
        }
        .expect("w_b is None");
        let w_c_eval = match &self.w_c {
            Some(w_c) => w_c.evaluate(&point[w_c.num_vars()..].to_vec()),
            None => None,
        }
        .expect("w_c is None");

        let add_i_eval = match &self.add_i {
            Some(add_i) => add_i.evaluate(&r_b_c),
            None => None,
        }
        .expect("add_i is None");
        let mul_i_eval = match &self.mul_i {
            Some(mul_i) => mul_i.evaluate(&r_b_c),
            None => None,
        }
        .expect("mul_i is None");

        let add_section_eval = add_i_eval * (w_b_eval + w_c_eval);
        let mul_section_eval = mul_i_eval * (w_b_eval * w_c_eval);

        Some(add_section_eval + mul_section_eval)
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
        unimplemented!()
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

            let add_i = match (&self.add_i, &rhs.add_i) {
                (Some(add_i), Some(rhs_add_i)) => Some(add_i + rhs_add_i),
                _ => None,
            };

            let mul_i = match (&self.mul_i, &rhs.mul_i) {
                (Some(mul_i), Some(rhs_mul_i)) => Some(mul_i + rhs_mul_i),
                _ => None,
            };

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

#[cfg(test)]
mod tests {
    use super::*;
    use ark_test_curves::bls12_381::Fr;

    #[test]
    fn test_evaluiation() {
        // f(a,b,c) = 2abc + 3b + 4
        let add_i = Multilinear::<Fr>::new(
            vec![
                Fr::from(4),
                Fr::from(4),
                Fr::from(7),
                Fr::from(7),
                Fr::from(4),
                Fr::from(4),
                Fr::from(7),
                Fr::from(9),
            ],
            3,
        );
        // f(b) = 4b
        let w_b = Multilinear::<Fr>::new(vec![Fr::from(0), Fr::from(4)], 1);
        // f(c) = 3c
        let w_c = Multilinear::<Fr>::new(vec![Fr::from(0), Fr::from(3)], 1);
        // f(a,b,c) = 2ab + bc + 3
        let mul_i = Multilinear::<Fr>::new(
            vec![
                Fr::from(3),
                Fr::from(3),
                Fr::from(3),
                Fr::from(4),
                Fr::from(3),
                Fr::from(3),
                Fr::from(5),
                Fr::from(6),
            ],
            3,
        );

        let w = W {
            add_i: Some(add_i),
            mul_i: Some(mul_i),
            w_b: Some(w_b),
            w_c: Some(w_c),
            random_sampling: vec![Fr::from(2u32)],
        };

        let expected_evaulation = Fr::from(1023u32);

        let evaluation = w.evaluate(&[Fr::from(3), Fr::from(1)].to_vec());

        assert_eq!(evaluation, Some(expected_evaulation));
    }
}
