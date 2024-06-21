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
pub fn linear_combination_homomorphic_poly_eval<P>(
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

#[cfg(test)]
mod tests {
    use super::*;
    use ark_test_curves::bls12_381::Fr;

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
}
