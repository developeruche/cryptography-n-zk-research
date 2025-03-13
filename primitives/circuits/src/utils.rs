use ark_ff::PrimeField;
use polynomial::{interface::MultilinearPolynomialInterface, multilinear::Multilinear};

/// This function is used to convert a vector of usize to a multilinear extension
/// param vec: This is the vector of usize, indicating the variables that are set to ONE
/// param max_size: This is the maximum size of the multilinear extension
pub fn usize_vec_to_mle<F: PrimeField>(vec: &[usize], num_var: usize) -> Multilinear<F> {
    let mut mle = Multilinear::zero(num_var);

    for i in vec {
        mle.evaluations[*i] = F::one();
    }

    mle
}

pub fn get_gate_properties(a: usize, b: usize, c: usize, layer_index: usize) -> usize {
    let mut a_bin = format!("{:b}", a);
    let mut b_bin = format!("{:b}", b);
    let mut c_bin = format!("{:b}", c);

    pad_left(&mut a_bin, layer_index);
    pad_left(&mut b_bin, layer_index + 1);
    pad_left(&mut c_bin, layer_index + 1);

    let abc_bin_string = a_bin + &b_bin + &c_bin;
    let abc_decimal = usize::from_str_radix(&abc_bin_string, 2).unwrap();

    abc_decimal
}

pub fn compute_mle_num_var_from_layer_index(layer_index: usize) -> usize {
    if layer_index == 0 {
        return 3;
    }

    let a_len = layer_index;
    let b_n_c_len = a_len + 1;

    a_len + (2 * b_n_c_len)
}

pub fn pad_left(text: &mut String, target_len: usize) {
    let padding_len = target_len.saturating_sub(text.len());
    let padding = String::from("0").repeat(padding_len);
    text.insert_str(0, &padding);
}

pub fn compute_constraint_item(layer_index: usize, gate_index: usize) -> usize {
    let a_str = layer_index.to_string();
    let b_str = gate_index.to_string();
    let combined_str = format!("{}{}", a_str, b_str);
    let combined_usize: usize = combined_str
        .parse()
        .expect("Failed to parse combined string to usize");

    combined_usize
}

pub fn check_init<F: PrimeField>(
    r_s_a: Vec<Vec<F>>,
    r_s_b: Vec<Vec<F>>,
    r_s_c: Vec<Vec<F>>,
    w: Vec<F>,
) -> bool {
    let mut result_a = Vec::new();
    let mut result_b = Vec::new();
    let mut result_c = Vec::new();

    for i in r_s_a {
        result_a.push(mul_n_sum(i.clone(), w.clone()));
    }

    for i in r_s_b {
        result_b.push(mul_n_sum(i.clone(), w.clone()));
    }

    for i in r_s_c {
        result_c.push(mul_n_sum(i.clone(), w.clone()));
    }

    quick_vec_mul(result_a, result_b) == result_c
}

pub fn mul_n_sum<F: PrimeField>(a: Vec<F>, b: Vec<F>) -> F {
    let mut result = F::zero();

    for i in 0..a.len() {
        result += a[i] * b[i];
    }

    result
}

