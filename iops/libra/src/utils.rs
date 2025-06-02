//! Utility functions for Libra.
use p3_field::{ExtensionField, Field};
use poly::Fields;

pub fn generate_igz<F: Field, E: ExtensionField<F>>(points: &[E]) -> Vec<E> {
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

pub fn product_combined_fn<F: Field, E: ExtensionField<F>>(
    values: &[Fields<F, E>],
) -> Fields<F, E> {
    Fields::Extension(values[0].to_extension_field() * values[1].to_extension_field())
}

pub fn merge_sumcheck_proofs<T>(vecs: Vec<Vec<T>>) -> Option<Vec<T>>
where
    T: std::ops::Add<Output = T> + Copy,
{
    vecs.into_iter().reduce(|acc, v| {
        acc.into_iter()
            .zip(v.into_iter())
            .map(|(a, b)| a + b)
            .collect()
    })
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::*;
    use p3_field::{AbstractField, extension::BinomialExtensionField};
    use p3_mersenne_31::Mersenne31;
    use poly::{Fields, MultilinearExtension, mle::MultilinearPoly, vpoly::VPoly};

    type F = Mersenne31;
    type E = BinomialExtensionField<Mersenne31, 3>;

    #[test]
    fn test_merge_sumcheck_proofs() {
        // Test with u32 vectors
        let vec1 = vec![1u32, 2, 3];
        let vec2 = vec![4u32, 5, 6];
        let vec3 = vec![7u32, 8, 9];

        let vecs = vec![vec1, vec2, vec3];
        let merged = merge_sumcheck_proofs(vecs).unwrap();

        assert_eq!(merged, vec![12, 15, 18]);

        // Test with empty vector list
        let empty_vecs: Vec<Vec<u32>> = vec![];
        assert!(merge_sumcheck_proofs(empty_vecs).is_none());

        // Test with single vector
        let single_vec = vec![vec![10, 20, 30]];
        let merged_single = merge_sumcheck_proofs(single_vec).unwrap();
        assert_eq!(merged_single, vec![10, 20, 30]);

        // Test with Fields<F, E>
        let f_vec1 = vec![
            Fields::<F, E>::Base(F::new(1)),
            Fields::<F, E>::Base(F::new(2)),
        ];

        let f_vec2 = vec![
            Fields::<F, E>::Base(F::new(3)),
            Fields::<F, E>::Base(F::new(4)),
        ];

        let f_vecs = vec![f_vec1, f_vec2];
        let f_merged = merge_sumcheck_proofs(f_vecs).unwrap();

        assert_eq!(f_merged[0], Fields::<F, E>::Base(F::new(4))); // 1 + 3 = 4
        assert_eq!(f_merged[1], Fields::<F, E>::Base(F::new(6))); // 2 + 4 = 6

        // Test with Vec<Vec<Fields<F, E>>> (similar to sumcheck proofs)
        let inner1 = vec![
            Fields::<F, E>::Base(F::new(1)),
            Fields::<F, E>::Base(F::new(2)),
        ];

        let inner2 = vec![
            Fields::<F, E>::Base(F::new(3)),
            Fields::<F, E>::Base(F::new(4)),
        ];

        let proof = vec![inner1.clone(), inner2.clone()];

        let merged_proofs = merge_sumcheck_proofs::<Fields<F, E>>(proof).unwrap();

        assert_eq!(merged_proofs.len(), 2);
        assert_eq!(merged_proofs[0], Fields::<F, E>::Base(F::new(4))); // 1 + 3 = 4
        assert_eq!(merged_proofs[1], Fields::<F, E>::Base(F::new(6))); // 2 + 4 = 6
    }

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
            Rc::new(product_combined_fn),
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

        // Identity check;
        // I(w, z) = 1; iff w = z for all w, z ∈ {0,1}^l

        // when w = [0, 0]
        let w_1_point = vec![E::from_wrapped_u32(0), E::from_wrapped_u32(0)];
        let i_wz_1 = MultilinearPoly::new_from_vec(
            2,
            (generate_igz::<F, E>(&w_1_point).iter())
                .map(|&x| Fields::<F, E>::Extension(x))
                .collect(),
        );
        assert_eq!(
            i_wz_1.evaluate(
                &w_1_point
                    .iter()
                    .map(|&x| Fields::<F, E>::Extension(x))
                    .collect::<Vec<_>>()
            ),
            Fields::<F, E>::Extension(E::one())
        );

        // when w = [1, 1]
        let w_2_point = vec![E::from_wrapped_u32(1), E::from_wrapped_u32(1)];
        let i_wz_2 = MultilinearPoly::new_from_vec(
            2,
            (generate_igz::<F, E>(&w_2_point).iter())
                .map(|&x| Fields::<F, E>::Extension(x))
                .collect(),
        );
        assert_eq!(
            i_wz_2.evaluate(
                &w_2_point
                    .iter()
                    .map(|&x| Fields::<F, E>::Extension(x))
                    .collect::<Vec<_>>()
            ),
            Fields::<F, E>::Extension(E::one())
        );

        // when w = [0, 1]
        let w_3_point = vec![E::from_wrapped_u32(0), E::from_wrapped_u32(1)];
        let i_wz_3 = MultilinearPoly::new_from_vec(
            2,
            (generate_igz::<F, E>(&w_3_point).iter())
                .map(|&x| Fields::<F, E>::Extension(x))
                .collect(),
        );
        assert_eq!(
            i_wz_3.evaluate(
                &w_3_point
                    .iter()
                    .map(|&x| Fields::<F, E>::Extension(x))
                    .collect::<Vec<_>>()
            ),
            Fields::<F, E>::Extension(E::one())
        );

        // when w = [1, 0]
        let w_4_point = vec![E::from_wrapped_u32(1), E::from_wrapped_u32(0)];
        let i_wz_4 = MultilinearPoly::new_from_vec(
            2,
            (generate_igz::<F, E>(&w_4_point).iter())
                .map(|&x| Fields::<F, E>::Extension(x))
                .collect(),
        );
        assert_eq!(
            i_wz_4.evaluate(
                &w_4_point
                    .iter()
                    .map(|&x| Fields::<F, E>::Extension(x))
                    .collect::<Vec<_>>()
            ),
            Fields::<F, E>::Extension(E::one())
        );
    }
}
