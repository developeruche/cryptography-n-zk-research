use ark_ec::{pairing::Pairing, Group};
use ark_ff::PrimeField;
use polynomial::univariant::UnivariantPolynomial;

/// This function provides a way to multiply coefficients of a polynomial and a group element vector elementwise\
/// this mathematically  provides a way to `[poly(x)]_1`
pub fn linear_combination_homomorphic_poly_eval_g1<P, F>(
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

/// This function generates the powers of tau for the circuit
/// in the group G1
///
/// examples;
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

/// This function generates the powers of tau for the circuit
/// in the group G2
pub fn generate_powers_of_tau_g2<P: Pairing>(tau: &P::ScalarField, n: usize) -> Vec<P::G2> {
    let n = n + 1;
    let mut powers_of_tau_g2 = Vec::with_capacity(n);
    let mut tau_power = *tau;
    let generator = P::G2::generator();

    powers_of_tau_g2.push(generator);

    for _ in 1..n {
        powers_of_tau_g2.push(generator.mul_bigint(tau_power.into_bigint()));
        tau_power = tau_power * *tau;
    }

    powers_of_tau_g2
}

/// This function provides a way to multiply coefficients of a polynomial and a group element vector elementwise
/// this mathematically  provides a way to `[poly(x)]_1`
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

/// This function provides a way to multiply coefficients of a polynomial and a group element vector elementwise
/// this mathematically  provides a way to `[poly(x)]_2`
pub fn linear_combination_homomorphic_poly_eval_g2_primefield<P, F>(
    poly: &UnivariantPolynomial<F>,
    powers_of_secret_gx: &[P::G2],
) -> P::G2
where
    P: Pairing,
    F: PrimeField,
{
    poly.coefficients
        .iter()
        .enumerate()
        .fold(P::G2::default(), |mut acc, (index, coeff)| {
            let res = powers_of_secret_gx[index].mul_bigint(coeff.into_bigint());
            acc = acc + res;
            acc
        })
}

/// This function is used tp perform zero and one check on the given pattern and object
/// this is used for langrange interpolation for multilimear polynomial
pub fn perform_zero_and_one_check<F: PrimeField>(pattern: &[F], object: &[F]) -> F {
    let mut result = F::one();

    for (i, hypercube_element) in pattern.iter().enumerate() {
        if hypercube_element.is_zero() {
            result *= F::one() - object[i];
        } else {
            result *= object[i]
        }
    }

    result
}

/// This function generates the G1 SRS for the given hypercube and object
pub fn bh_to_g1_srs<F: PrimeField, P: Pairing>(bh: &[Vec<F>], object: &[F]) -> Vec<P::G1> {
    let mut srs = Vec::with_capacity(bh.len());
    let generator = P::G1::generator();

    for i in bh.iter() {
        let result = perform_zero_and_one_check(i, object); // langrange interpolation and evaluating the variable x immidiately
        srs.push(generator.mul_bigint(result.into_bigint()));
    }

    srs
}

/// This function performs the group operation of G2 on a vec of elements
pub fn g2_operation<F: PrimeField, P: Pairing>(oprands: &[F]) -> Vec<P::G2> {
    let mut result = Vec::with_capacity(oprands.len());
    let generator = P::G2::generator();

    for i in oprands.iter() {
        result.push(generator.mul_bigint(i.into_bigint()))
    }

    result
}

/// This function is used to create a vanishing polynomial for the given data
pub fn generate_vanishing_polynomial<F: PrimeField>(data: &Vec<F>) -> UnivariantPolynomial<F> {
    let mut v_poly = UnivariantPolynomial::one();

    for c in data {
        v_poly *= UnivariantPolynomial::new(vec![-*c, F::one()]);
    }

    v_poly
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_test_curves::bls12_381::Fr;

    fn generator_operation<F: PrimeField, P: Pairing>(oprands: &[F]) -> Vec<P::G1> {
        let mut result = Vec::with_capacity(oprands.len());
        let generator = P::G1::generator();

        for i in oprands.iter() {
            result.push(generator.mul_bigint(i.into_bigint()))
        }

        result
    }

    #[test]
    fn test_perform_zero_and_one_check() {
        let object = vec![Fr::from(2u8), Fr::from(4u8)];

        let pattern_1 = vec![Fr::from(0u8), Fr::from(0u8)];
        let pattern_2 = vec![Fr::from(0u8), Fr::from(1u8)];
        let pattern_3 = vec![Fr::from(1u8), Fr::from(0u8)];
        let pattern_4 = vec![Fr::from(1u8), Fr::from(1u8)];

        let result_1 = perform_zero_and_one_check(&pattern_1, &object);
        let result_2 = perform_zero_and_one_check(&pattern_2, &object);
        let result_3 = perform_zero_and_one_check(&pattern_3, &object);
        let result_4 = perform_zero_and_one_check(&pattern_4, &object);

        assert_eq!(result_1, Fr::from(3u8));
        assert_eq!(result_2, Fr::from(-4));
        assert_eq!(result_3, Fr::from(-6));
        assert_eq!(result_4, Fr::from(8u8));
    }

    #[test]
    fn test_bh_to_g1_srs() {
        let object = vec![Fr::from(2u8), Fr::from(4u8)];
        let bh = vec![
            vec![Fr::from(0u8), Fr::from(0u8)],
            vec![Fr::from(0u8), Fr::from(1u8)],
            vec![Fr::from(1u8), Fr::from(0u8)],
            vec![Fr::from(1u8), Fr::from(1u8)],
        ];

        let srs = bh_to_g1_srs::<Fr, ark_bls12_381::Bls12_381>(&bh, &object);

        assert_eq!(srs.len(), 4);
        assert_eq!(
            srs,
            generator_operation::<Fr, ark_bls12_381::Bls12_381>(&[
                Fr::from(3u8),
                Fr::from(-4),
                Fr::from(-6),
                Fr::from(8u8)
            ])
        );
    }

    #[test]
    fn test_generate_vanishing_polynomial() {
        let data = vec![Fr::from(2u8), Fr::from(4u8)];
        let v_poly = generate_vanishing_polynomial::<Fr>(&data);

        assert_eq!(
            v_poly.coefficients,
            vec![Fr::from(8u8), Fr::from(-6), Fr::from(1u8)]
        );
    }
}
