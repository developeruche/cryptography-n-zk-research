use ark_ff::Field;
use std::{
    iter::Sum,
    ops::{Add, AddAssign},
};

use crate::{
    interface::MultivariantPolynomialInterface,
    utils::{multilinear_evalutation_equation, round_pairing_index_ext},
};

/// A multilinear polynomial over a field.
#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct Multilinear<F> {
    /// The number of variables in the polynomial.
    pub num_vars: usize,
    /// The evaluations of the polynomial at the different points.
    pub evaluations: Vec<F>,
}

impl<F: Field> Multilinear<F> {
    /// This function creates a new multilinear polynomial from a list of evaluations
    pub fn new(evaluations: Vec<F>, num_vars: usize) -> Self {
        // SANITY_CHECK: Ensure that the number of evaluations is equal to the number of variables raised to power of 2
        assert_eq!(
            evaluations.len(),
            1 << num_vars,
            "Number of evaluations must be equal to 2^num_vars"
        );
        Self {
            num_vars,
            evaluations,
        }
    }

    /// This function returns the additive identity of the polynomial
    pub fn zero(num_vars: usize) -> Self {
        Self::new(vec![F::zero(); 1 << num_vars], num_vars)
    }

    /// This function returns the additive identity of this polynomial (self)
    pub fn self_zero(&self) -> Self {
        Self::zero(self.num_vars)
    }

    /// This function is used to check if the polynomial is zero
    pub fn is_zero(&self) -> bool {
        self.evaluations.iter().all(|x| x.is_zero())
    }
}

impl<F: Field> MultivariantPolynomialInterface<F> for Multilinear<F> {
    /// This function returns the number of variables in the polynomial
    fn num_vars(&self) -> usize {
        self.num_vars
    }

    /// This function creates a new polynomial from a list of evaluations
    fn partial_evaluation(&self, evaluation_point: F, variable_index: usize) -> Self {
        let round_pairing_indices = round_pairing_index_ext(self.evaluations.len(), variable_index);

        let mut new_evaluations = Vec::new();
        for round_pair in round_pairing_indices {
            let y_1 = self.evaluations[round_pair.0];
            let y_2 = self.evaluations[round_pair.1];
            let new_y = multilinear_evalutation_equation(evaluation_point, y_1, y_2);
            new_evaluations.push(new_y);
        }

        Self::new(new_evaluations, self.num_vars - 1)
    }

    /// This function allow for multiple partial evaluations
    fn partial_evaluations(&self, evaluation_points: Vec<F>, variable_indices: Vec<usize>) -> Self {
        let mut eval_polynomial = self.clone();

        if evaluation_points.len() != variable_indices.len() {
            panic!(
                "The length of evaluation_points and variable_indices should be the same: {}, {}",
                evaluation_points.len(),
                variable_indices.len()
            );
        }

        for i in 0..evaluation_points.len() {
            eval_polynomial =
                eval_polynomial.partial_evaluation(evaluation_points[i], variable_indices[i]);
        }

        eval_polynomial
    }

    /// Evaluates `self` at the given `point` in `Self::Point`. this is done using partial evaluations
    fn evaluate(&self, point: &Vec<F>) -> Option<F> {
        let mut eval_result = None;
        let mut eval_polynomial = self.clone();

        for i in 0..point.len() {
            eval_polynomial = eval_polynomial.partial_evaluation(point[i], 0);
            eval_result = Some(eval_polynomial.evaluations[0]);
        }

        eval_result
    }
}

impl<F: Field> Add for Multilinear<F> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut new_evaluations = Vec::new();
        // TODO: come up with an algo for handling the case where the number of variables in the two polynomials are not the same
        if self.num_vars != other.num_vars {
            panic!("The number of variables in the two polynomials must be the same");
        }

        for i in 0..self.evaluations.len() {
            new_evaluations.push(self.evaluations[i] + other.evaluations[i]);
        }

        Self::new(new_evaluations, self.num_vars)
    }
}

