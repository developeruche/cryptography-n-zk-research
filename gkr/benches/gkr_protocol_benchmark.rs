//! This file contains the benchmarking code for the gkr protocol.
use ark_test_curves::bls12_381::Fr;
use circuits::{interfaces::CircuitInterface, primitives::Circuit};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gkr::{interfaces::GKRProtocolInterface, protocol::GKRProtocol};

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

criterion_group!(
    benches,
    gkr_protocol_benchmark,
    gkr_protocol_benchmark_without_eval
);

criterion_main!(benches);
