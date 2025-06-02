//! Utility functions for Libra.
use p3_field::{ExtensionField, Field};
use poly::Fields;

type Mle<F, E> = Vec<Fields<F, E>>;

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

pub fn merge_sumcheck_proofs<F: Field, E: ExtensionField<F>>(
    data: Vec<Vec<Mle<F, E>>>,
) -> Vec<Vec<Fields<F, E>>> {
    assert!(data.len() > 0, "Data must not be empty");
    assert!(
        data.iter().all(|x| x.len() > 0),
        "Each vector must not be empty"
    );
    assert!(
        data.iter().all(|x| x.len() == data[0].len()),
        "All vectors must have the same length"
    );

    let mut out = vec![];

    for i in 0..data[0].len() {
        let mut out_i = vec![];

        for j in 0..data.len() {
            out_i.push(data[j][i].clone())
        }

        out.push(add_vec_of_mle(out_i));
    }

    out
}

pub fn add_vec_of_mle<F: Field, E: ExtensionField<F>>(mles: Vec<Mle<F, E>>) -> Vec<Fields<F, E>> {
    // Handle empty input
    if mles.is_empty() {
        return Vec::new();
    }

    let first_len = mles[0].len();

    // Verify all MLEs have the same length
    assert!(
        mles.iter().all(|mle| mle.len() == first_len),
        "All MLE vectors must have the same length"
    );

    // Create the result by summing at each position
    (0..first_len)
        .map(|pos| {
            mles.iter()
                .fold(Fields::<F, E>::Base(F::zero()), |acc, mle| acc + mle[pos])
        })
        .collect()
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
        let mle_1 = vec![
            Fields::<F, E>::Base(F::from_wrapped_u32(1)),
            Fields::<F, E>::Base(F::from_wrapped_u32(2)),
            Fields::<F, E>::Base(F::from_wrapped_u32(3)),
        ];
        let mle_2 = vec![
            Fields::<F, E>::Base(F::from_wrapped_u32(4)),
            Fields::<F, E>::Base(F::from_wrapped_u32(5)),
            Fields::<F, E>::Base(F::from_wrapped_u32(6)),
        ];

        let mle_3 = vec![
            Fields::<F, E>::Base(F::from_wrapped_u32(2)),
            Fields::<F, E>::Base(F::from_wrapped_u32(4)),
            Fields::<F, E>::Base(F::from_wrapped_u32(8)),
        ];
        let mle_4 = vec![
            Fields::<F, E>::Base(F::from_wrapped_u32(10)),
            Fields::<F, E>::Base(F::from_wrapped_u32(12)),
            Fields::<F, E>::Base(F::from_wrapped_u32(14)),
        ];

        let mle = vec![vec![mle_1, mle_2], vec![mle_3, mle_4]];

        let mle = merge_sumcheck_proofs(mle);

        let expected = vec![
            vec![
                Fields::<F, E>::Base(F::from_wrapped_u32(3)),
                Fields::<F, E>::Base(F::from_wrapped_u32(6)),
                Fields::<F, E>::Base(F::from_wrapped_u32(11)),
            ],
            vec![
                Fields::<F, E>::Base(F::from_wrapped_u32(14)),
                Fields::<F, E>::Base(F::from_wrapped_u32(17)),
                Fields::<F, E>::Base(F::from_wrapped_u32(20)),
            ],
        ];

        assert_eq!(mle, expected);
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

    #[test]
    fn test_add_vec_of_mle() {
        // Test case 1: Basic addition of two MLEs with Base fields
        let mle1 = vec![
            Fields::<F, E>::Base(F::new(1)),
            Fields::<F, E>::Base(F::new(2)),
            Fields::<F, E>::Base(F::new(3)),
        ];

        let mle2 = vec![
            Fields::<F, E>::Base(F::new(4)),
            Fields::<F, E>::Base(F::new(5)),
            Fields::<F, E>::Base(F::new(6)),
        ];

        let result = add_vec_of_mle(vec![mle1.clone(), mle2.clone()]);

        assert_eq!(result.len(), 3);
        assert_eq!(result[0], Fields::<F, E>::Base(F::new(5))); // 1 + 4
        assert_eq!(result[1], Fields::<F, E>::Base(F::new(7))); // 2 + 5
        assert_eq!(result[2], Fields::<F, E>::Base(F::new(9))); // 3 + 6

        // Test case 2: Addition of three MLEs
        let mle3 = vec![
            Fields::<F, E>::Base(F::new(7)),
            Fields::<F, E>::Base(F::new(8)),
            Fields::<F, E>::Base(F::new(9)),
        ];

        let result = add_vec_of_mle(vec![mle1.clone(), mle2.clone(), mle3.clone()]);

        assert_eq!(result.len(), 3);
        assert_eq!(result[0], Fields::<F, E>::Base(F::new(12))); // 1 + 4 + 7
        assert_eq!(result[1], Fields::<F, E>::Base(F::new(15))); // 2 + 5 + 8
        assert_eq!(result[2], Fields::<F, E>::Base(F::new(18))); // 3 + 6 + 9

        // Test case 3: Addition of MLEs with Extension fields
        let ext_mle1 = vec![
            Fields::<F, E>::Extension(E::from_wrapped_u32(1)),
            Fields::<F, E>::Extension(E::from_wrapped_u32(2)),
        ];

        let ext_mle2 = vec![
            Fields::<F, E>::Extension(E::from_wrapped_u32(3)),
            Fields::<F, E>::Extension(E::from_wrapped_u32(4)),
        ];

        let result = add_vec_of_mle(vec![ext_mle1.clone(), ext_mle2.clone()]);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0], Fields::<F, E>::Extension(E::from_wrapped_u32(4))); // 1 + 3
        assert_eq!(result[1], Fields::<F, E>::Extension(E::from_wrapped_u32(6))); // 2 + 4

        // Test case 4: Addition of MLEs with mixed Base and Extension fields
        let mixed_mle1 = vec![
            Fields::<F, E>::Base(F::new(1)),
            Fields::<F, E>::Extension(E::from_wrapped_u32(2)),
        ];

        let mixed_mle2 = vec![
            Fields::<F, E>::Extension(E::from_wrapped_u32(3)),
            Fields::<F, E>::Base(F::new(4)),
        ];

        let result = add_vec_of_mle(vec![mixed_mle1.clone(), mixed_mle2.clone()]);

        assert_eq!(result.len(), 2);
        // First element: Base(1) + Extension(3) = Extension(4)
        assert_eq!(result[0].to_extension_field(), E::from_wrapped_u32(4));
        // Second element: Extension(2) + Base(4) = Extension(6)
        assert_eq!(result[1].to_extension_field(), E::from_wrapped_u32(6));

        // Test case 5: Single MLE
        let result = add_vec_of_mle(vec![mle1.clone()]);

        assert_eq!(result.len(), 3);
        assert_eq!(result[0], Fields::<F, E>::Base(F::new(1)));
        assert_eq!(result[1], Fields::<F, E>::Base(F::new(2)));
        assert_eq!(result[2], Fields::<F, E>::Base(F::new(3)));

        // Test case 6: Empty input
        let result = add_vec_of_mle(Vec::<Mle<F, E>>::new());
        assert_eq!(result.len(), 0);

        // Test case 7: MLEs with different lengths (should panic)
        let short_mle = vec![
            Fields::<F, E>::Base(F::new(1)),
            Fields::<F, E>::Base(F::new(2)),
        ];

        let long_mle = vec![
            Fields::<F, E>::Base(F::new(3)),
            Fields::<F, E>::Base(F::new(4)),
            Fields::<F, E>::Base(F::new(5)),
        ];

        // This should panic with "All MLE vectors must have the same length"
        let result =
            std::panic::catch_unwind(|| add_vec_of_mle(vec![short_mle.clone(), long_mle.clone()]));
        assert!(result.is_err());
    }

    #[test]
    fn test_merge_sumcheck_proofs_comprehensive() {
        // Basic case: Standard 2x2 structure with Base fields
        {
            let mle_1 = vec![
                Fields::<F, E>::Base(F::from_wrapped_u32(1)),
                Fields::<F, E>::Base(F::from_wrapped_u32(2)),
            ];
            let mle_2 = vec![
                Fields::<F, E>::Base(F::from_wrapped_u32(3)),
                Fields::<F, E>::Base(F::from_wrapped_u32(4)),
            ];

            let mle_3 = vec![
                Fields::<F, E>::Base(F::from_wrapped_u32(5)),
                Fields::<F, E>::Base(F::from_wrapped_u32(6)),
            ];
            let mle_4 = vec![
                Fields::<F, E>::Base(F::from_wrapped_u32(7)),
                Fields::<F, E>::Base(F::from_wrapped_u32(8)),
            ];

            let data = vec![vec![mle_1, mle_2], vec![mle_3, mle_4]];

            let result = merge_sumcheck_proofs(data);

            let expected = vec![
                vec![
                    Fields::<F, E>::Base(F::from_wrapped_u32(6)), // 1 + 5
                    Fields::<F, E>::Base(F::from_wrapped_u32(8)), // 2 + 6
                ],
                vec![
                    Fields::<F, E>::Base(F::from_wrapped_u32(10)), // 3 + 7
                    Fields::<F, E>::Base(F::from_wrapped_u32(12)), // 4 + 8
                ],
            ];

            assert_eq!(result, expected);
        }

        // Edge case 1: Single row, multiple columns
        {
            let mle_1 = vec![
                Fields::<F, E>::Base(F::from_wrapped_u32(1)),
                Fields::<F, E>::Base(F::from_wrapped_u32(2)),
            ];
            let mle_2 = vec![
                Fields::<F, E>::Base(F::from_wrapped_u32(3)),
                Fields::<F, E>::Base(F::from_wrapped_u32(4)),
            ];

            let data = vec![vec![mle_1, mle_2]];

            let result = merge_sumcheck_proofs(data);

            let expected = vec![
                vec![
                    Fields::<F, E>::Base(F::from_wrapped_u32(1)),
                    Fields::<F, E>::Base(F::from_wrapped_u32(2)),
                ],
                vec![
                    Fields::<F, E>::Base(F::from_wrapped_u32(3)),
                    Fields::<F, E>::Base(F::from_wrapped_u32(4)),
                ],
            ];

            assert_eq!(result, expected);
        }

        // Edge case 2: Multiple rows, single column
        {
            let mle_1 = vec![
                Fields::<F, E>::Base(F::from_wrapped_u32(1)),
                Fields::<F, E>::Base(F::from_wrapped_u32(2)),
            ];
            let mle_2 = vec![
                Fields::<F, E>::Base(F::from_wrapped_u32(3)),
                Fields::<F, E>::Base(F::from_wrapped_u32(4)),
            ];

            let data = vec![vec![mle_1], vec![mle_2]];

            let result = merge_sumcheck_proofs(data);

            let expected = vec![vec![
                Fields::<F, E>::Base(F::from_wrapped_u32(4)), // 1 + 3
                Fields::<F, E>::Base(F::from_wrapped_u32(6)), // 2 + 4
            ]];

            assert_eq!(result, expected);
        }

        // Edge case 3: Minimum valid input - single element
        {
            let mle = vec![
                Fields::<F, E>::Base(F::from_wrapped_u32(1)),
                Fields::<F, E>::Base(F::from_wrapped_u32(2)),
            ];

            let data = vec![vec![mle]];

            let result = merge_sumcheck_proofs(data);

            let expected = vec![vec![
                Fields::<F, E>::Base(F::from_wrapped_u32(1)),
                Fields::<F, E>::Base(F::from_wrapped_u32(2)),
            ]];

            assert_eq!(result, expected);
        }

        // Edge case 4: Mix of Base and Extension fields
        {
            let mle_1 = vec![
                Fields::<F, E>::Base(F::from_wrapped_u32(1)),
                Fields::<F, E>::Extension(E::from_wrapped_u32(2)),
            ];
            let mle_2 = vec![
                Fields::<F, E>::Extension(E::from_wrapped_u32(3)),
                Fields::<F, E>::Base(F::from_wrapped_u32(4)),
            ];

            let mle_3 = vec![
                Fields::<F, E>::Extension(E::from_wrapped_u32(5)),
                Fields::<F, E>::Base(F::from_wrapped_u32(6)),
            ];
            let mle_4 = vec![
                Fields::<F, E>::Base(F::from_wrapped_u32(7)),
                Fields::<F, E>::Extension(E::from_wrapped_u32(8)),
            ];

            let data = vec![vec![mle_1, mle_2], vec![mle_3, mle_4]];

            let result = merge_sumcheck_proofs(data);

            // Check first result: Element 1 + Element 3
            assert_eq!(result[0][0].to_extension_field(), E::from_wrapped_u32(6)); // 1 + 5 
            assert_eq!(result[0][1].to_extension_field(), E::from_wrapped_u32(8)); // 2 + 6

            // Check second result: Element 2 + Element 4
            assert_eq!(result[1][0].to_extension_field(), E::from_wrapped_u32(10)); // 3 + 7
            assert_eq!(result[1][1].to_extension_field(), E::from_wrapped_u32(12)); // 4 + 8
        }

        // Edge case 5: Zero values
        {
            let mle_1 = vec![
                Fields::<F, E>::Base(F::zero()),
                Fields::<F, E>::Base(F::zero()),
            ];
            let mle_2 = vec![
                Fields::<F, E>::Base(F::zero()),
                Fields::<F, E>::Base(F::zero()),
            ];

            let mle_3 = vec![
                Fields::<F, E>::Base(F::from_wrapped_u32(1)),
                Fields::<F, E>::Base(F::from_wrapped_u32(2)),
            ];
            let mle_4 = vec![
                Fields::<F, E>::Base(F::from_wrapped_u32(3)),
                Fields::<F, E>::Base(F::from_wrapped_u32(4)),
            ];

            let data = vec![vec![mle_1, mle_2], vec![mle_3, mle_4]];

            let result = merge_sumcheck_proofs(data);

            let expected = vec![
                vec![
                    Fields::<F, E>::Base(F::from_wrapped_u32(1)),
                    Fields::<F, E>::Base(F::from_wrapped_u32(2)),
                ],
                vec![
                    Fields::<F, E>::Base(F::from_wrapped_u32(3)),
                    Fields::<F, E>::Base(F::from_wrapped_u32(4)),
                ],
            ];

            assert_eq!(result, expected);
        }

        // Edge case 6: Rectangular structure (not square)
        {
            let mle_1 = vec![
                Fields::<F, E>::Base(F::from_wrapped_u32(1)),
                Fields::<F, E>::Base(F::from_wrapped_u32(2)),
            ];
            let mle_2 = vec![
                Fields::<F, E>::Base(F::from_wrapped_u32(3)),
                Fields::<F, E>::Base(F::from_wrapped_u32(4)),
            ];
            let mle_3 = vec![
                Fields::<F, E>::Base(F::from_wrapped_u32(5)),
                Fields::<F, E>::Base(F::from_wrapped_u32(6)),
            ];

            let mle_4 = vec![
                Fields::<F, E>::Base(F::from_wrapped_u32(7)),
                Fields::<F, E>::Base(F::from_wrapped_u32(8)),
            ];
            let mle_5 = vec![
                Fields::<F, E>::Base(F::from_wrapped_u32(9)),
                Fields::<F, E>::Base(F::from_wrapped_u32(10)),
            ];
            let mle_6 = vec![
                Fields::<F, E>::Base(F::from_wrapped_u32(11)),
                Fields::<F, E>::Base(F::from_wrapped_u32(12)),
            ];

            let data = vec![vec![mle_1, mle_2, mle_3], vec![mle_4, mle_5, mle_6]];

            let result = merge_sumcheck_proofs(data);

            let expected = vec![
                vec![
                    Fields::<F, E>::Base(F::from_wrapped_u32(8)),  // 1 + 7
                    Fields::<F, E>::Base(F::from_wrapped_u32(10)), // 2 + 8
                ],
                vec![
                    Fields::<F, E>::Base(F::from_wrapped_u32(12)), // 3 + 9
                    Fields::<F, E>::Base(F::from_wrapped_u32(14)), // 4 + 10
                ],
                vec![
                    Fields::<F, E>::Base(F::from_wrapped_u32(16)), // 5 + 11
                    Fields::<F, E>::Base(F::from_wrapped_u32(18)), // 6 + 12
                ],
            ];

            assert_eq!(result, expected);
        }

        // Edge case 7: Many rows
        {
            let mle = vec![
                Fields::<F, E>::Base(F::from_wrapped_u32(1)),
                Fields::<F, E>::Base(F::from_wrapped_u32(2)),
            ];

            // Create 10 rows with the same MLE
            let data = (0..10).map(|_| vec![mle.clone()]).collect::<Vec<_>>();

            let result = merge_sumcheck_proofs(data);

            let expected = vec![vec![
                Fields::<F, E>::Base(F::from_wrapped_u32(10)), // 1 * 10
                Fields::<F, E>::Base(F::from_wrapped_u32(20)), // 2 * 10
            ]];

            assert_eq!(result, expected);
        }

        // Error case 1: Empty input
        {
            let empty_data: Vec<Vec<Mle<F, E>>> = Vec::new();

            let result = std::panic::catch_unwind(|| merge_sumcheck_proofs(empty_data));

            assert!(result.is_err());
        }

        // Error case 2: Empty inner vector
        {
            let empty_inner: Vec<Mle<F, E>> = Vec::new();
            let data = vec![empty_inner];

            let result = std::panic::catch_unwind(|| merge_sumcheck_proofs(data));

            assert!(result.is_err());
        }

        // Error case 3: Inconsistent dimensions
        {
            let mle_1 = vec![
                Fields::<F, E>::Base(F::from_wrapped_u32(1)),
                Fields::<F, E>::Base(F::from_wrapped_u32(2)),
            ];
            let mle_2 = vec![
                Fields::<F, E>::Base(F::from_wrapped_u32(3)),
                Fields::<F, E>::Base(F::from_wrapped_u32(4)),
            ];

            let mle_3 = vec![
                Fields::<F, E>::Base(F::from_wrapped_u32(5)),
                Fields::<F, E>::Base(F::from_wrapped_u32(6)),
            ];
            // This row has only one element, but the first row has two
            let data = vec![vec![mle_1, mle_2], vec![mle_3]];

            let result = std::panic::catch_unwind(|| merge_sumcheck_proofs(data));

            assert!(result.is_err());
        }
    }
}