impl<F: Field> AddAssign for Multilinear<F> {
    fn add_assign(&mut self, other: Self) {
        // TODO: come up with an algo for handling the case where the number of variables in the two polynomials are not the same
        if self.num_vars != other.num_vars {
            panic!("The number of variables in the two polynomials must be the same");
        }

        for i in 0..self.evaluations.len() {
            self.evaluations[i] += other.evaluations[i];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_test_curves::bls12_381::Fr;

    #[test]
    fn test_partial_evaluation() {
        let evaluations = vec![Fr::from(3), Fr::from(1), Fr::from(2), Fr::from(5)];
        let num_vars = 2;
        let polynomial = Multilinear::new(evaluations, num_vars);

        let evaluation_point = Fr::from(5);
        let new_polynomial = polynomial.partial_evaluation(evaluation_point, 0);

        let expected_evaluations = vec![Fr::from(-2), Fr::from(21)];
        assert_eq!(new_polynomial.evaluations, expected_evaluations);
        assert_eq!(new_polynomial.num_vars, 1);
    }

    #[test]
    fn test_evaluate() {
        let evaluations = vec![Fr::from(3), Fr::from(1), Fr::from(2), Fr::from(5)];
        let num_vars = 2;
        let polynomial = Multilinear::new(evaluations, num_vars);

        let point = vec![Fr::from(5), Fr::from(6)];
        let eval_result = polynomial.evaluate(&point);

        assert_eq!(eval_result, Some(Fr::from(136)));
    }

    #[test]
    fn test_partial_evaluation_2() {
        let evaluations = vec![
            Fr::from(3),
            Fr::from(9),
            Fr::from(7),
            Fr::from(13),
            Fr::from(6),
            Fr::from(12),
            Fr::from(10),
            Fr::from(18),
        ];
        let num_vars = 3;
        // 2xyz + 3x + 4y + 6z + 3
        let polynomial = Multilinear::new(evaluations, num_vars);

        let point = vec![Fr::from(2), Fr::from(3), Fr::from(1)];
        let eval_result = polynomial.evaluate(&point);

        assert_eq!(eval_result, Some(Fr::from(39)));

        // testing partial evaluations with multiple points
        let new_polynomial_eval = polynomial
            .partial_evaluations(vec![Fr::from(2), Fr::from(3), Fr::from(1)], vec![0, 0, 0]);
        let x_eval_result = new_polynomial_eval.evaluations[0];
        assert_eq!(x_eval_result, Fr::from(39));

        // obtain: f(2,y,z) = 4yz + 4y + 6z + 9 at y = 3, z = 2 = 57
        let new_polynomial_x_1 = polynomial.partial_evaluation(Fr::from(2), 0);
        // 4yz + 4y + 6z + 9
        let x_1_eval_result = new_polynomial_x_1.evaluate(&vec![Fr::from(3), Fr::from(2)]);
        assert_eq!(x_1_eval_result, Some(Fr::from(57)));

        // obtain: f(x,3,z) = 6xz + 3x + 6z + 15 at y = 3, z = 2 = 72
        let new_polynomial_y_1 = polynomial.partial_evaluation(Fr::from(3), 1);
        // 6xz + 3x + 6z + 15
        let y_1_eval_result = new_polynomial_y_1.evaluate(&vec![Fr::from(3), Fr::from(2)]);
        assert_eq!(y_1_eval_result, Some(Fr::from(72)));

        // obtain: f(x,y,1) = 2xy + 3x + 4y + 9  at y = 3, z = 2 = 38
        let new_polynomial_z_1 = polynomial.partial_evaluation(Fr::from(1), 2);
        // 2xy + 3x + 4y + 9
        let z_1_eval_result = new_polynomial_z_1.evaluate(&vec![Fr::from(3), Fr::from(2)]);
        assert_eq!(z_1_eval_result, Some(Fr::from(38)));

        // obtain: f(2,3,z) = 18z + 21  at y = 3,  = 75
        let new_polynomial_x_y =
            polynomial.partial_evaluations(vec![Fr::from(2), Fr::from(3)], vec![0, 0]);
        // 18z + 21
        let x_y_eval_result = new_polynomial_x_y.evaluate(&vec![Fr::from(3)]);
        assert_eq!(x_y_eval_result, Some(Fr::from(75)));

        // obtain: f(2,y,1) = 8y + 15  at y = 3, = 39
        let new_polynomial_x_z =
            polynomial.partial_evaluations(vec![Fr::from(2), Fr::from(1)], vec![0, 1]);
        // 8y + 15
        let x_z_eval_result = new_polynomial_x_z.evaluate(&vec![Fr::from(3)]);
        assert_eq!(x_z_eval_result, Some(Fr::from(39)));

        // obtain: f(x,3,1) = 9x + 21  at y = 3, = 48
        let new_polynomial_y_z =
            polynomial.partial_evaluations(vec![Fr::from(3), Fr::from(1)], vec![1, 1]);
        // 9x + 21
        let y_z_eval_result = new_polynomial_y_z.evaluate(&vec![Fr::from(3)]);
        assert_eq!(y_z_eval_result, Some(Fr::from(48)));
    }
}
