//! This file contains the benchmarking code for the kzg protocol.
use ark_bls12_381::{Bls12_381, Fr};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use kzg_rust::{
    interface::{self, KZGMultiLinearInterface, KZGUnivariateInterface},
    multilinear::MultilinearKZG,
    primitives::{MultiLinearSRS, SRS},
    univariate::UnivariateKZG,
};
use polynomial::{multilinear::Multilinear, univariant::UnivariantPolynomial};

fn kzg_multilinear_protocol_benchmark(c: &mut Criterion) {
    let poly = Multilinear::random(10);
    let point = vec![
        Fr::from(2),
        Fr::from(3),
        Fr::from(4),
        Fr::from(5),
        Fr::from(6),
        Fr::from(7),
        Fr::from(8),
        Fr::from(9),
        Fr::from(10),
        Fr::from(11),
    ];
    let srs: MultiLinearSRS<Bls12_381> = black_box(MultilinearKZG::generate_srs(&[
        Fr::from(5u32),
        Fr::from(7u32),
        Fr::from(11u32),
        Fr::from(13u32),
        Fr::from(17u32),
        Fr::from(19u32),
        Fr::from(23u32),
        Fr::from(29u32),
        Fr::from(31u32),
        Fr::from(37u32),
    ]));

    c.bench_function("KZG protocol Multilinear FULL PCS", |b| {
        b.iter(|| {
            let commitment = MultilinearKZG::commit(&srs, &poly);
            let (point_evaluation, proof) = MultilinearKZG::open(&srs, &poly, &point);
            let result =
                MultilinearKZG::verify(&srs, &commitment, &point, &point_evaluation, &proof);
            assert_eq!(result, true);
        })
    });
}

fn kzg_multilinear_protocol_benchmark_only_commit(c: &mut Criterion) {
    let poly = Multilinear::<Fr>::random(10);
    let _ = vec![
        Fr::from(2),
        Fr::from(3),
        Fr::from(4),
        Fr::from(5),
        Fr::from(6),
        Fr::from(7),
        Fr::from(8),
        Fr::from(9),
        Fr::from(10),
        Fr::from(11),
    ];
    let srs: MultiLinearSRS<Bls12_381> = black_box(MultilinearKZG::generate_srs(&[
        Fr::from(5u32),
        Fr::from(7u32),
        Fr::from(11u32),
        Fr::from(13u32),
        Fr::from(17u32),
        Fr::from(19u32),
        Fr::from(23u32),
        Fr::from(29u32),
        Fr::from(31u32),
        Fr::from(37u32),
    ]));

    c.bench_function("KZG protocol Multilinear ONLY commit PCS", |b| {
        b.iter(|| {
            let _ = MultilinearKZG::commit(&srs, &poly);
        })
    });
}

fn kzg_multilinear_protocol_benchmark_only_open(c: &mut Criterion) {
    let poly = Multilinear::random(10);
    let point = vec![
        Fr::from(2),
        Fr::from(3),
        Fr::from(4),
        Fr::from(5),
        Fr::from(6),
        Fr::from(7),
        Fr::from(8),
        Fr::from(9),
        Fr::from(10),
        Fr::from(11),
    ];
    let srs: MultiLinearSRS<Bls12_381> = black_box(MultilinearKZG::generate_srs(&[
        Fr::from(5u32),
        Fr::from(7u32),
        Fr::from(11u32),
        Fr::from(13u32),
        Fr::from(17u32),
        Fr::from(19u32),
        Fr::from(23u32),
        Fr::from(29u32),
        Fr::from(31u32),
        Fr::from(37u32),
    ]));

    let commitment = MultilinearKZG::commit(&srs, &poly);

    c.bench_function("KZG protocol Multilinear ONLY open PCS", |b| {
        b.iter(|| {
            let (point_evaluation, proof) = MultilinearKZG::open(&srs, &poly, &point);
            let result =
                MultilinearKZG::verify(&srs, &commitment, &point, &point_evaluation, &proof);
            assert_eq!(result, true);
        })
    });
}


criterion_group!(
    benches,
    kzg_multilinear_protocol_benchmark,
    kzg_multilinear_protocol_benchmark_only_commit,
    kzg_multilinear_protocol_benchmark_only_open,
);

criterion_main!(benches);
