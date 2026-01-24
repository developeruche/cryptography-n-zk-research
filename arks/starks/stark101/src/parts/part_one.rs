//! Part One of the Stark101 course
//! - Low degree Extension
//! - Commitment

use polynomial::{ark_ff::PrimeField, evaluation::univariate::UnivariateEval};


pub fn part_one<F: PrimeField>(a_s: &[F]) {
    // Trace of the computation
    // a_0, a_1, a_2, a_3,..., a_n
    // 1, g, g^2, g^3,..., g^n
    // f(g) = a;

    let f_of_g = UnivariateEval::from_coefficients(a_s.to_vec());
    
    // Choosing the larger evaluation domain
    
}
