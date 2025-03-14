//! This file contains the benchmarking code for the gkr protocol.
use ark_test_curves::bls12_381::{Bls12_381, Fr};
use circuits::{interfaces::CircuitInterface, primitives::Circuit};
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use gkr::utils::gen_random_taus;
use pcs::{
    interface::KZGMultiLinearInterface, kzg::multilinear::MultilinearKZG,
    primitives::MultiLinearSRS,
};
use polynomial::{interface::MultilinearPolynomialInterface, multilinear::Multilinear};
use succinct_protocol::{SuccinctGKRProtocol, interfaces::SuccinctGKRProtocolInterface};

fn gkr_protocol_benchmark(c: &mut Criterion) {
    let circuit = black_box(Circuit::random(8));
    let input = black_box(
        (0u64..256)
            .into_iter()
            .map(|x| Fr::from(x))
            .collect::<Vec<Fr>>(),
    );

    c.bench_function("Succicnt GKR protocol with evaluation", |b| {
        b.iter(|| {
            let input_in_poly_form = Multilinear::interpolate(&input);
            let tau_s = gen_random_taus::<Fr>(input_in_poly_form.num_vars);
            let srs: MultiLinearSRS<Bls12_381> = MultilinearKZG::generate_srs(&tau_s);
            let commitment = MultilinearKZG::commit(&srs, &input_in_poly_form);

            let evaluation = circuit.evaluate(&input);

            let proof = SuccinctGKRProtocol::prove(&circuit, &evaluation, &commitment, &srs);

            assert!(SuccinctGKRProtocol::verify(
                &circuit,
                &proof,
                &commitment,
                &srs
            ));
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
    let input_in_poly_form = Multilinear::interpolate(&input);
    let tau_s = gen_random_taus::<Fr>(input_in_poly_form.num_vars);
    let srs: MultiLinearSRS<Bls12_381> = MultilinearKZG::generate_srs(&tau_s);
    let commitment = MultilinearKZG::commit(&srs, &input_in_poly_form);

    let evaluation = circuit.evaluate(&input);

    c.bench_function("Succicnt GKR protocol without evaluation", |b| {
        b.iter(|| {
            let proof = SuccinctGKRProtocol::prove(&circuit, &evaluation, &commitment, &srs);

            assert!(SuccinctGKRProtocol::verify(
                &circuit,
                &proof,
                &commitment,
                &srs
            ));
        })
    });
}

fn gkr_protocol_benchmark_without_eval_only_proof(c: &mut Criterion) {
    let circuit = black_box(Circuit::random(8));
    let input = black_box(
        (0u64..256)
            .into_iter()
            .map(|x| Fr::from(x))
            .collect::<Vec<Fr>>(),
    );
    let input_in_poly_form = Multilinear::interpolate(&input);
    let tau_s = gen_random_taus::<Fr>(input_in_poly_form.num_vars);
    let srs: MultiLinearSRS<Bls12_381> = MultilinearKZG::generate_srs(&tau_s);
    let commitment = MultilinearKZG::commit(&srs, &input_in_poly_form);

    let evaluation = circuit.evaluate(&input);

    c.bench_function(
        "Succicnt GKR protocol without evaluation and only Prove",
        |b| {
            b.iter(|| {
                let _ = SuccinctGKRProtocol::prove(&circuit, &evaluation, &commitment, &srs);
            })
        },
    );
}

fn gkr_protocol_benchmark_without_eval_only_verify(c: &mut Criterion) {
    let circuit = black_box(Circuit::random(8));
    let input = black_box(
        (0u64..256)
            .into_iter()
            .map(|x| Fr::from(x))
            .collect::<Vec<Fr>>(),
    );
    let input_in_poly_form = Multilinear::interpolate(&input);
    let tau_s = gen_random_taus::<Fr>(input_in_poly_form.num_vars);
    let srs: MultiLinearSRS<Bls12_381> = MultilinearKZG::generate_srs(&tau_s);
    let commitment = MultilinearKZG::commit(&srs, &input_in_poly_form);

    let evaluation = circuit.evaluate(&input);

    let proof = SuccinctGKRProtocol::prove(&circuit, &evaluation, &commitment, &srs);

    c.bench_function(
        "Succicnt GKR protocol without evaluation and only verify",
        |b| {
            b.iter(|| {
                assert!(SuccinctGKRProtocol::verify(
                    &circuit,
                    &proof,
                    &commitment,
                    &srs
                ));
            })
        },
    );
}

criterion_group!(
    benches,
    gkr_protocol_benchmark,
    gkr_protocol_benchmark_without_eval,
    gkr_protocol_benchmark_without_eval_only_proof,
    gkr_protocol_benchmark_without_eval_only_verify
);

criterion_main!(benches);
