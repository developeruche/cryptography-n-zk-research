use crate::{
    interface::MultilinearPolynomialInterface,
    utils::{
        compute_number_of_variables, multilinear_evalutation_equation, round_pairing_index_ext,
    },
};
use ark_ff::{BigInteger, PrimeField};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use std::ops::{Add, AddAssign};

/// A multilinear polynomial over a field.
#[derive(Clone, PartialEq, Eq, Hash, Default, Debug, CanonicalSerialize, CanonicalDeserialize)]
pub struct Multilinear<F: PrimeField> {
    /// The number of variables in the polynomial.
    pub num_vars: usize,
    /// The evaluations of the polynomial at the different points.
    pub evaluations: Vec<F>,
}

impl<F: PrimeField> Multilinear<F> {
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

    /// This function is used to return the bytes representation of the polynomial
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut m_ploy_bytes = Vec::new();

        for eval in &self.evaluations {
            let big_int = eval.into_bigint().to_bytes_be();
            m_ploy_bytes.extend_from_slice(&big_int);
        }

        m_ploy_bytes
    }
}

impl<F: PrimeField> MultilinearPolynomialInterface<F> for Multilinear<F> {
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

    fn extend_with_new_variables(&self, num_of_new_variables: usize) -> Self {
        let repeat_length = 1 << num_of_new_variables;
        let mut new_evaluations = Vec::new();

        for eval in &self.evaluations {
            for _ in 0..repeat_length {
                new_evaluations.push(*eval);
            }
        }

        Self::new(new_evaluations, self.num_vars + num_of_new_variables)
    }

    fn add_distinct(&self, rhs: &Self) -> Self {
        let mut new_evaluations = Vec::new();
        let repeat_sequence = rhs.evaluations.len();

        for i in 0..self.evaluations.len() {
            for j in 0..repeat_sequence {
                new_evaluations.push(self.evaluations[i] + rhs.evaluations[j]);
            }
        }

        Self::new(new_evaluations, self.num_vars + rhs.num_vars)
    }

    fn mul_distinct(&self, rhs: &Self) -> Self {
        let mut new_evaluations = Vec::new();
        let repeat_sequence = rhs.evaluations.len();

        for i in 0..self.evaluations.len() {
            for j in 0..repeat_sequence {
                new_evaluations.push(self.evaluations[i] * rhs.evaluations[j]);
            }
        }

        Self::new(new_evaluations, self.num_vars + rhs.num_vars)
    }

    fn interpolate(y_s: &[F]) -> Self {
        let number_of_vars = compute_number_of_variables(y_s.len() as u128);
        let mut y_s = y_s.to_vec();
        y_s.resize(1 << number_of_vars as usize, F::ZERO);

        Self::new(y_s, number_of_vars as usize)
    }
}

impl<F: PrimeField> Add for Multilinear<F> {
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

impl<F: PrimeField> AddAssign for Multilinear<F> {
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

    #[test]
    fn test_extend_with_new_variables() {
        // 2xy + 4x + 2y + 5
        let poly = Multilinear::new(vec![Fr::from(5), Fr::from(7), Fr::from(9), Fr::from(13)], 2);
        let new_poly = poly.extend_with_new_variables(1);
        let resulting_evaluations = vec![
            Fr::from(5),
            Fr::from(5),
            Fr::from(7),
            Fr::from(7),
            Fr::from(9),
            Fr::from(9),
            Fr::from(13),
            Fr::from(13),
        ];

        assert_eq!(new_poly.num_vars, 3);
        assert_eq!(new_poly.evaluations, resulting_evaluations);
    }

    #[test]
    fn test_extend_with_new_variables_0() {
        // 2xy + 4x + 2y + 5
        let poly = Multilinear::new(vec![Fr::from(5), Fr::from(7), Fr::from(9), Fr::from(13)], 2);
        let new_poly = poly.extend_with_new_variables(2);

        let resulting_evaluations = vec![
            Fr::from(5),
            Fr::from(5),
            Fr::from(5),
            Fr::from(5),
            Fr::from(7),
            Fr::from(7),
            Fr::from(7),
            Fr::from(7),
            Fr::from(9),
            Fr::from(9),
            Fr::from(9),
            Fr::from(9),
            Fr::from(13),
            Fr::from(13),
            Fr::from(13),
            Fr::from(13),
        ];

        assert_eq!(new_poly.num_vars, 4);
        assert_eq!(new_poly.evaluations, resulting_evaluations);
    }

