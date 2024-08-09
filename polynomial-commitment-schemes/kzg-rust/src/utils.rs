use ark_ec::{pairing::Pairing, Group};
use ark_ff::PrimeField;
use polynomial::univariant::UnivariantPolynomial;

pub fn linear_combination_homomorphic_poly_eval_g1<P>(
    poly: &UnivariantPolynomial<P::ScalarField>,
    powers_of_secret_gx: &[P::G1],
) -> P::G1
where
    P: Pairing,
{
    poly.coefficients
        .iter()
        .enumerate()
        .fold(P::G1::default(), |mut acc, (index, coeff)| {
            let res = powers_of_secret_gx[index].mul_bigint(coeff.into_bigint());
            acc = acc + res;
            acc
        })
}

/// This function generates the powers of tau for the circuit
/// tau = 5;
/// powers_of_tau_g1 = [g.5^0 g.5^1, g.5^2, g.5^3, g.5^4, g.5^5, g.5^6, g.5^7]
pub fn generate_powers_of_tau_g1<P: Pairing>(tau: &P::ScalarField, n: usize) -> Vec<P::G1> {
    let n = n + 1;
    let mut powers_of_tau_g1 = Vec::with_capacity(n);
    let mut tau_power = *tau;
    let generator = P::G1::generator();

    powers_of_tau_g1.push(generator);

    for _ in 1..n {
        powers_of_tau_g1.push(generator.mul_bigint(tau_power.into_bigint()));
        tau_power = tau_power * *tau;
    }

    powers_of_tau_g1
}

pub fn linear_combination_homomorphic_poly_eval_g1_primefield<P, F>(
    poly: &UnivariantPolynomial<F>,
    powers_of_secret_gx: &[P::G1],
) -> P::G1
where
    P: Pairing,
    F: PrimeField,
{
    poly.coefficients
        .iter()
        .enumerate()
        .fold(P::G1::default(), |mut acc, (index, coeff)| {
            let res = powers_of_secret_gx[index].mul_bigint(coeff.into_bigint());
            acc = acc + res;
            acc
        })
}