pub fn quick_vec_mul<F: PrimeField>(a: Vec<F>, b: Vec<F>) -> Vec<F> {
    let mut result = Vec::new();

    for i in 0..a.len() {
        result.push(a[i] * b[i]);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_test_curves::bls12_381::Fr;
    use polynomial::interface::MultilinearPolynomialInterface;

    #[test]
    fn test_usize_vec_to_mle() {
        let vec = vec![1];
        let num_var = 3;

        let mle = usize_vec_to_mle::<Fr>(&vec, num_var);

        let eval_0 = mle.evaluate(&vec![Fr::from(0u32), Fr::from(0u32), Fr::from(0u32)]);
        let eval_1 = mle.evaluate(&vec![Fr::from(0u32), Fr::from(0u32), Fr::from(1u32)]);
        let eval_2 = mle.evaluate(&vec![Fr::from(1u32), Fr::from(0u32), Fr::from(0u32)]);

        // assert all binanry inputs to mle returns ZERO
        assert_eq!(eval_0, Some(Fr::from(0u32)));
        assert_eq!(eval_2, Some(Fr::from(0u32)));

        // assert the input to mle returns ONE
        assert_eq!(eval_1, Some(Fr::from(1u32)));
    }

    #[test]
    fn test_usize_vec_to_mle_0() {
        let vec = vec![1];
        let num_var = 5;

        let mle = usize_vec_to_mle::<Fr>(&vec, num_var);

        let eval_0_1 = mle.evaluate(&vec![
            Fr::from(0),
            Fr::from(0),
            Fr::from(0),
            Fr::from(0),
            Fr::from(0),
        ]);
        let eval_0_2 = mle.evaluate(&vec![
            Fr::from(0),
            Fr::from(1),
            Fr::from(0),
            Fr::from(0),
            Fr::from(0),
        ]);
        let eval_0_3 = mle.evaluate(&vec![
            Fr::from(0),
            Fr::from(1),
            Fr::from(1),
            Fr::from(0),
            Fr::from(0),
        ]);
        let eval_0_4 = mle.evaluate(&vec![
            Fr::from(1),
            Fr::from(0),
            Fr::from(0),
            Fr::from(0),
            Fr::from(1),
        ]);
        let eval_0_5 = mle.evaluate(&vec![
            Fr::from(1),
            Fr::from(0),
            Fr::from(0),
            Fr::from(0),
            Fr::from(0),
        ]);
        let eval_0_6 = mle.evaluate(&vec![
            Fr::from(0),
            Fr::from(0),
            Fr::from(1),
            Fr::from(0),
            Fr::from(0),
        ]);

        let eval_1 = mle.evaluate(&vec![
            Fr::from(0),
            Fr::from(0),
            Fr::from(0),
            Fr::from(0),
            Fr::from(1),
        ]);

        // assert all binanry inputs to mle returns ZERO
        assert_eq!(eval_0_1, Some(Fr::from(0u32)));
        assert_eq!(eval_0_2, Some(Fr::from(0u32)));
        assert_eq!(eval_0_3, Some(Fr::from(0u32)));
        assert_eq!(eval_0_4, Some(Fr::from(0u32)));
        assert_eq!(eval_0_5, Some(Fr::from(0u32)));
        assert_eq!(eval_0_6, Some(Fr::from(0u32)));

        // assert the input to mle returns ONE
        assert_eq!(eval_1, Some(Fr::from(1u32)));
    }

    #[test]
    fn test_usize_vec_to_mle_1() {
        let vec = vec![83, 165, 247]; // 01010011, 10100101, 11110111
        let num_var = 8;

        let mle = usize_vec_to_mle::<Fr>(&vec, num_var);

        let eval_0_1 = mle.evaluate(&vec![
            Fr::from(1),
            Fr::from(1),
            Fr::from(0),
            Fr::from(1),
            Fr::from(0),
            Fr::from(1),
            Fr::from(0),
            Fr::from(1),
        ]);
        let eval_0_2 = mle.evaluate(&vec![
            Fr::from(1),
            Fr::from(0),
            Fr::from(0),
            Fr::from(0),
            Fr::from(0),
            Fr::from(0),
            Fr::from(1),
            Fr::from(1),
        ]);
        let eval_0_3 = mle.evaluate(&vec![
            Fr::from(1),
            Fr::from(1),
            Fr::from(0),
            Fr::from(0),
            Fr::from(0),
            Fr::from(0),
            Fr::from(0),
            Fr::from(1),
        ]);
        let eval_0_4 = mle.evaluate(&vec![
            Fr::from(1),
            Fr::from(0),
            Fr::from(0),
            Fr::from(0),
            Fr::from(1),
            Fr::from(0),
            Fr::from(1),
            Fr::from(1),
        ]);
        let eval_0_5 = mle.evaluate(&vec![
            Fr::from(1),
            Fr::from(0),
            Fr::from(0),
            Fr::from(0),
            Fr::from(0),
            Fr::from(0),
            Fr::from(0),
            Fr::from(1),
        ]);
        let eval_0_6 = mle.evaluate(&vec![
            Fr::from(0),
            Fr::from(0),
            Fr::from(1),
            Fr::from(0),
            Fr::from(0),
            Fr::from(1),
            Fr::from(0),
            Fr::from(1),
        ]);
        let eval_0_7 = mle.evaluate(&vec![
            Fr::from(0),
            Fr::from(1),
            Fr::from(0),
            Fr::from(1),
            Fr::from(0),
            Fr::from(0),
            Fr::from(0),
            Fr::from(1),
        ]);

        let eval_1_0 = mle.evaluate(&vec![
            Fr::from(0),
            Fr::from(1),
            Fr::from(0),
            Fr::from(1),
            Fr::from(0),
            Fr::from(0),
            Fr::from(1),
            Fr::from(1),
        ]);
        let eval_1_1 = mle.evaluate(&vec![
            Fr::from(1),
            Fr::from(0),
            Fr::from(1),
            Fr::from(0),
            Fr::from(0),
            Fr::from(1),
            Fr::from(0),
            Fr::from(1),
        ]);
        let eval_1_2 = mle.evaluate(&vec![
            Fr::from(1),
            Fr::from(1),
            Fr::from(1),
            Fr::from(1),
            Fr::from(0),
            Fr::from(1),
            Fr::from(1),
            Fr::from(1),
        ]);

        // assert all binanry inputs to mle returns ZERO
        assert_eq!(eval_0_1, Some(Fr::from(0u32)));
        assert_eq!(eval_0_2, Some(Fr::from(0u32)));
        assert_eq!(eval_0_3, Some(Fr::from(0u32)));
        assert_eq!(eval_0_4, Some(Fr::from(0u32)));
        assert_eq!(eval_0_5, Some(Fr::from(0u32)));
        assert_eq!(eval_0_6, Some(Fr::from(0u32)));
        assert_eq!(eval_0_7, Some(Fr::from(0u32)));

        // assert the input to mle returns ONE
        assert_eq!(eval_1_0, Some(Fr::from(1u32)));
        assert_eq!(eval_1_1, Some(Fr::from(1u32)));
        assert_eq!(eval_1_2, Some(Fr::from(1u32)));
    }

    #[test]
    fn test_compute_mle_num_var_from_layer_index() {
        let num_vars = compute_mle_num_var_from_layer_index(0);
        assert_eq!(num_vars, 3);

        let num_vars = compute_mle_num_var_from_layer_index(1);
        assert_eq!(num_vars, 5);

        let num_vars = compute_mle_num_var_from_layer_index(2);
        assert_eq!(num_vars, 8);
    }

    #[test]
    fn test_get_gate_properties() {}

    #[test]
    fn test_check_init() {
        // constraints_A = np.array([[0, 1, 0, 0, 0, 0],
        //                           [0, 0, 0, 1, 0, 0],
        //                           [0, 1, 0, 0, 1, 0],
        //                           [5, 0, 0, 0, 0, 1]])
        // constraints_B = np.array([[0, 1, 0, 0, 0, 0],
        //                           [0, 1, 0, 0, 0, 0],
        //                           [1, 0, 0, 0, 0, 0],
        //                           [1, 0, 0, 0, 0, 0]])
        // constraints_C = np.array([[0, 0, 0, 1, 0, 0],
        //                           [0, 0, 0, 0, 1, 0],
        //                           [0, 0, 0, 0, 0, 1],
        //                           [0, 0, 1, 0, 0, 0]])
        let r_s_a = vec![
            vec![
                Fr::from(0),
                Fr::from(1),
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
            ],
            vec![
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(1),
                Fr::from(0),
                Fr::from(0),
            ],
            vec![
                Fr::from(0),
                Fr::from(1),
                Fr::from(0),
                Fr::from(0),
                Fr::from(1),
                Fr::from(0),
            ],
            vec![
                Fr::from(5),
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(1),
            ],
        ];
        let r_s_b = vec![
            vec![
                Fr::from(0),
                Fr::from(1),
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
            ],
            vec![
                Fr::from(0),
                Fr::from(1),
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
            ],
            vec![
                Fr::from(1),
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
            ],
            vec![
                Fr::from(1),
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
            ],
        ];
        let r_s_c = vec![
            vec![
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(1),
                Fr::from(0),
                Fr::from(0),
            ],
            vec![
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(1),
                Fr::from(0),
            ],
            vec![
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(1),
            ],
            vec![
                Fr::from(0),
                Fr::from(0),
                Fr::from(1),
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
            ],
        ];

        // [1, 3, 35, 9, 27, 30]
        let w = vec![
            Fr::from(1),
            Fr::from(3),
            Fr::from(35),
            Fr::from(9),
            Fr::from(27),
            Fr::from(30),
        ];

        let res = check_init::<Fr>(r_s_a, r_s_b, r_s_c, w);

        assert_eq!(res, true);
    }
}
