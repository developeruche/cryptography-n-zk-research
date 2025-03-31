#[cfg(test)]
mod integration_tests {
    use ark_ff::Zero;
    use ark_std::vec::Vec;
    use fiat_shamir::FiatShamirTranscript;
    use polynomial::multilinear::Multilinear;

    use crate::{
        a1_prover::IPForMLSumcheck,
        primitives::{LinearLagrangeList, ProverState, SumcheckProof},
    };

    type F = ark_test_curves::bls12_381::Fr;

    #[test]
    fn test_sumcheck() {
        // Define the combine function
        fn combine_fn(data: &Vec<F>) -> F {
            assert!(data.len() > 0);
            data[0]
        }

        // Take a simple polynomial
        let num_variables = 3;
        let num_evaluations = (1 as u32) << num_variables;
        let evaluations: Vec<F> = (0..num_evaluations).map(|i| F::from(2 * i)).collect();
        let claimed_sum = evaluations.iter().fold(F::zero(), |acc, e| acc + e);
        let poly = Multilinear::<F>::new(evaluations, num_variables);

        let polynomials: Vec<LinearLagrangeList<F>> = vec![LinearLagrangeList::<F>::from(&poly)];
        let mut prover_state: ProverState<F> = IPForMLSumcheck::prover_init(&polynomials, 1);

        // create a proof
        let mut prover_transcript = FiatShamirTranscript::default();
        let proof: SumcheckProof<F> = IPForMLSumcheck::<F>::prove::<_>(
            &mut prover_state,
            &combine_fn,
            &mut prover_transcript,
        );

        let mut verifier_transcript = FiatShamirTranscript::default();
        let result = IPForMLSumcheck::verify(claimed_sum, &proof, &mut verifier_transcript);
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn test_product_sumcheck() {
        // Define the combine function
        fn combine_fn(data: &Vec<F>) -> F {
            assert!(data.len() == 2);
            data[0] * data[1]
        }

        // Take two simple polynomial
        let num_variables = 3;
        let num_evaluations = (1 as u32) << num_variables;
        let evaluations_a: Vec<F> = (0..num_evaluations).map(|i| F::from(2 * i)).collect();
        let evaluations_b: Vec<F> = (0..num_evaluations).map(|i| F::from(i + 1)).collect();
        let claimed_sum = evaluations_a
            .iter()
            .zip(evaluations_b.iter())
            .fold(F::zero(), |acc, (a, b)| acc + a * b);
        let poly_a = Multilinear::<F>::new(evaluations_a, num_variables);
        let poly_b = Multilinear::<F>::new(evaluations_b, num_variables);

        let polynomials: Vec<LinearLagrangeList<F>> = vec![
            LinearLagrangeList::<F>::from(&poly_a),
            LinearLagrangeList::<F>::from(&poly_b),
        ];
        let mut prover_state: ProverState<F> = IPForMLSumcheck::prover_init(&polynomials, 2);
        let mut prover_transcript = FiatShamirTranscript::default();
        let proof: SumcheckProof<F> = IPForMLSumcheck::<F>::prove::<_>(
            &mut prover_state,
            &combine_fn,
            &mut prover_transcript,
        );

        let mut verifier_transcript = FiatShamirTranscript::default();
        let result = IPForMLSumcheck::verify(claimed_sum, &proof, &mut verifier_transcript);
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn test_r1cs_sumcheck() {
        // Define the combine function for r1cs: (a * b * e) - (c * e) = 0
        fn combine_fn(data: &Vec<F>) -> F {
            assert!(data.len() == 4);
            data[0] * data[1] * data[3] - data[2] * data[3]
        }

        // Take four simple polynomial
        const NV: usize = 10;
        let poly_a: Multilinear<F> = Multilinear::random(NV);
        let poly_b: Multilinear<F> = Multilinear::random(NV);
        let poly_c: Multilinear<F> = Multilinear::new(
            poly_a
                .evaluations
                .iter()
                .zip(poly_b.evaluations.iter())
                .map(|(a, b)| a * b)
                .collect(),
            NV,
        );
        let poly_e: Multilinear<F> = Multilinear::random(NV);
        let claimed_sum: F = F::zero();

        let polynomials: Vec<LinearLagrangeList<F>> = vec![
            LinearLagrangeList::<F>::from(&poly_a),
            LinearLagrangeList::<F>::from(&poly_b),
            LinearLagrangeList::<F>::from(&poly_c),
            LinearLagrangeList::<F>::from(&poly_e),
        ];
        let mut prover_state: ProverState<F> = IPForMLSumcheck::prover_init(&polynomials, 3);
        let mut prover_transcript = FiatShamirTranscript::default();
        let proof: SumcheckProof<F> = IPForMLSumcheck::<F>::prove::<_>(
            &mut prover_state,
            &combine_fn,
            &mut prover_transcript,
        );

        let mut verifier_transcript = FiatShamirTranscript::default();
        let result = IPForMLSumcheck::verify(claimed_sum, &proof, &mut verifier_transcript);
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn test_grk_sumcheck() {
        // Defines the combination for the multivariate polynomial, the sumcheck protocol is ran on in the GKR protocol.
        fn combine_fn(data: &Vec<F>) -> F {
            assert!(data.len() == 4);
            (data[0] * data[1]) + (data[2] * data[3])
        }

        const NV: usize = 10;
        let poly_a: Multilinear<F> = Multilinear::random(NV);
        let poly_b: Multilinear<F> = Multilinear::random(NV);
        let poly_c: Multilinear<F> = Multilinear::random(NV);
        let poly_d: Multilinear<F> = Multilinear::random(NV);
        let claimed_sum = poly_a
            .evaluations
            .iter()
            .zip(poly_b.evaluations.iter())
            .zip(poly_c.evaluations.iter())
            .zip(poly_d.evaluations.iter())
            .fold(F::zero(), |acc, (((a, b), c), d)| acc + ((a * b) + (c * d)));

        let polynomials: Vec<LinearLagrangeList<F>> = vec![
            LinearLagrangeList::<F>::from(&poly_a),
            LinearLagrangeList::<F>::from(&poly_b),
            LinearLagrangeList::<F>::from(&poly_c),
            LinearLagrangeList::<F>::from(&poly_d),
        ];

        let mut prover_state: ProverState<F> = IPForMLSumcheck::prover_init(&polynomials, 2);
        let mut prover_transcript = FiatShamirTranscript::default();
        let proof: SumcheckProof<F> = IPForMLSumcheck::<F>::prove::<_>(
            &mut prover_state,
            &combine_fn,
            &mut prover_transcript,
        );

        let mut verifier_transcript = FiatShamirTranscript::default();
        let result = IPForMLSumcheck::verify(claimed_sum, &proof, &mut verifier_transcript);
        assert_eq!(result.unwrap(), true);
    }
}
