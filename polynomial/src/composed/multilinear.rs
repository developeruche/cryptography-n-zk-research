use crate::composed::interfaces::ComposedMultilinearInterface;
use crate::interface::MultilinearPolynomialInterface;
use crate::multilinear::Multilinear;
use ark_ff::PrimeField;

/// This is a composition of multilinear polynomials whose binding operation is multiplication
#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct ComposedMultilinear<F: PrimeField> {
    /// These are all the multilinear polynomials
    pub polys: Vec<Multilinear<F>>,
}

impl<F: PrimeField> ComposedMultilinear<F> {
    /// This is the constructor for the composed multilinear polynomial
    pub fn new(polys: Vec<Multilinear<F>>) -> Self {
        // check to see that all the polynomials have the same number of variables
        let n_vars = polys[0].num_vars();
        assert!(polys.iter().all(|p| p.num_vars() == n_vars));

        ComposedMultilinear { polys }
    }
}

impl<F: PrimeField> MultilinearPolynomialInterface<F> for ComposedMultilinear<F> {
    fn num_vars(&self) -> usize {
        self.polys[0].num_vars()
    }

    fn partial_evaluation(&self, evaluation_point: F, variable_index: usize) -> Self {
        // this would perform partial evaluation on all the composing polynomials
        let mut new_polys = Vec::new();

        for poly in &self.polys {
            new_polys.push(poly.partial_evaluation(evaluation_point, variable_index));
        }

        ComposedMultilinear { polys: new_polys }
    }

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

    fn evaluate(&self, point: &Vec<F>) -> Option<F> {
        let mut result = F::one();

        for poly in &self.polys {
            let eval = poly.evaluate(point);
            match eval {
                Some(val) => result *= val,
                None => return None,
            }
        }

        Some(result)
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
        Self { polys: vec![] }
    }

    fn is_zero(&self) -> bool {
        if self.polys.len() == 0 {
            return true;
        } else {
            if self.polys.iter().all(|p| p.is_zero()) {
                return true;
            } else {
                return false;
            }
        }
    }

    fn internal_add(&self, rhs: &Self) -> Self {
        unimplemented!()
    }

    fn internal_add_assign(&mut self, rhs: &Self) {
        unimplemented!()
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        for poly in &self.polys {
            bytes.extend_from_slice(&poly.to_bytes());
        }

        bytes
    }
}

impl<F: PrimeField> ComposedMultilinearInterface<F> for ComposedMultilinear<F> {
    fn elementwise_product(&self) -> Vec<F> {
        // Find the minimum length of the vectors
        let min_length = match self.polys.get(0) {
            Some(poly) => poly.evaluations.len(),
            None => 0,
        };

        // Perform element-wise product
        (0..min_length)
            .map(|i| self.polys.iter().map(|v| v.evaluations[i]).product())
            .collect()
    }

    fn max_degree(&self) -> usize {
        self.polys.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_test_curves::bls12_381::Fr;

    #[test]
    fn test_evaluation() {
        let poly1 = Multilinear::new(vec![Fr::from(0), Fr::from(1), Fr::from(2), Fr::from(3)], 2);
        let poly2 = Multilinear::new(vec![Fr::from(0), Fr::from(0), Fr::from(0), Fr::from(1)], 2);

        let composed = ComposedMultilinear::new(vec![poly1, poly2]);

        let eval = composed.evaluate(&vec![Fr::from(2), Fr::from(3)]);
        assert_eq!(eval, Some(Fr::from(42)));
    }

    #[test]
    fn test_partial_evaluation() {
        let poly1 = Multilinear::new(vec![Fr::from(0), Fr::from(1), Fr::from(2), Fr::from(3)], 2);
        let poly2 = Multilinear::new(vec![Fr::from(0), Fr::from(0), Fr::from(0), Fr::from(1)], 2);

        let composed = ComposedMultilinear::new(vec![poly1, poly2]);

        let eval = composed.partial_evaluation(Fr::from(2), 0);
        assert_eq!(eval.evaluate(&vec![Fr::from(3)]), Some(Fr::from(42)));
    }

    #[test]
    fn test_partial_evaluations() {
        let poly1 = Multilinear::new(vec![Fr::from(0), Fr::from(1), Fr::from(2), Fr::from(3)], 2);
        let poly2 = Multilinear::new(vec![Fr::from(0), Fr::from(0), Fr::from(0), Fr::from(1)], 2);

        let composed = ComposedMultilinear::new(vec![poly1, poly2]);

        let eval = composed.partial_evaluations(vec![Fr::from(2)], vec![0]);
        assert_eq!(eval.evaluate(&vec![Fr::from(3)]), Some(Fr::from(42)));
    }

    #[test]
    fn test_elementwise_product() {
        let poly1 = Multilinear::new(vec![Fr::from(0), Fr::from(1), Fr::from(2), Fr::from(3)], 2);
        let poly2 = Multilinear::new(vec![Fr::from(0), Fr::from(0), Fr::from(0), Fr::from(1)], 2);

        let composed = ComposedMultilinear::new(vec![poly1, poly2]);

        let eval = composed.elementwise_product();
        assert_eq!(
            eval,
            vec![Fr::from(0), Fr::from(0), Fr::from(0), Fr::from(3)]
        );
    }

    #[test]
    fn test_elementwise_product_2() {
        let poly1 = Multilinear::new(vec![Fr::from(0), Fr::from(1), Fr::from(2), Fr::from(3)], 2);
        let poly2 = Multilinear::new(vec![Fr::from(1), Fr::from(4), Fr::from(0), Fr::from(5)], 2);

        let composed = ComposedMultilinear::new(vec![poly1, poly2]);

        let eval = composed.elementwise_product();
        assert_eq!(
            eval,
            vec![Fr::from(0), Fr::from(4), Fr::from(0), Fr::from(15)]
        );
    }
}
