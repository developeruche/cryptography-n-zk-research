use ark_ff::Field;

pub fn generate_hypercube_vec(m: usize) -> Vec<Vec<u8>> {
    if m >= 63 {
        panic!("m is too large for a vector-based hypercube");
    }
    let num_rows = 1 << m;
    let mut result = Vec::with_capacity(num_rows);

    for i in 0..num_rows {
        let mut row = Vec::with_capacity(m);
        for bit_index in (0..m).rev() {
            row.push(((i >> bit_index) & 1) as u8);
        }

        result.push(row);
    }

    result
}

pub fn eq_1_func<F: Field>(z_i: F, b_i: F) -> F {
    z_i * b_i + (F::one() - z_i) * (F::one() - b_i)
}

pub fn eq_funcs<F: Field>(points: &[F]) -> Vec<F> {
    let bh = generate_hypercube_vec(points.len());
    let mut evals = Vec::with_capacity(bh.len());

    // Iterate over every point 'b' in the hypercube {0,1}^m
    for b in bh {
        let mut product_term = F::one();

        for (i, &bit) in b.iter().enumerate() {
            // If b_i = 1, term is z_i
            // If b_i = 0, term is (1 - z_i)
            let term = if bit == 1 {
                points[i]
            } else {
                F::one() - points[i]
            };

            product_term *= term;
        }
        evals.push(product_term);
    }

    evals
}

pub fn mle_eval_direct<F: Field>(points: &[F], evals: &[F]) -> F {
    let m = points.len();
    assert_eq!(evals.len(), 1 << m, "Evaluations size must match 2^m");

    let eq_s = eq_funcs(points);

    let mut res = F::zero();

    for (i, eval) in evals.iter().enumerate() {
        let temp_res = *eval * eq_s[i];
        res += temp_res;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_test_curves::bls12_381::Fr;

    #[test]
    fn test_direct_mle_eval() {
        let evals = vec![Fr::from(3), Fr::from(1), Fr::from(2), Fr::from(5)];
        let points = vec![Fr::from(5), Fr::from(6)];
        let result = mle_eval_direct(&points, &evals);
        assert_eq!(result, Fr::from(136));
    }

    #[test]
    fn test_direct_mle_eval_0() {
        let evals = vec![
            Fr::from(1),
            Fr::from(2),
            Fr::from(3),
            Fr::from(4),
            Fr::from(5),
            Fr::from(6),
            Fr::from(7),
            Fr::from(8),
        ];
        let points = vec![Fr::from(2), Fr::from(3), Fr::from(4)];
        let result = mle_eval_direct(&points, &evals);
        assert_eq!(result, Fr::from(19));
    }
}
