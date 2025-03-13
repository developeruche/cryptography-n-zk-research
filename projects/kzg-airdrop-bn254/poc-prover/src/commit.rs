use ark_bn254::{Bn254, Fr, G1Affine, G1Projective, G2Affine};
use ark_ec::{pairing::Pairing, AffineRepr, CurveGroup, VariableBaseMSM};
use ark_ff::{Field, PrimeField};
use ark_poly::{univariate::DensePolynomial, DenseUVPolynomial, Polynomial};
use ark_poly_commit::{
    kzg10::{Powers, Randomness, KZG10},
    PCRandomness,
};

pub struct CommitmentScheme<'a> {
    poly: &'a DensePolynomial<Fr>, // for quicker msm operation
    w: Fr,                         // root of unity
    powers: Powers<'a, Bn254>,     // srs_g1
    srs_g2: Vec<G2Affine>,
}

impl<'a> CommitmentScheme<'a> {
    // poly's length must be 2^k
    pub fn setup(
        poly: &'a DensePolynomial<Fr>,
        w: Fr,
        srs_g1: &'a [G1Affine],
        srs_g2: Vec<G2Affine>,
    ) -> Self {
        assert_eq!(
            poly.coeffs().len(),
            srs_g1.len(),
            "poly length is greater than srs_g1 length"
        );

        CommitmentScheme {
            poly,
            w,
            powers: Powers {
                powers_of_g: srs_g1.into(),
                powers_of_gamma_g: srs_g1.into(),
            },
            srs_g2,
        }
    }

    pub fn commit(&self) -> G1Affine {
        KZG10::commit(&self.powers, &self.poly.clone(), None, None)
            .unwrap()
            .0
             .0
    }

    pub fn open(&self, x_idx: u64) -> (Fr, G1Affine) {
        let x = self.w.pow([x_idx]);
        let y = self.poly.evaluate(&x);

        let witness_poly = KZG10::<Bn254, DensePolynomial<Fr>>::compute_witness_polynomial(
            self.poly,
            x,
            &Randomness::<Fr, DensePolynomial<Fr>>::empty(),
        )
        .unwrap()
        .0;

        let (num_leading_zeros, witness_coeffs) =
            skip_leading_zeros_and_convert_to_bigints(&witness_poly);

        let w = <G1Projective as VariableBaseMSM>::msm_bigint(
            &self.powers.powers_of_g[num_leading_zeros..],
            &witness_coeffs,
        );

        (y, w.into_affine())
    }

    // e(C- yG1, G2) = e(H, \tau G2 - xG2)
    pub fn verify(&self, commit: G1Affine, y: Fr, proof: G1Affine, x_idx: u64) -> bool {
        let x = self.w.pow([x_idx]);
        let y_g1: G1Affine = (G1Affine::generator() * y).into();
        let lhs = Bn254::pairing(commit - y_g1, G2Affine::generator());

        let x_g2: G2Affine = (G2Affine::generator() * x).into();
        let rhs = Bn254::pairing(proof, self.srs_g2[1] - x_g2);
        lhs == rhs
    }
}

fn skip_leading_zeros_and_convert_to_bigints<F: PrimeField, P: DenseUVPolynomial<F>>(
    p: &P,
) -> (usize, Vec<F::BigInt>) {
    let mut num_leading_zeros = 0;
    while num_leading_zeros < p.coeffs().len() && p.coeffs()[num_leading_zeros].is_zero() {
        num_leading_zeros += 1;
    }
    let coeffs = convert_to_bigints(&p.coeffs()[num_leading_zeros..]);
    (num_leading_zeros, coeffs)
}

fn convert_to_bigints<F: PrimeField>(p: &[F]) -> Vec<F::BigInt> {
    ark_std::cfg_iter!(p)
        .map(|s| s.into_bigint())
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::poly::poly;

    fn new_srs() -> (Vec<G1Affine>, Vec<G2Affine>) {
        let x = vec![1, 3, 9, 27, 81, 243, 729, 2187];
        let x = x.iter().map(|x| Fr::from(*x)).collect::<Vec<Fr>>();
        let g1 = x
            .iter()
            .map(|x| (G1Affine::generator() * x).into())
            .collect::<Vec<G1Affine>>();
        let g2 = vec![G2Affine::generator(), (G2Affine::generator() * x[1]).into()];
        (g1, g2)
    }

    #[test]
    fn test_commitment_scheme() {
        let points = vec![
            Fr::from(12u64),
            Fr::from(123u64),
            Fr::from(1234u64),
            Fr::from(12345u64),
            Fr::from(123456u64),
            Fr::from(1234567u64),
            Fr::from(12345678u64),
            Fr::from(123456789u64),
        ];

        let (f, w) = poly(points).unwrap();
        let srs = new_srs();
        let cs = CommitmentScheme::setup(&f, w, &srs.0, srs.1);

        let commit = cs.commit();

        for i in 0..8 {
            let (y, proof) = cs.open(i);

            assert!(cs.verify(commit, y, proof, i))
        }
    }
}
