use ark_ec::{pairing::Pairing, AffineRepr, Group};
use ark_ff::{Field, PrimeField};
use polynomial::{
    interface::{PolynomialInterface, UnivariantPolynomialInterface},
    univariant::UnivariantPolynomial,
};

/// This is the index of the public variables in the witness
/// this is constant for groth16
const PRIVATE_VARIABLES_INDEX: usize = 1;

/// This function generates the t-polynomial for the circuit
/// we get this;
/// t(x) = (x-1)(x-2)(x-3)(x-4)(x-5)(x-6)(x-7)
/// where 7 is the number of constraints
pub fn generate_t_poly<F: PrimeField>(number_of_constraints: usize) -> UnivariantPolynomial<F> {
    let mut t = UnivariantPolynomial::from_coefficients_vec(vec![F::one()]);
    for i in 1..number_of_constraints + 1 {
        t = t * UnivariantPolynomial::from_coefficients_vec(vec![-F::from(i as u64), F::one()]);
    }

    t
}

/// tau = 5;
/// powers_of_secret_gx = [g^5, g^10, g^15, g^20, g^25, g^30, g^35]
pub fn linear_combination_homomorphic_poly_eval_g1<P>(
    poly: &UnivariantPolynomial<P::ScalarField>,
    powers_of_secret_gx: Vec<P::G1>,
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
/// powers_of_tau_g1 = [g^5^0 g^5, g^10, g^15, g^20, g^25, g^30, g^35]
pub fn generate_powers_of_tau_g1<P: Pairing>(tau: P::ScalarField, n: usize) -> Vec<P::G1> {
    let mut powers_of_tau_g1 = Vec::with_capacity(n);
    let mut tau_power = tau;
    let generator = P::G1::generator();

    powers_of_tau_g1.push(generator);

    for _ in 1..n {
        powers_of_tau_g1.push(generator.mul_bigint(tau_power.into_bigint()));
        tau_power = tau_power * tau;
    }

    powers_of_tau_g1
}

pub fn generate_powers_of_tau_g2<P: Pairing>(tau: P::ScalarField, n: usize) -> Vec<P::G2> {
    let mut powers_of_tau_g2 = Vec::with_capacity(n);
    let mut tau_power = tau;
    let generator = P::G2::generator();

    powers_of_tau_g2.push(generator);

    for _ in 1..n {
        powers_of_tau_g2.push(generator.mul_bigint(tau_power.into_bigint()));
        tau_power = tau_power * tau;
    }

    powers_of_tau_g2
}

pub fn generate_powers_of_tau_g1_alpha_or_beta<P: Pairing>(
    tau: P::ScalarField,
    alpha_or_beta: P::ScalarField,
    n: usize,
) -> Vec<P::G1> {
    let mut powers_of_tau_g1_alpha_or_beta = Vec::with_capacity(n);
    let mut tau_power = tau;
    let generator = P::G1::generator();

    powers_of_tau_g1_alpha_or_beta.push(generator.mul_bigint(alpha_or_beta.into_bigint()));

    for _ in 1..n {
        let g1_p_of_tau = generator.mul_bigint(tau_power.into_bigint());
        powers_of_tau_g1_alpha_or_beta.push(g1_p_of_tau.mul_bigint(alpha_or_beta.into_bigint()));
        tau_power = tau_power * tau;
    }

    powers_of_tau_g1_alpha_or_beta
}

pub fn generate_powers_of_tau_t_poly_delta_inverse_g1<P: Pairing>(
    tau: P::ScalarField,
    delta_inverse: P::ScalarField,
    t_poly: &UnivariantPolynomial<P::ScalarField>,
    n: usize,
) -> Vec<P::G1> {
    let mut powers_of_tau_t_poly_delta_inverse_g1 = Vec::with_capacity(n);
    let mut tau_power = tau;
    let generator = P::G1::generator();

    let first_element = t_poly.evaluate(&tau) * delta_inverse;
    powers_of_tau_t_poly_delta_inverse_g1.push(generator.mul_bigint(first_element.into_bigint()));

    for _ in 1..n {
        powers_of_tau_t_poly_delta_inverse_g1.push(
            generator.mul_bigint((tau_power * t_poly.evaluate(&tau) * delta_inverse).into_bigint()),
        );
        tau_power = tau_power * tau;
    }

    powers_of_tau_t_poly_delta_inverse_g1
}

pub fn generate_c_tau_plus_beta_a_tau_plus_alpha_b_tau_g1_public<P: Pairing>(
    c_tau_plus_beta_a_tau_plus_alpha_b_tau: &Vec<P::ScalarField>,
    gamma: &P::ScalarField,
) -> Vec<P::G1> {
    let mut result = Vec::with_capacity(c_tau_plus_beta_a_tau_plus_alpha_b_tau.len());
    let generator = P::G1::generator();

    for c in 0..PRIVATE_VARIABLES_INDEX {
        result.push(
            generator.mul_bigint((c_tau_plus_beta_a_tau_plus_alpha_b_tau[c] / gamma).into_bigint()),
        );
    }

    result
}

pub fn generate_c_tau_plus_beta_a_tau_plus_alpha_b_tau_g1_private<P: Pairing>(
    c_tau_plus_beta_a_tau_plus_alpha_b_tau: &Vec<P::ScalarField>,
    delta: &P::ScalarField,
) -> Vec<P::G1> {
    let mut result = Vec::with_capacity(c_tau_plus_beta_a_tau_plus_alpha_b_tau.len());
    let generator = P::G1::generator();

    for c in PRIVATE_VARIABLES_INDEX..c_tau_plus_beta_a_tau_plus_alpha_b_tau.len() {
        result.push(
            generator.mul_bigint((c_tau_plus_beta_a_tau_plus_alpha_b_tau[c] / delta).into_bigint()),
        );
    }

    result
}

pub fn compute_l_i_of_tau_g1<P: Pairing>(
    a_poly_i: &UnivariantPolynomial<P::ScalarField>,
    b_poly_i: &UnivariantPolynomial<P::ScalarField>,
    c_poly_i: &UnivariantPolynomial<P::ScalarField>,
    alpha_t_g1: Vec<P::G1>,
    beta_t_g1: Vec<P::G1>,
    t_g1: Vec<P::G1>,
) -> P::G1 {
    let beta_a_i_of_tau = linear_combination_homomorphic_poly_eval_g1::<P>(a_poly_i, beta_t_g1);
    let alpha_b_i_of_tau = linear_combination_homomorphic_poly_eval_g1::<P>(b_poly_i, alpha_t_g1);
    let c_i_of_tau = linear_combination_homomorphic_poly_eval_g1::<P>(c_poly_i, t_g1);

    beta_a_i_of_tau + alpha_b_i_of_tau + c_i_of_tau
}

pub fn compute_delta_inverse_l_tau_g1<P: Pairing>(
    a_poly: &Vec<UnivariantPolynomial<P::ScalarField>>,
    b_poly: &Vec<UnivariantPolynomial<P::ScalarField>>,
    c_poly: &Vec<UnivariantPolynomial<P::ScalarField>>,
    alpha_t_g1: &Vec<P::G1>,
    beta_t_g1: &Vec<P::G1>,
    t_g1: &Vec<P::G1>,
    delta_inverse: &P::ScalarField,
    controlling_pin: usize,
    n: usize,
) -> Vec<P::G1> {
    let mut result = Vec::with_capacity(n);

    for i in controlling_pin..n {
        let l_p_of_tau_g1 = compute_l_i_of_tau_g1::<P>(
            &a_poly[i],
            &b_poly[i],
            &c_poly[i],
            alpha_t_g1.clone(),
            beta_t_g1.clone(),
            t_g1.clone(),
        );
        let l_p_of_tau_g1_delta_inverse = l_p_of_tau_g1.mul_bigint(delta_inverse.into_bigint());
        result.push(l_p_of_tau_g1_delta_inverse);
    }

    result
}

pub fn compute_t_of_tau_delta_inverse_g1<P: Pairing>(
    p_of_tau: &Vec<P::G1>,
    t_poly: &UnivariantPolynomial<P::ScalarField>,
    delta_inverse: &P::ScalarField,
    n: usize,
) -> Vec<P::G1> {
    let mut result = Vec::with_capacity(n);
    let t_of_tau_g1 = linear_combination_homomorphic_poly_eval_g1::<P>(&t_poly, p_of_tau.clone());

    for i in p_of_tau {
        let hold = *i + t_of_tau_g1;
        result.push(hold.mul_bigint(delta_inverse.into_bigint()));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_ec::AffineRepr;
    use ark_test_curves::bls12_381::Fr;
    use polynomial::interface::PolynomialInterface;

    #[test]
    fn test_generate_t_poly() {
        // t(x) = (x-1)(x-2)
        let number_of_constraints = 2;
        let t = generate_t_poly::<Fr>(number_of_constraints);

        let expected_t = UnivariantPolynomial::from_coefficients_vec(vec![
            Fr::from(2),
            Fr::from(-3),
            Fr::from(1),
        ]);

        assert_eq!(t, expected_t);
    }

    #[test]
    fn test_generate_t_poly_0() {
        // t(x) = (x-1)(x-2)(x-3)
        let number_of_constraints = 3;
        let t = generate_t_poly::<Fr>(number_of_constraints);

        let expected_t = UnivariantPolynomial::from_coefficients_vec(vec![
            Fr::from(-6),
            Fr::from(11),
            Fr::from(-6),
            Fr::from(1),
        ]);

        assert_eq!(t, expected_t);
    }

    #[test]
    fn test_linear_combination_homomorphic_poly_eval_g1() {
        let powers_of_tau_g1 =
            generate_powers_of_tau_g1::<ark_test_curves::bls12_381::Bls12_381>(Fr::from(5u64), 3);
        // f(tau).G1 when tau = 5 is know and f(x) = 2x^2 + 3x + 1
        let poly = UnivariantPolynomial::from_coefficients_vec(vec![
            Fr::from(1),
            Fr::from(3),
            Fr::from(2),
        ]);
        let res = linear_combination_homomorphic_poly_eval_g1::<
            ark_test_curves::bls12_381::Bls12_381,
        >(&poly, powers_of_tau_g1);

        let generator = ark_test_curves::bls12_381::g1::G1Affine::generator();
        let poly_at_tau = poly.evaluate(&Fr::from(5u64));
        let expected_res = generator.mul_bigint(poly_at_tau.into_bigint());

        assert_eq!(res, expected_res);
    }

    #[test]
    fn test_compute_l_i_of_tau_g1() {
        let a_i = UnivariantPolynomial::from_coefficients_vec(vec![Fr::from(1), Fr::from(2)]);
        let b_i = UnivariantPolynomial::from_coefficients_vec(vec![Fr::from(3), Fr::from(4)]);
        let c_i = UnivariantPolynomial::from_coefficients_vec(vec![Fr::from(5), Fr::from(6)]);

        let alpha = Fr::from(5);
        let beta = Fr::from(7);
        let tau = Fr::from(4);

        let alpha_t_g1 = generate_powers_of_tau_g1_alpha_or_beta::<
            ark_test_curves::bls12_381::Bls12_381,
        >(tau, alpha, 3);
        let beta_t_g1 = generate_powers_of_tau_g1_alpha_or_beta::<
            ark_test_curves::bls12_381::Bls12_381,
        >(tau, beta, 3);
        let t_g1 = generate_powers_of_tau_g1::<ark_test_curves::bls12_381::Bls12_381>(tau, 3);

        let l_of_i = (a_i.clone() * beta) + (b_i.clone() * alpha) + c_i.clone();
        let l_of_i_at_tau = l_of_i.evaluate(&tau);

        let generator_1 = ark_test_curves::bls12_381::g1::G1Affine::generator();

        let expected_res = generator_1.mul_bigint(l_of_i_at_tau.into_bigint());

        let res = compute_l_i_of_tau_g1::<ark_test_curves::bls12_381::Bls12_381>(
            &a_i, &b_i, &c_i, alpha_t_g1, beta_t_g1, t_g1,
        );

        assert_eq!(res, expected_res);
    }

    #[test]
    fn test_compute_t_of_tau_delta_inverse_g1() {
        let delta_inverse = Fr::from(7);
        let tau = Fr::from(4);
        let t_poly = UnivariantPolynomial::from_coefficients_vec(vec![
            Fr::from(1),
            Fr::from(3),
            Fr::from(2),
        ]);
        let p_of_tau = generate_powers_of_tau_g1::<ark_test_curves::bls12_381::Bls12_381>(tau, 3);
        let t_of_tau = t_poly.evaluate(&tau);
        let mut result = Vec::new();

        for i in &p_of_tau {
            let hold = i.mul_bigint(delta_inverse.into_bigint());
            result.push(hold.mul_bigint(t_of_tau.into_bigint()));
        }

        let res = compute_t_of_tau_delta_inverse_g1::<ark_test_curves::bls12_381::Bls12_381>(
            &p_of_tau,
            &t_poly,
            &delta_inverse,
            3,
        );

        assert_eq!(res, result);
    }
}
