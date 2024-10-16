use super::Domain;
use ark_ff::PrimeField;

pub fn split_odd_even<F: PrimeField>(arr: &Vec<F>) -> (Vec<F>, Vec<F>) {
    let even: Vec<F> = arr.iter().step_by(2).cloned().collect();
    let odd: Vec<F> = arr.iter().skip(1).step_by(2).cloned().collect();

    (even, odd)
}
