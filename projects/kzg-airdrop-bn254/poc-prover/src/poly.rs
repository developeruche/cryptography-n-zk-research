use ark_bn254::Fr;
use ark_ff::Field;
use ark_ff::One;
use ark_poly::{
    univariate::DensePolynomial, DenseUVPolynomial, EvaluationDomain, Evaluations,
    GeneralEvaluationDomain,
};
use std::ops::{Div, Sub};

use crate::error::{Error, Result};

// given [y0, y1, y3, ..., y_n-1], Returns the f(x) with (n-1) degree passing through points:
// (w^0, y0), (w^1, y1), ..., (w^n-1, y_n-1)
// n must < subfield size
pub fn poly(points: Vec<Fr>) -> Result<(DensePolynomial<Fr>, Fr)> {
    let domain = GeneralEvaluationDomain::<Fr>::new(points.len())
        .ok_or(Error::PolyError("no poly".to_string()))?;

    let res = Evaluations::from_vec_and_domain(points, domain).interpolate();
    Ok((res, domain.element(1)))
}

// calculate q(x) = (f(x) - yi) / (x - w^i)
pub fn cal_witness_poly(f: &DensePolynomial<Fr>, i: u64, y_i: Fr, w: Fr) -> DensePolynomial<Fr> {
    let q = f.sub(&DensePolynomial::from_coefficients_vec(vec![y_i]));

    let x = DensePolynomial::from_coefficients_vec(vec![-w.pow([i]), Fr::one()]);
    q.div(&x)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_ff::Field;
    use ark_poly::Polynomial;
    use std::str::FromStr;

    #[test]
    fn test_poly() {
        let points = vec![
            Fr::from(3u64),
            Fr::from(7u64),
            Fr::from(23u64),
            Fr::from(24u64),
        ];
        let (f, w) = poly(points).unwrap();
        println!("w: {:?}", w.to_string());
        assert_eq!(f.evaluate(&Fr::from(1u64)), Fr::from(3u64));
        assert_eq!(f.evaluate(&w), Fr::from(7u64));
        assert_eq!(f.evaluate(&w.pow([2])), Fr::from(23u64));
        assert_eq!(f.evaluate(&w.pow([3])), Fr::from(24u64));
    }

    // sage code:
    // r = 21888242871839275222246405745257275088548364400416034343698204186575808495617
    // Fr = GF(r)
    // w = Fr(21888242871839275217838484774961031246007050428528088939761107053157389710902)
    // R.<x> = PolynomialRing(Fr, 'x')
    // points = [(1,3),(w,7),(w^2,23), (w^3, 24)]
    // f = R.lagrange_polynomial(points)
    // p = (f-23)/(x-w^2)
    // p
    // 5472060717959818824295265560073355102937675480627776552657213863672231958938*x^2 +
    // 21888242871839275203512741621498238757747780019892266376965541369547528660582*x +
    // 16416182153879456416684804308942956316411273300312025757773653139931856371704
    #[test]
    fn test_vanish() {
        let points = vec![
            Fr::from(3u64),
            Fr::from(7u64),
            Fr::from(23u64),
            Fr::from(24u64),
        ];
        let (f, w) = poly(points).unwrap();

        let q = cal_witness_poly(&f, 2, Fr::from(23u64), w);
        assert_eq!(
            q,
            DensePolynomial::from_coefficients_vec(vec![
                Fr::from_str(
                    "16416182153879456416684804308942956316411273300312025757773653139931856371704"
                )
                .unwrap(),
                Fr::from_str(
                    "21888242871839275203512741621498238757747780019892266376965541369547528660582"
                )
                .unwrap(),
                Fr::from_str(
                    "5472060717959818824295265560073355102937675480627776552657213863672231958938"
                )
                .unwrap(),
            ])
        );
    }
}
