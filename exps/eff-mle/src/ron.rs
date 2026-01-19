use ark_ff::Field;

#[derive(Clone, Copy, Debug)]
pub struct UpdateRatio<F: Field> {
    up: F,   // Multiplier to flip 0 -> 1:  z / (1-z)
    down: F, // Multiplier to flip 1 -> 0:  (1-z) / z
}

pub fn mle_eval_ron_optimized<F: Field>(evals: &[F], z: &[F]) -> F {
    let m = z.len();
    assert_eq!(evals.len(), 1 << m, "Evaluations size must match 2^m");

    let mut ratios = Vec::with_capacity(m);
    let mut active_bit_positions = Vec::with_capacity(m);

    let mut global_idx = 0usize;
    let mut current_eq = F::one();

    for (i, &val) in z.iter().enumerate() {
        let global_bit_pos = m - 1 - i;

        if val.is_zero() {
        } else if val.is_one() {
            global_idx |= 1 << global_bit_pos;
        } else {
            let one_minus_val = F::one() - val;
            let r_up = val * one_minus_val.inverse().unwrap();
            let r_down = one_minus_val * val.inverse().unwrap();

            ratios.push(UpdateRatio {
                up: r_up,
                down: r_down,
            });
            active_bit_positions.push(global_bit_pos);
            current_eq *= one_minus_val;
        }
    }

    let num_active = active_bit_positions.len();
    if num_active == 0 {
        return current_eq * evals[global_idx];
    }

    let num_steps = 1 << num_active;
    let mut total_sum = F::zero();

    total_sum += current_eq * evals[global_idx];

    let mut prev_gray = 0usize;

    for k in 1..num_steps {
        let gray = (k >> 1) ^ k;
        let diff = prev_gray ^ gray;

        let active_idx = diff.trailing_zeros() as usize;
        let global_bit_pos = active_bit_positions[active_idx];

        let direction_up = (gray & diff) != 0;

        if direction_up {
            current_eq *= ratios[active_idx].up;
        } else {
            current_eq *= ratios[active_idx].down;
        }

        global_idx ^= 1 << global_bit_pos;
        total_sum += current_eq * evals[global_idx];

        prev_gray = gray;
    }

    total_sum
}

#[cfg(test)]
mod tests {
    use crate::direct::mle_eval_direct;

    use super::*;
    use ark_test_curves::bls12_381::Fr;

    fn create_rand_vec<F: Field>(num_vars: u32) -> Vec<F> {
        let rand_engine = &mut ark_std::test_rng();
        (0..num_vars)
            .into_iter()
            .map(|_| F::rand(rand_engine))
            .collect()
    }

    #[test]
    fn test_optimized_mle_eval() {
        let evals = vec![Fr::from(3), Fr::from(1), Fr::from(2), Fr::from(5)];
        let points = vec![Fr::from(1), Fr::from(6)];

        let result = mle_eval_ron_optimized(&evals, &points);
        assert_eq!(result, Fr::from(20));
    }

    #[test]
    fn test_ron_vs_direct() {
        const NUM_VAR: u32 = 10;
        let evals = create_rand_vec::<Fr>(1 << NUM_VAR);
        let points = create_rand_vec::<Fr>(NUM_VAR);
        let result = mle_eval_ron_optimized(&evals, &points);
        assert_eq!(result, mle_eval_direct(&points, &evals));
    }
}
