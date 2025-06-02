use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use libra::{
    LinearTimeSumCheck, LinearTimeSumCheckTr,
    utils::{generate_igz, product_combined_fn},
};
use p3_field::{AbstractField, extension::BinomialExtensionField};
use p3_mersenne_31::Mersenne31;
use poly::{Fields, MultilinearExtension, mle::MultilinearPoly, vpoly::VPoly};
use std::rc::Rc;
use sum_check::{SumCheck, interface::SumCheckInterface, primitives::SumCheckProof};
use transcript::Transcript;

type F = Mersenne31;
type E = BinomialExtensionField<Mersenne31, 3>;
type Mle = VPoly<F, E>;

/// Creates polynomial setup for testing sum-check protocols.
/// Returns the v_poly for use in tests.
fn create_test_polynomial_setup() -> VPoly<F, E> {
    let mut f_var_zero_vec = vec![Fields::<F, E>::Base(F::new(0u32)); 32];
    f_var_zero_vec[1] = Fields::<F, E>::Base(F::new(1u32));
    let f_1_trad = MultilinearPoly::new_from_vec(5, f_var_zero_vec);

    let g = Fields::<F, E>::Base(F::new(2u32));

    let f2_x_f2_y_vec = [1u32, 2, 3, 4, 2, 4, 6, 8, 3, 6, 9, 12, 4, 8, 12, 16]
        .iter()
        .map(|&x| Fields::<F, E>::Base(F::new(x)))
        .collect::<Vec<_>>();
    let f2_x_f2_y = MultilinearPoly::new_from_vec(4, f2_x_f2_y_vec);

    let f_1_trad_g = f_1_trad.partial_evaluate(&[g]);

    VPoly::<F, E>::new(
        vec![f_1_trad_g, f2_x_f2_y],
        2,
        4,
        Rc::new(product_combined_fn),
    )
}

fn bench_traditional_sum_check(c: &mut Criterion) {
    let mut group = c.benchmark_group("Traditional SumCheck");

    // Benchmark proving
    group.bench_function("prove", |b| {
        b.iter(|| {
            let v_poly = create_test_polynomial_setup();
            let claimed_sum = v_poly.sum_over_hypercube();
            let mut transcript = Transcript::init();
            black_box(SumCheck::prove(claimed_sum, &v_poly, &mut transcript).unwrap());
        });
    });

    // Benchmark verification
    group.bench_function("verify", |b| {
        // Setup once outside the benchmark loop
        let v_poly = create_test_polynomial_setup();
        let claimed_sum = v_poly.sum_over_hypercube();
        let mut transcript = Transcript::init();
        let proof = SumCheck::prove(claimed_sum, &v_poly, &mut transcript).unwrap();

        b.iter(|| {
            let mut transcript = Transcript::init();
            black_box(SumCheck::verify(&v_poly, &proof, &mut transcript).unwrap());
        });
    });

    group.finish();
}

fn bench_linear_time_sum_check(c: &mut Criterion) {
    let mut group = c.benchmark_group("Linear Time SumCheck");

    // Benchmark proving
    group.bench_function("prove", |b| {
        b.iter(|| {
            let f_1_sparse = vec![(0, 0, 1)];

            let f2_x_f2_y_vec = [1u32, 2, 3, 4]
                .iter()
                .map(|&x| F::new(x))
                .collect::<Vec<_>>();

            let g = E::from_wrapped_u32(2u32);
            let ig_z = generate_igz::<F, E>(&[g]);

            let mut transcript = Transcript::init();
            black_box(
                LinearTimeSumCheck::<F, E>::sum_check(
                    &f_1_sparse,
                    &f2_x_f2_y_vec,
                    &f2_x_f2_y_vec,
                    &ig_z,
                    &mut transcript,
                )
                .unwrap(),
            );
        });
    });

    // Benchmark verification (similar to the test)
    group.bench_function("verify", |b| {
        // Setup once outside the benchmark loop
        let f_1_sparse = vec![(0, 0, 1)];
        let f2_x_f2_y_vec = [1u32, 2, 3, 4]
            .iter()
            .map(|&x| F::new(x))
            .collect::<Vec<_>>();
        let g = E::from_wrapped_u32(2u32);
        let ig_z = generate_igz::<F, E>(&[g]);

        let mut transcript = Transcript::init();
        let (round_polys, _) = LinearTimeSumCheck::<F, E>::sum_check(
            &f_1_sparse,
            &f2_x_f2_y_vec,
            &f2_x_f2_y_vec,
            &ig_z,
            &mut transcript,
        )
        .unwrap();

        let claimed_sum = Fields::Base(F::new(2147483645));
        let v_proof = SumCheckProof::new(claimed_sum, round_polys);

        b.iter(|| {
            let mut transcript = Transcript::init();
            black_box(SumCheck::<F, E, Mle>::verify_partial(
                &v_proof,
                &mut transcript,
            ));
        });
    });

    group.finish();
}

// Compare performance with varying input sizes
fn bench_sum_check_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("SumCheck Scaling");

    // Define sizes to test (powers of 2)
    let sizes = [2, 4, 8, 16];

    for size in sizes.iter() {
        let size = *size;
        group.throughput(Throughput::Elements(size as u64));

        // Traditional sumcheck with varying size
        group.bench_with_input(BenchmarkId::new("Traditional", size), &size, |b, &size| {
            b.iter(|| {
                // Create a polynomial with the given size
                let mut f_var_zero_vec = vec![Fields::<F, E>::Base(F::new(0u32)); 1 << size];
                f_var_zero_vec[1] = Fields::<F, E>::Base(F::new(1u32));
                let f_1_trad = MultilinearPoly::new_from_vec(size, f_var_zero_vec);

                let g = Fields::<F, E>::Base(F::new(2u32));

                // Simplified f2 for scaling tests
                let f2_vec = (0..1 << (size - 1))
                    .map(|x| Fields::<F, E>::Base(F::new(x as u32)))
                    .collect::<Vec<_>>();
                let f2 = MultilinearPoly::new_from_vec(size - 1, f2_vec);

                let f_1_trad_g = f_1_trad.partial_evaluate(&[g]);

                let v_poly = VPoly::<F, E>::new(
                    vec![f_1_trad_g, f2],
                    2,
                    size - 1,
                    Rc::new(product_combined_fn),
                );

                let claimed_sum = v_poly.sum_over_hypercube();
                let mut transcript = Transcript::init();
                black_box(SumCheck::prove(claimed_sum, &v_poly, &mut transcript).unwrap());
            });
        });

        // Linear-time sumcheck with varying size
        group.bench_with_input(BenchmarkId::new("LinearTime", size), &size, |b, &size| {
            b.iter(|| {
                let f_1_sparse = vec![(0, 0, 1)];

                // Scaled f2 vector
                let f2_vec = (0..1 << (size - 1))
                    .map(|x| F::new(x as u32))
                    .collect::<Vec<_>>();

                let g = E::from_wrapped_u32(2u32);
                let ig_z = generate_igz::<F, E>(&[g]);

                let mut transcript = Transcript::init();
                black_box(
                    LinearTimeSumCheck::<F, E>::sum_check(
                        &f_1_sparse,
                        &f2_vec,
                        &f2_vec,
                        &ig_z,
                        &mut transcript,
                    )
                    .unwrap(),
                );
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_traditional_sum_check,
    bench_linear_time_sum_check,
    bench_sum_check_scaling
);
criterion_main!(benches);
