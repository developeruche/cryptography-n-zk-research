//! Utility functions for Libra.
use p3_field::{ExtensionField, Field};

pub(crate) fn generate_igz<F: Field, E: ExtensionField<F>>(points: &[E]) -> Vec<E> {
    let mut res = vec![E::one()];

    for point in points {
        let mut v = vec![];
        for val in &res {
            v.push(*val * (E::one() - *point));
            v.push(*val * *point);
        }
        res = v;
    }

    res
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::*;
    use p3_field::{AbstractExtensionField, AbstractField, extension::BinomialExtensionField};
    use p3_mersenne_31::Mersenne31;
    use poly::{Fields, MultilinearExtension, mle::MultilinearPoly, vpoly::VPoly};

    type F = Mersenne31;
    type E = BinomialExtensionField<Mersenne31, 3>;

    fn extend_with_new_variables(evals: &Vec<E>, num_of_new_variables: usize) -> Vec<E> {
        let repeat_length = 1 << num_of_new_variables;
        let mut new_evaluations = Vec::new();

        for eval in evals {
            for _ in 0..repeat_length {
                new_evaluations.push(*eval);
            }
        }

        new_evaluations
    }

    fn combined_fn(values: &[Fields<F, E>]) -> Fields<F, E> {
        Fields::Extension(values[0].to_extension_field() * values[1].to_extension_field())
    }

    #[test]
    fn test_generate_igz() {
        let evals = vec![1u32; 64];
        // f(z,x,y) where z,x,y are of size 2
        let var_6_poly = MultilinearPoly::<F, E>::new_from_vec(
            6,
            evals
                .iter()
                .map(|&x| Fields::from_u32(x))
                .collect::<Vec<_>>(),
        );

        let points = vec![E::from_wrapped_u32(2), E::from_wrapped_u32(3)];
        let expected = vec![
            E::from_wrapped_u32(2),
            -E::from_wrapped_u32(3),
            -E::from_wrapped_u32(4),
            E::from_wrapped_u32(6),
        ];

        // I_g(z)
        let igz_table = generate_igz::<F, E>(&points);

        // assert len = 4
        assert_eq!(igz_table.len(), 4);

        // assert values
        assert_eq!(igz_table, expected);

        // IGZ blown up
        let igz_6_var = extend_with_new_variables(&igz_table, 4);
        // I_g(z,x,y)
        let igz_6_var_mle = MultilinearPoly::<F, E>::new_from_vec(
            6,
            igz_6_var
                .iter()
                .map(|&x| Fields::Extension(x))
                .collect::<Vec<_>>(),
        );

        // igz * mle
        // I_g(z,x,y) * f(z, x,y)
        let v_poly = VPoly::new(
            vec![igz_6_var_mle, var_6_poly.clone()],
            2,
            6,
            Rc::new(combined_fn),
        );

        let sum_1 = v_poly.partial_evaluate(
            &[0, 0]
                .iter()
                .map(|&x| Fields::from_u32(x))
                .collect::<Vec<_>>(),
        );
        let sum_2 = v_poly.partial_evaluate(
            &[0, 1]
                .iter()
                .map(|&x| Fields::from_u32(x))
                .collect::<Vec<_>>(),
        );
        let sum_3 = v_poly.partial_evaluate(
            &[1, 0]
                .iter()
                .map(|&x| Fields::from_u32(x))
                .collect::<Vec<_>>(),
        );
        let sum_4 = v_poly.partial_evaluate(
            &[1, 1]
                .iter()
                .map(|&x| Fields::from_u32(x))
                .collect::<Vec<_>>(),
        );

        let eval_points = [2, 3, 4, 5]
            .iter()
            .map(|&x| Fields::from_u32(x))
            .collect::<Vec<_>>();

        // ∑_{z∈{0,1}^l} I_g(z,x,y) * f(z, x,y); where l = 2
        let expected = sum_1.evaluate(&eval_points)
            + sum_2.evaluate(&eval_points)
            + sum_3.evaluate(&eval_points)
            + sum_4.evaluate(&eval_points);

        let actual_complete_points = [2, 3, 2, 3, 4, 5]
            .iter()
            .map(|&x| Fields::from_u32(x))
            .collect::<Vec<_>>();

        // f(g,x,y)
        let actual = var_6_poly.evaluate(&actual_complete_points);

        assert_eq!(actual, expected);
    }
}
