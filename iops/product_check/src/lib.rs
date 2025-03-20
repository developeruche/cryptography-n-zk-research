//! This module contains implementations of product check protocol.
use ark_ec::pairing::Pairing;
use ark_ff::{One, Zero};
use fiat_shamir::{FiatShamirTranscript, TranscriptInterface};
use interface::ProductCheckInterface;
use pcs::{
    interface::KZGMultiLinearInterface, kzg::multilinear::MultilinearKZG,
    primitives::MultiLinearSRS,
};
use polynomial::{
    composed::multilinear::ComposedMultilinear, interface::MultilinearPolynomialInterface,
    multilinear::Multilinear,
};
use primitives::ProductCheckProof;
use utils::{generate_fractional_polynomial, generate_product_poly, perform_zero_check_protocol};
use zero_check::{ZeroCheck, interface::ZeroCheckInterface};
pub mod interface;
pub mod multilinear;
pub mod primitives;
pub mod utils;

/// Struct used to instantiate product check protocol.
pub struct ProductCheck<P: Pairing> {
    _marker: std::marker::PhantomData<P>,
}

impl<P: Pairing> ProductCheckInterface for ProductCheck<P> {
    type Poly = ComposedMultilinear<P::ScalarField>;
    type KZGSRS = MultiLinearSRS<P>;
    type Transcript = FiatShamirTranscript;
    type Proof = ProductCheckProof<P>;
    type Multilinear = Multilinear<P::ScalarField>;
    type FinalQueryAndEval = (Vec<P::ScalarField>, P::ScalarField, P::ScalarField);

    fn prove(
        poly_1: &Self::Poly,
        poly_2: &Self::Poly,
        kzg_srs: &Self::KZGSRS,
        transcript: &mut Self::Transcript,
    ) -> Result<
        (
            Self::Proof,
            Self::Multilinear,
            Self::Multilinear,
            Self::Poly,
        ),
        anyhow::Error,
    > {
        // performing some sanity checks
        // Dimension of the composed polynomials is same
        if poly_1.polys.len() != poly_2.polys.len() {
            return Err(anyhow::anyhow!("Polynomials have different lengths"));
        }

        let fractional_poly = generate_fractional_polynomial(poly_1, poly_2);
        let product_poly = generate_product_poly(&fractional_poly);

        // making polynomial commit rather than oracles query
        let fractional_poly_commitment = MultilinearKZG::commit(&kzg_srs, &fractional_poly);
        let product_poly_commitment = MultilinearKZG::commit(&kzg_srs, &product_poly);

        // appending commitments to transcript
        transcript.append_with_label(
            "fractional_poly_commitment",
            fractional_poly_commitment.to_string().as_bytes().to_vec(),
        );
        transcript.append_with_label(
            "product_poly_commitment",
            product_poly_commitment.to_string().as_bytes().to_vec(),
        );

        let alpha: P::ScalarField = transcript.sample_as_field_element();

        let (zero_check_proof, q_x) = perform_zero_check_protocol(
            poly_1,
            poly_2,
            &fractional_poly,
            &product_poly,
            &alpha,
            transcript,
        )?;

        Ok((
            ProductCheckProof {
                zero_check_proof,
                product_poly_commitment,
                fractional_poly_commitment,
            },
            product_poly,
            fractional_poly,
            q_x,
        ))
    }

