use std::fmt::format;
use num::One;
pub use num_bigint::BigUint;
use crate::utils::{exponentiate, hash_str};

pub mod utils;




#[derive(Debug)]
pub struct NICP {
    pub alpha: BigUint,
    pub beta: BigUint,
    pub modulus: BigUint,
    pub order: BigUint
}


impl NICP {
    pub fn new() -> NICP {
        NICP {
            alpha: BigUint::from(6u32),
            beta: BigUint::from(2892u32),
            modulus: BigUint::from(10009u32),
            order: BigUint::from(5004u32)
        }
    }
}




pub fn gen_challenge(
    y_one: &BigUint,
    y_two: &BigUint,
    r_one: &BigUint,
    r_two: &BigUint
) -> BigUint {
    let to_hash_str = format!("{}{}{}{}",y_one,y_two,r_one,r_two);
    let hash__ = hash_str(&to_hash_str.as_str());

    BigUint::from_bytes_be(&hex::decode(hash__).unwrap())
}


pub fn solve_challenge(
    rand_prover_k: &BigUint,
    secret_from_prover: &BigUint,
    rand_verifier_c: &BigUint,
    modulus: &BigUint
) -> BigUint {
    let c_mul_x = rand_verifier_c * secret_from_prover;

    if *rand_prover_k > c_mul_x {
        (rand_prover_k - c_mul_x).modpow(&BigUint::one(), modulus)
    } else {
        modulus - (c_mul_x - rand_prover_k).modpow(&BigUint::one(), modulus)
    }
}


pub fn verify_challenge(
    alpha: &BigUint,
    beta: &BigUint,
    solution: &BigUint,
    challenge_in: &BigUint,
    y_one: &BigUint,
    y_two: &BigUint,
    prime: &BigUint
) -> bool {
    let alpha_expo_s = exponentiate(alpha, solution, prime);
    let y_one_exp_c = exponentiate(y_one, challenge_in, prime);
    let alpha_e_s_multi_y_one_expo_c = alpha_expo_s * y_one_exp_c;
    let gen_r_one =  exponentiate(&alpha_e_s_multi_y_one_expo_c, &BigUint::one(), prime);

    let beta_expo_s = exponentiate(beta, solution, prime);
    let y_two_expo_c = exponentiate(y_two, challenge_in, prime);
    let beta_e_s_multi_y_two_expo_c = beta_expo_s * y_two_expo_c;
    let gen_r_two = exponentiate(&beta_e_s_multi_y_two_expo_c, &BigUint::one(), prime);

    println!("this is the generated {} - {}", gen_r_one, gen_r_two);
    let gen_challenge_res =  gen_challenge(
        y_one,
        y_two,
        &gen_r_one,
        &gen_r_two
    );


    println!("{} - {} this is the c in and out", challenge_in, gen_challenge_res);


    *challenge_in == gen_challenge_res
}













#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_challenge_one() {
        let k = BigUint::from(10u32);
        let x = BigUint::from(3u32);
        let c = BigUint::from(3u32);
        let p = BigUint::from(10u32);


        let solution = solve_challenge(
            &k,
            &x,
            &c,
            &p
        );


        assert_eq!(solution, BigUint::one());
    }

    #[test]
    fn test_solve_challenge_two() {
        // testing negative
        let x = BigUint::from(4u32);
        let c = BigUint::from(3u32);
        let k = BigUint::from(10u32);
        let p = BigUint::from(10u32);

        let solution = solve_challenge(
            &k,
            &x,
            &c,
            &p
        );


        assert_eq!(solution, BigUint::from(8u32));
    }



    #[test]
    fn test_the_toy_example() {
        let p = BigUint::from(10009u32);
        let q = (&p - BigUint::one()) / BigUint::from(2u32);

        let x = BigUint::from(300u32);
        let g = BigUint::from(3u32);
        let h = BigUint::from(2892u32);

        let y1 = exponentiate(&g, &x, &p);
        let y2 = exponentiate(&h, &x, &p);

        let k = BigUint::from(10u32);

        let r1 = exponentiate(&g, &k, &p);
        let r2 = exponentiate(&h, &k, &p);

        let c = gen_challenge(
            &y1,
            &y2,
            &r1,
            &r2
        );

        let s = solve_challenge(&k, &x, &c, &q);

        println!("this is the input {} - {} ", r1, r2);


        let verification = verify_challenge(&g, &h, &s, &c, &y1, &y2, &p);
        assert!(verification)
    }
}