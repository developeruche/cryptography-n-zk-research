use ark_ec::{pairing::Pairing, Group};
use ark_ff::PrimeField;
use polynomial::{interface::UnivariantPolynomialInterface, univariant::UnivariantPolynomial};

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
}
