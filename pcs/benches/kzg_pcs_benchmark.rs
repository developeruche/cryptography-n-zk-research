//! This file contains the benchmarking code for the kzg protocol.
use ark_bls12_381::{Bls12_381, Fr};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pcs::{
    interface::{self, KZGUnivariateInterface},
    kzg::univariate::UnivariateKZG,
    primitives::SRS,
};
use polynomial::univariant::UnivariantPolynomial;

const SIZE: usize = 256;

fn kzg_univariant_protocol_benchmark(c: &mut Criterion) {
    let tau = Fr::from(10u64);
    let poly_degree = SIZE;
    let poly = black_box(UnivariantPolynomial::random(SIZE));
    let srs: SRS<Bls12_381> = black_box(UnivariateKZG::generate_srs(&tau, poly_degree));

    c.bench_function("KZG protocol  FULL PCS", |b| {
        b.iter(|| {
            let commitment = UnivariateKZG::commit(&srs, &poly);
            let (point_evaluation, proof) = UnivariateKZG::open::<Fr>(&srs, &poly, &Fr::from(2u64));
            let is_valid = UnivariateKZG::verify::<Fr>(
                &srs,
                &commitment,
                &Fr::from(2u64),
                &point_evaluation,
                &proof,
            );

            assert!(is_valid);
        })
    });
}

fn kzg_univariant_protocol_benchmark_only_commit(c: &mut Criterion) {
    let tau = Fr::from(10u64);
    let poly_degree = SIZE;
    let poly: UnivariantPolynomial<Fr> = black_box(UnivariantPolynomial::random(SIZE));
    let srs: SRS<Bls12_381> = black_box(UnivariateKZG::generate_srs(&tau, poly_degree));

    c.bench_function("KZG protocol ONLY commitment", |b| {
        b.iter(|| {
            let _ = UnivariateKZG::commit(&srs, &poly);
        })
    });
}

fn kzg_univariant_protocol_benchmark_only_open(c: &mut Criterion) {
    let tau = Fr::from(10u64);
    let poly_degree = SIZE;
    let poly = black_box(UnivariantPolynomial::random(SIZE));
    let srs: SRS<Bls12_381> = black_box(UnivariateKZG::generate_srs(&tau, poly_degree));
    let commitment = UnivariateKZG::commit(&srs, &poly);

    c.bench_function("KZG protocol ONLY opening", |b| {
        b.iter(|| {
            let (point_evaluation, proof) = UnivariateKZG::open::<Fr>(&srs, &poly, &Fr::from(2u64));
            let is_valid = UnivariateKZG::verify::<Fr>(
                &srs,
                &commitment,
                &Fr::from(2u64),
                &point_evaluation,
                &proof,
            );

            assert!(is_valid);
        })
    });
}

fn kzg_univariant_protocol_benchmark_only_open_three_point(c: &mut Criterion) {
    let tau = Fr::from(10u64);
    let poly_degree = SIZE;
    let poly = black_box(UnivariantPolynomial::random(SIZE));
    let srs: SRS<Bls12_381> = black_box(UnivariateKZG::generate_srs(&tau, poly_degree));
    let commitment = UnivariateKZG::commit(&srs, &poly);

    c.bench_function("KZG protocol ONLY opening in 3 point", |b| {
        b.iter(|| {
            let (point_evaluation, proof) = UnivariateKZG::open::<Fr>(&srs, &poly, &Fr::from(2u64));
            let is_valid = UnivariateKZG::verify::<Fr>(
                &srs,
                &commitment,
                &Fr::from(2u64),
                &point_evaluation,
                &proof,
            );

            assert!(is_valid);

            let (point_evaluation, proof) = UnivariateKZG::open::<Fr>(&srs, &poly, &Fr::from(8u64));
            let is_valid = UnivariateKZG::verify::<Fr>(
                &srs,
                &commitment,
                &Fr::from(8u64),
                &point_evaluation,
                &proof,
            );

            assert!(is_valid);

            let (point_evaluation, proof) =
                UnivariateKZG::open::<Fr>(&srs, &poly, &Fr::from(12u64));
            let is_valid = UnivariateKZG::verify::<Fr>(
                &srs,
                &commitment,
                &Fr::from(12u64),
                &point_evaluation,
                &proof,
            );

            assert!(is_valid);
        })
    });
}

fn kzg_univariant_protocol_benchmark_only_open_three_point_batch(c: &mut Criterion) {
    let tau = Fr::from(10u64);
    let poly_degree = SIZE;
    let poly = black_box(UnivariantPolynomial::random(SIZE));
    let srs: SRS<Bls12_381> = black_box(UnivariateKZG::generate_srs(&tau, poly_degree));
    let commitment = UnivariateKZG::commit(&srs, &poly);

    c.bench_function("KZG protocol ONLY opening in 3 point using batch", |b| {
        b.iter(|| {
            let (point_evaluation, proof) =
                <UnivariateKZG as interface::BatchKZGUnivariateInterface<Bls12_381>>::open::<Fr>(
                    &srs,
                    &poly,
                    &vec![Fr::from(2u64), Fr::from(8u64), Fr::from(12u64)],
                );
            let is_valid =
                <UnivariateKZG as interface::BatchKZGUnivariateInterface<Bls12_381>>::verify::<Fr>(
                    &srs,
                    &commitment,
                    &vec![Fr::from(2u64), Fr::from(8u64), Fr::from(12u64)],
                    &point_evaluation,
                    &proof,
                );

            assert!(is_valid);
        })
    });
}

criterion_group!(
    benches,
    kzg_univariant_protocol_benchmark,
    kzg_univariant_protocol_benchmark_only_commit,
    kzg_univariant_protocol_benchmark_only_open,
    kzg_univariant_protocol_benchmark_only_open_three_point,
    kzg_univariant_protocol_benchmark_only_open_three_point_batch,
);

criterion_main!(benches);
