//! This file contains the benchmarking code for the sum_check function.
//! 1. Normal sum check benchmarking
//! 2. Running sum check on a composed polynomial
//! 3. Running sum check on a multi-composed polynomial

use ark_test_curves::bls12_381::Fr;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fiat_shamir::FiatShamirTranscript;
use polynomial::{composed::multilinear::ComposedMultilinear, multilinear::Multilinear};
use sum_check::{
    composed::{
        multicomposed::{MultiComposedProver, MultiComposedVerifier},
        prover::ComposedProver,
        verifier::ComposedVerifier,
    },
    interface::{
        ComposedProverInterface, ComposedVerifierInterface, MultiComposedProverInterface,
        MultiComposedVerifierInterface, ProverInterface, VerifierInterface,
    },
    prover::Prover,
    verifier::Verifier,
};
use sumcheck201playground::{
    a1_prover::IPForMLSumcheck,
    primitives::{LinearLagrangeList, ProverState, SumcheckProof},
};

fn normal_sum_check_benchmark(c: &mut Criterion) {
    let poly = black_box(Multilinear::<Fr>::random(12));

    c.bench_function("Normal sum check", |b| {
        b.iter(|| {
            let mut transcript = FiatShamirTranscript::default();
            let sum = Prover::calculate_sum(&poly);
            let proof = Prover::sum_check_proof(&poly, &mut transcript, &sum);
            assert!(Verifier::verify(&proof));
        })
    });
}

fn composed_sum_check_benchmark(c: &mut Criterion) {
    let poly_1 = black_box(Multilinear::<Fr>::random(20));
    let poly_2 = black_box(Multilinear::<Fr>::random(20));
    let composed_poly = black_box(ComposedMultilinear::new(vec![poly_1, poly_2]));
    let sum = ComposedProver::calculate_sum(&composed_poly);

    c.bench_function("Composed sum check", |b| {
        b.iter(|| {
            let mut transcript = FiatShamirTranscript::default();
            let (proof, _) = ComposedProver::sum_check_proof(&composed_poly, &mut transcript, &sum);
            let mut transcript_ = FiatShamirTranscript::default();
            assert!(ComposedVerifier::verify(
                &proof,
                &composed_poly,
                &mut transcript_
            ));
        })
    });
}

fn combine_fn(data: &Vec<Fr>) -> Fr {
    assert!(data.len() == 2);
    data[0] * data[1]
}

fn super_sum_check_benchmark(c: &mut Criterion) {
    let poly_1 = black_box(Multilinear::<Fr>::random(20));
    let poly_2 = black_box(Multilinear::<Fr>::random(20));

    let polynomials: Vec<LinearLagrangeList<Fr>> = vec![
        LinearLagrangeList::<Fr>::from(&poly_1),
        LinearLagrangeList::<Fr>::from(&poly_2),
    ];

    let sum = poly_1
        .evaluations
        .iter()
        .zip(poly_2.evaluations.iter())
        .fold(Fr::from(0), |acc, (a, b)| acc + (a * b));

    c.bench_function("Super Sumcheck: Sumcheck201", |b| {
        b.iter(|| {
            let mut prover_state: ProverState<Fr> = IPForMLSumcheck::prover_init(&polynomials, 2);
            let mut prover_transcript = FiatShamirTranscript::default();
            let proof: SumcheckProof<Fr> = IPForMLSumcheck::<Fr>::prove::<_>(
                &mut prover_state,
                &combine_fn,
                &mut prover_transcript,
            );

            let mut verifier_transcript = FiatShamirTranscript::default();
            let result = IPForMLSumcheck::verify(sum, &proof, &mut verifier_transcript);
            assert_eq!(result.unwrap(), true);
        })
    });
}

fn multi_composed_sum_check_benchmark(c: &mut Criterion) {
    let poly_1 = black_box(Multilinear::<Fr>::random(12));
    let poly_2 = black_box(Multilinear::<Fr>::random(12));
    let poly_3 = black_box(Multilinear::<Fr>::random(12));
    let poly_4 = black_box(Multilinear::<Fr>::random(12));

    let composed_poly_1 = black_box(ComposedMultilinear::new(vec![poly_1, poly_2]));
    let composed_poly_2 = black_box(ComposedMultilinear::new(vec![poly_3, poly_4]));

    let multi_composed_poly = vec![composed_poly_1, composed_poly_2];

    c.bench_function("Multi-composed sum check", |b| {
        b.iter(|| {
            let sum = MultiComposedProver::calculate_sum(&multi_composed_poly);
            let (proof, _) = MultiComposedProver::sum_check_proof(&multi_composed_poly, &sum);
            assert!(MultiComposedVerifier::verify(&proof, &multi_composed_poly));
        })
    });
}

criterion_group!(
    benches,
    // multi_composed_sum_check_benchmark,
    composed_sum_check_benchmark,
    // normal_sum_check_benchmark,
    super_sum_check_benchmark
);

criterion_main!(benches);
