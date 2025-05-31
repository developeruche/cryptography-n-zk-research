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
    use super::*;
    use p3_field::{AbstractExtensionField, AbstractField, extension::BinomialExtensionField};
    use p3_mersenne_31::Mersenne31;
    use poly::{Fields, mle::MultilinearPoly};

    type F = Mersenne31;
    type E = BinomialExtensionField<Mersenne31, 3>;

    #[test]
    fn test_generate_igz() {
        let evals = vec![1u32; 64];
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

        let igz_table = generate_igz::<F, E>(&points);

        // assert len = 4
        assert_eq!(igz_table.len(), 4);

        // assert values
        assert_eq!(igz_table, expected);
    }
}