    #[test]
    fn test_add_distinct() {
        // f(a, b, c) = 2a + 3ab + c + 4
        let poly_1 = Multilinear::new(
            vec![
                Fr::from(4),
                Fr::from(5),
                Fr::from(4),
                Fr::from(5),
                Fr::from(6),
                Fr::from(7),
                Fr::from(9),
                Fr::from(10),
            ],
            3,
        );
        // f(x) = 2x + 9
        let poly_2 = Multilinear::new(vec![Fr::from(9), Fr::from(11)], 1);

        let result = poly_1.add_distinct(&poly_2);

        let resulting_evaluations = vec![
            Fr::from(13),
            Fr::from(15),
            Fr::from(14),
            Fr::from(16),
            Fr::from(13),
            Fr::from(15),
            Fr::from(14),
            Fr::from(16),
            Fr::from(15),
            Fr::from(17),
            Fr::from(16),
            Fr::from(18),
            Fr::from(18),
            Fr::from(20),
            Fr::from(19),
            Fr::from(21),
        ];

        assert_eq!(result.num_vars, 4);
        assert_eq!(result.evaluations, resulting_evaluations);
    }

    #[test]
    fn test_add_distinct_0() {
        // f(a, b, c) = 2a + 3ab + c + 4

        // f(x,y) = 4xy + 3x + 4y + 3
    }

    #[test]
    fn test_mul_distinct() {
        // f(a, b, c) = 2a + 3ab + c + 4
        let poly_1 = Multilinear::new(
            vec![
                Fr::from(4),
                Fr::from(5),
                Fr::from(4),
                Fr::from(5),
                Fr::from(6),
                Fr::from(7),
                Fr::from(9),
                Fr::from(10),
            ],
            3,
        );
        // f(x) = 2x + 9
        let poly_2 = Multilinear::new(vec![Fr::from(9), Fr::from(11)], 1);

        let result = poly_1.mul_distinct(&poly_2);

        let resulting_evaluations = vec![
            Fr::from(36),
            Fr::from(44),
            Fr::from(45),
            Fr::from(55),
            Fr::from(36),
            Fr::from(44),
            Fr::from(45),
            Fr::from(55),
            Fr::from(54),
            Fr::from(66),
            Fr::from(63),
            Fr::from(77),
            Fr::from(81),
            Fr::from(99),
            Fr::from(90),
            Fr::from(110),
        ];

        assert_eq!(result.num_vars, 4);
        assert_eq!(result.evaluations, resulting_evaluations);
    }

    #[test]
    fn test_interpolate() {
        let y_s = vec![Fr::from(1), Fr::from(2), Fr::from(3)];
        let result = Multilinear::<Fr>::interpolate(&y_s);

        let expected =
            Multilinear::new(vec![Fr::from(1), Fr::from(2), Fr::from(3), Fr::from(0)], 2);
        
        
        assert_eq!(expected, result);
        
        
        assert_eq!(result.evaluate(&vec![Fr::from(0), Fr::from(0)]).unwrap(), Fr::from(1));
        assert_eq!(result.evaluate(&vec![Fr::from(0), Fr::from(1)]).unwrap(), Fr::from(2));
        assert_eq!(result.evaluate(&vec![Fr::from(1), Fr::from(0)]).unwrap(), Fr::from(3));
        assert_eq!(result.evaluate(&vec![Fr::from(1), Fr::from(1)]).unwrap(), Fr::from(0));
    }

    #[test]
    fn test_interpolate_1() {
        let y_s = vec![
            Fr::from(2),
            Fr::from(4),
            Fr::from(6),
            Fr::from(8),
            Fr::from(10),
        ];
        let result = Multilinear::<Fr>::interpolate(&y_s);

        let expected = Multilinear::new(
            vec![
                Fr::from(2),
                Fr::from(4),
                Fr::from(6),
                Fr::from(8),
                Fr::from(10),
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
            ],
            3,
        );

        assert_eq!(expected, result);
        
        
        assert_eq!(result.evaluate(&vec![Fr::from(0), Fr::from(0), Fr::from(0)]).unwrap(), Fr::from(2));
        assert_eq!(result.evaluate(&vec![Fr::from(0), Fr::from(0), Fr::from(1)]).unwrap(), Fr::from(4));
        assert_eq!(result.evaluate(&vec![Fr::from(0), Fr::from(1), Fr::from(0)]).unwrap(), Fr::from(6));
        assert_eq!(result.evaluate(&vec![Fr::from(0), Fr::from(1), Fr::from(1)]).unwrap(), Fr::from(8));
        assert_eq!(result.evaluate(&vec![Fr::from(1), Fr::from(0), Fr::from(0)]).unwrap(), Fr::from(10));
        assert_eq!(result.evaluate(&vec![Fr::from(1), Fr::from(0), Fr::from(1)]).unwrap(), Fr::from(0));
        assert_eq!(result.evaluate(&vec![Fr::from(1), Fr::from(1), Fr::from(0)]).unwrap(), Fr::from(0));
        assert_eq!(result.evaluate(&vec![Fr::from(1), Fr::from(1), Fr::from(1)]).unwrap(), Fr::from(0));
    }
}
