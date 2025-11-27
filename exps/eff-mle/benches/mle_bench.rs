use ark_ff::Field;
use ark_test_curves::bls12_381::Fr;
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use eff_mle::direct::mle_eval_direct;
use eff_mle::ron::mle_eval_ron_optimized;

fn create_rand_vec<F: Field>(size: usize) -> Vec<F> {
    let rand_engine = &mut ark_std::test_rng();
    (0..size).map(|_| F::rand(rand_engine)).collect()
}

fn bench_mle_evaluations(c: &mut Criterion) {
    let num_vars_list = [5, 10, 17, 20];

    let mut group = c.benchmark_group("MLE Evaluation");

    for &num_vars in &num_vars_list {
        let evals: Vec<Fr> = create_rand_vec(1 << num_vars);
        let points: Vec<Fr> = create_rand_vec(num_vars);

        group.bench_with_input(
            BenchmarkId::new("direct", num_vars),
            &(&evals, &points),
            |b, (evals, points)| b.iter(|| mle_eval_direct(black_box(points), black_box(evals))),
        );

        group.bench_with_input(
            BenchmarkId::new("ron_optimized", num_vars),
            &(&evals, &points),
            |b, (evals, points)| {
                b.iter(|| mle_eval_ron_optimized(black_box(evals), black_box(points)))
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_mle_evaluations);
criterion_main!(benches);
