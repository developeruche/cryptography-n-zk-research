use ark_ff::PrimeField;
use polynomial::univariant::UnivariantPolynomial;

pub fn apply_w_to_polynomial<F: PrimeField>(
    poly: &UnivariantPolynomial<F>,
    w: &F,
) -> UnivariantPolynomial<F> {
    let mut result = Vec::new();
    let mut w_power = F::one();
    for coeff in poly.coefficients.iter() {
        result.push(*coeff * w_power);
        w_power *= w;
    }

    UnivariantPolynomial::new(result)
}

pub fn split_poly_in_3<F: PrimeField>(
    poly: &UnivariantPolynomial<F>,
    circuit_group_size: usize,
) -> (
    UnivariantPolynomial<F>,
    UnivariantPolynomial<F>,
    UnivariantPolynomial<F>,
) {
    let poly_1_coeffs = poly.coefficients[..circuit_group_size].to_vec();
    let poly_2_coeffs = poly.coefficients[circuit_group_size..2 * circuit_group_size].to_vec();
    let poly_3_coeffs = poly.coefficients[2 * circuit_group_size..].to_vec();

    (
        UnivariantPolynomial::new(poly_1_coeffs),
        UnivariantPolynomial::new(poly_2_coeffs),
        UnivariantPolynomial::new(poly_3_coeffs),
    )
}

#[cfg(test)]
pub mod test {
    use ark_test_curves::bls12_381::Fr;
    use polynomial::interface::PolynomialInterface;

    use super::*;

    #[test]
    fn test_apply_w_to_polynomial() {
        let poly = UnivariantPolynomial::<Fr>::new(vec![
            Fr::from(1u8),
            Fr::from(1u8),
            Fr::from(2u8),
            Fr::from(3u8),
            Fr::from(5u8),
        ]);
        let w = Fr::from(3u8);
        let x = Fr::from(2u8);
        let poly_eval = poly.evaluate(&x);

        assert_eq!(poly_eval, Fr::from(115));

        let poly_w = apply_w_to_polynomial(&poly, &w);
        let poly_w_eval = poly_w.evaluate(&x);

        assert_eq!(poly_w_eval, Fr::from(7207));
    }
}