    fn verify(
        proof: &Self::Proof,
        q_x: &Self::Poly,
        transcript: &mut Self::Transcript,
    ) -> Result<Self::FinalQueryAndEval, anyhow::Error> {
        // appending commitments to transcript
        transcript.append_with_label(
            "fractional_poly_commitment",
            proof
                .fractional_poly_commitment
                .to_string()
                .as_bytes()
                .to_vec(),
        );
        transcript.append_with_label(
            "product_poly_commitment",
            proof
                .product_poly_commitment
                .to_string()
                .as_bytes()
                .to_vec(),
        );
        let alpha: P::ScalarField = transcript.sample_as_field_element();

        if !ZeroCheck::verify(&proof.zero_check_proof, q_x, transcript)? {
            return Err(anyhow::anyhow!("ZeroCheck failed"));
        }

        let mut final_query = vec![P::ScalarField::one(); q_x.num_vars()];
        final_query[q_x.num_vars() - 1] = P::ScalarField::zero();
        let final_eval = P::ScalarField::one();

        Ok((final_query, final_eval, alpha))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_test_curves::bls12_381::{Bls12_381, Fr};

    // fn test_product_check(nv: usize) -> Result<(), PolyIOPErrors> {
    //     let mut rng = test_rng();

    //     let f1: DenseMultilinearExtension<Fr> = DenseMultilinearExtension::rand(nv, &mut rng);
    //     let mut g1 = f1.clone();
    //     g1.evaluations.reverse();
    //     let f2: DenseMultilinearExtension<Fr> = DenseMultilinearExtension::rand(nv, &mut rng);
    //     let mut g2 = f2.clone();
    //     g2.evaluations.reverse();
    //     let fs = vec![Arc::new(f1), Arc::new(f2)];
    //     let gs = vec![Arc::new(g2), Arc::new(g1)];
    //     let mut hs = vec![];
    //     for _ in 0..fs.len() {
    //         hs.push(Arc::new(DenseMultilinearExtension::rand(
    //             fs[0].num_vars,
    //             &mut rng,
    //         )));
    //     }

    //     let srs = MultilinearKzgPCS::<Bls12_381>::gen_srs_for_testing(&mut rng, nv)?;
    //     let (pcs_param, _) = MultilinearKzgPCS::<Bls12_381>::trim(&srs, None, Some(nv))?;

    //     test_product_check_helper::<Bls12_381, MultilinearKzgPCS<Bls12_381>>(
    //         &fs, &gs, &hs, &pcs_param,
    //     )?;

    //     Ok(())
    // }

    #[test]
    fn test_product_check() {
        // let num_vars = 4;
        // let f1 = Multilinear::<Fr>::random(num_vars);
        // let f2 = Multilinear::<Fr>::random(num_vars);

        // let mut g1 = f1.clone();
        // let mut g2 = f2.clone();

        // g1.evaluations.reverse();
        // g2.evaluations.reverse();

        // let poly_1 = ComposedMultilinear::new(vec![f1, f2]);
        // let poly_2 = ComposedMultilinear::new(vec![g1, g2]);
        // let poly_3 = ComposedMultilinear::new(vec![Multilinear::<Fr>::random(num_vars), Multilinear::<Fr>::random(num_vars)]);

        // let srs: MultiLinearSRS<Bls12_381> =
        //     MultilinearKZG::generate_srs(&[Fr::from(5u32), Fr::from(7u32), Fr::from(11u32)]);

        // test_product_check_helper::<Bls12_381>(
        //     &poly_1, &poly_2, &poly_3, &srs
        // );
        let f1 = Multilinear::new(
            vec![
                Fr::from(2),
                Fr::from(2),
                Fr::from(5),
                Fr::from(5),
                Fr::from(6),
                Fr::from(9),
                Fr::from(9),
                Fr::from(14),
            ],
            3,
        );
        let f2 = Multilinear::new(
            vec![
                Fr::from(12),
                Fr::from(2),
                Fr::from(5),
                Fr::from(5),
                Fr::from(16),
                Fr::from(9),
                Fr::from(19),
                Fr::from(14),
            ],
            3,
        );

        let mut g1 = f1.clone();
        let mut g2 = f2.clone();

        g1.evaluations.reverse();
        g2.evaluations.reverse();

        let poly_1 = ComposedMultilinear::new(vec![f1, f2]);
        let poly_2 = ComposedMultilinear::new(vec![g1, g2]);

        let test_fractional_poly = generate_fractional_polynomial(&poly_1, &poly_2);
        let product_poly = generate_product_poly(&test_fractional_poly);

        let alpha = Fr::from(10);

        let mut transcript = FiatShamirTranscript::default();

        let (zero_check_proof, q_x) = perform_zero_check_protocol(
            &poly_1,
            &poly_2,
            &test_fractional_poly,
            &product_poly,
            &alpha,
            &mut transcript,
        )
        .unwrap();

        // println!("q(x): {:?}", q_x);
    }

    fn check_fractional_poly<P: Pairing>(
        fractional_poly: &Multilinear<P::ScalarField>,
        poly_1: &ComposedMultilinear<P::ScalarField>,
        poly_2: &ComposedMultilinear<P::ScalarField>,
    ) {
        let mut flag = true;
        let num_vars = fractional_poly.num_vars;

        for i in 0..1 << num_vars {
            let nom = poly_1
                .polys
                .iter()
                .fold(P::ScalarField::one(), |acc, f| acc * f.evaluations[i]);
            let denom = poly_2
                .polys
                .iter()
                .fold(P::ScalarField::one(), |acc, f| acc * f.evaluations[i]);

            if denom * fractional_poly.evaluations[i] != nom {
                flag = false;
                break;
            }
        }

        assert!(flag);
    }

    fn test_product_check_helper<P: Pairing>(
        poly_1: &ComposedMultilinear<P::ScalarField>,
        poly_2: &ComposedMultilinear<P::ScalarField>,
        poly_3: &ComposedMultilinear<P::ScalarField>,
        srs: &MultiLinearSRS<P>,
    ) {
        let mut transcript = FiatShamirTranscript::default();
        let (proof, product_poly, fractional_poly, q_x) =
            ProductCheck::prove(poly_1, poly_2, srs, &mut transcript).unwrap();

        check_fractional_poly::<P>(&fractional_poly, poly_1, poly_2);

        let mut transcript_ = FiatShamirTranscript::default();
        let (final_query, final_eval, _alpha) =
            ProductCheck::verify(&proof, &q_x, &mut transcript_).unwrap();

        assert_eq!(
            product_poly.evaluate(&final_query).unwrap(),
            final_eval,
            "Wrong product detected"
        );

        // test bad poly case (poly_1 nd poly_3)
        let mut transcript = FiatShamirTranscript::default();
        let (proof, product_poly, fractional_poly, q_x) =
            ProductCheck::prove(poly_1, poly_3, srs, &mut transcript).unwrap();

        let mut transcript_ = FiatShamirTranscript::default();
        let (final_query, final_eval, _alpha) =
            ProductCheck::verify(&proof, &q_x, &mut transcript_).unwrap();

        assert_eq!(
            product_poly.evaluate(&final_query).unwrap(),
            final_eval,
            "Can't detect wrong product"
        );

        check_fractional_poly::<P>(&fractional_poly, poly_1, poly_3);
    }
}
