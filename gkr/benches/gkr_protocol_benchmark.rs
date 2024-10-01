//! This file contains the benchmarking code for the gkr protocol.
use ark_test_curves::bls12_381::Fr;
use circuits::{interfaces::CircuitInterface, primitives::Circuit};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use field_tracker::Ft;
use gkr::{interfaces::GKRProtocolInterface, protocol::GKRProtocol};

type Ftt = Ft<4, Fr>;

fn gkr_protocol_benchmark(c: &mut Criterion) {
    let circuit = black_box(Circuit::random(8));
    let input = black_box(
        (0u64..256)
            .into_iter()
            .map(|x| Fr::from(x))
            .collect::<Vec<Fr>>(),
    );

    c.bench_function("GKR protocol with evaluation", |b| {
        b.iter(|| {
            let evaluation = circuit.evaluate(&input);
            let proof = GKRProtocol::prove(&circuit, &evaluation);
            assert!(GKRProtocol::verify(&circuit, &input, &proof));
        })
    });
}

fn gkr_protocol_benchmark_with_field_ops_tracker(c: &mut Criterion) {
    let circuit = black_box(Circuit::random(3));
    let input = black_box(
        (0u64..8)
            .into_iter()
            .map(|x| Ftt::from(x))
            .collect::<Vec<Ftt>>(),
    );

    c.bench_function("GKR protocol with evaluation and Tracking Enabled", |b| {
        b.iter(|| {
            let evaluation = circuit.evaluate(&input);
            let proof = GKRProtocol::prove(&circuit, &evaluation);
            assert!(GKRProtocol::verify(&circuit, &input, &proof));

            println!("{}", Ftt::summary());
        })
    });
}

fn gkr_protocol_benchmark_without_eval(c: &mut Criterion) {
    let circuit = black_box(Circuit::random(8));
    let input = black_box(
        (0u64..256)
            .into_iter()
            .map(|x| Fr::from(x))
            .collect::<Vec<Fr>>(),
    );
    let evaluation = black_box(circuit.evaluate(&input));

    c.bench_function("GKR protocol without evaluation", |b| {
        b.iter(|| {
            let proof = GKRProtocol::prove(&circuit, &evaluation);
            assert!(GKRProtocol::verify(&circuit, &input, &proof));
        })
    });
}

fn gkr_protocol_benchmark_without_only_prove(c: &mut Criterion) {
    let circuit = black_box(Circuit::random(8));
    let input = black_box(
        (0u64..256)
            .into_iter()
            .map(|x| Fr::from(x))
            .collect::<Vec<Fr>>(),
    );
    let evaluation = black_box(circuit.evaluate(&input));

    c.bench_function("GKR protocol without evaluation only Prove", |b| {
        b.iter(|| {
            let _ = GKRProtocol::prove(&circuit, &evaluation);
        })
    });
}

fn gkr_protocol_benchmark_without_only_verify(c: &mut Criterion) {
    let circuit = black_box(Circuit::random(8));
    let input = black_box(
        (0u64..256)
            .into_iter()
            .map(|x| Fr::from(x))
            .collect::<Vec<Fr>>(),
    );
    let evaluation = black_box(circuit.evaluate(&input));
    let proof = GKRProtocol::prove(&circuit, &evaluation);

    c.bench_function("GKR protocol without evaluation Only Verify", |b| {
        b.iter(|| {
            assert!(GKRProtocol::verify(&circuit, &input, &proof));
        })
    });
}

criterion_group!(
    benches,
    // gkr_protocol_benchmark,
    // gkr_protocol_benchmark_without_eval,
    // gkr_protocol_benchmark_without_only_prove,
    // gkr_protocol_benchmark_without_only_verify,
    gkr_protocol_benchmark_with_field_ops_tracker
);

criterion_main!(benches);
